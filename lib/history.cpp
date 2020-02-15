#include <unistd.h>
#include "history.h"
#include "leveldb/write_batch.h"
#include "database_generated.h"

using namespace std;


History History::instance;


int Session::avgOf(const vector<int>& times)
{
	vector<int> sorted = times;
	sort(sorted.begin(), sorted.end(), [](int a, int b) {
		// DNF must be considered largest possible time
		if ((a == -1) && (b == -1))
			return false;
		if (a == -1)
			return false;
		if (b == -1)
			return true;
		return a < b;
	});
	if (sorted.size() <= 2)
		return -1;
	sorted.erase(sorted.begin());
	sorted.erase(sorted.end() - 1);
	int sum = 0;
	for (auto i : sorted)
	{
		if (i == -1)
			return -1;
		sum += i;
	}
	return (int)(((float)sum / (float)sorted.size()) + 0.5f);
}


int Session::avgOfLast(size_t count, bool ignoreDNF)
{
	if (count > solves.size())
		return -1;
	size_t start = solves.size() - count;
	vector<int> times;
	for (size_t i = start; i < solves.size(); i++)
	{
		const Solve& solve = solves[i];
		if (ignoreDNF && !solve.ok)
			continue;
		if (solve.ok)
			times.push_back(solve.time);
		else
			times.push_back(-1);
	}
	return avgOf(times);
}


int Session::bestSolve(Solve* solve)
{
	if (solve)
		solve->ok = false;
	int best = -1;
	for (auto& i : solves)
	{
		if (!i.ok)
			continue;
		if ((best == -1) || ((int)i.time < best))
		{
			best = (int)i.time;
			if (solve)
				*solve = i;
		}
	}
	return best;
}


int Session::bestAvgOf(size_t count, int* start)
{
	if (solves.size() < count)
		return -1;
	int best = -1;
	if (start)
		*start = -1;
	for (size_t i = 0; i <= (solves.size() - count); i++)
	{
		vector<int> times;
		for (size_t j = 0; j < count; j++)
		{
			const Solve& solve = solves[i + j];
			if (solve.ok)
				times.push_back(solve.time);
			else
				times.push_back(-1);
		}
		int avg = avgOf(times);
		if (avg == -1)
			continue;
		if ((best == -1) || (avg < best))
		{
			best = avg;
			if (start)
				*start = (int)i;
		}
	}
	return best;
}


int Session::sessionAvg()
{
	return avgOfLast(solves.size(), true);
}


leveldb::Status History::OpenDatabase(const std::string& path,
	const std::function<bool(size_t, size_t)>& progressFn)
{
	CloseDatabase();

	if (!idGenerator)
		return leveldb::Status::InvalidArgument("ID generator not set");

	// Open database
	leveldb::Options options;
	options.create_if_missing = true;
	leveldb::Status status = leveldb::DB::Open(options, path, &database);
	if (!status.ok())
	{
		CloseDatabase();
		return status;
	}

	// Read session list
	string sessionListData;
	status = database->Get(leveldb::ReadOptions(), "sessions", &sessionListData);
	if (status.IsNotFound()) // Session list missing is fresh database
		return leveldb::Status::OK();
	if (!status.ok())
		return status;
	vector<string> sessionList;
	status = DeserializeSessionList(sessionListData, sessionList);
	if (!status.ok())
		return status;

	// Iterate through sessions and read them
	leveldb::Status finalStatus = leveldb::Status::OK();
	for (size_t sessionIndex = 0; sessionIndex < sessionList.size(); sessionIndex++)
	{
		if (progressFn(sessionIndex, sessionList.size()))
			return finalStatus;

		// Read session information
		string sessionData;
		status = database->Get(leveldb::ReadOptions(), "session:" + sessionList[sessionIndex], &sessionData);
		if (!status.ok())
		{
			finalStatus = status;
			continue;
		}
		shared_ptr<Session> session = make_shared<Session>();
		session->id = sessionList[sessionIndex];
		status = DeserializeSession(sessionData, session);
		if (!status.ok())
		{
			finalStatus = status;
			continue;
		}

		// Read session solve list
		string solveListData;
		status = database->Get(leveldb::ReadOptions(), "session_solves:" + session->id, &solveListData);
		if (!status.ok())
		{
			finalStatus = status;
			continue;
		}
		vector<string> solveList;
		status = DeserializeSolveList(solveListData, solveList);
		if (!status.ok())
		{
			finalStatus = status;
			continue;
		}

		// Iterate through solves and read them
		for (auto& solveId : solveList)
		{
			if (progressFn(sessionIndex, sessionList.size()))
				return finalStatus;

			string solveData;
			status = database->Get(leveldb::ReadOptions(), "solve:" + solveId, &solveData);
			if (!status.ok())
			{
				finalStatus = status;
				continue;
			}
			Solve solve;
			solve.id = solveId;
			status = DeserializeSolve(solveData, solve);
			if (!status.ok())
			{
				finalStatus = status;
				continue;
			}
			session->solves.push_back(solve);
		}

		if (session->solves.size() > 0)
			sessions.push_back(session);
	}

	string activeSessionId;
	status = database->Get(leveldb::ReadOptions(), "active_session", &activeSessionId);
	activeSession.reset();
	if (status.ok())
	{
		for (auto& i : sessions)
		{
			if (i->id == activeSessionId)
			{
				activeSession = i;
				break;
			}
		}
	}

	progressFn(sessionList.size(), sessionList.size());
	return finalStatus;
}


leveldb::Status History::OpenDatabase(const std::string& path)
{
	return OpenDatabase(path, [](size_t, size_t) { return false; });
}


void History::CloseDatabase()
{
	if (database)
	{
		delete database;
		database = nullptr;
	}
}


bool History::IsDatabaseOpen()
{
	return database != nullptr;
}


void History::RecordSolve(SolveType type, const Solve& solve)
{
	if (!activeSession || (activeSession->type != type))
	{
		// No active session or wrong session type, create a new session
		activeSession = make_shared<Session>();
		sessions.push_back(activeSession);
		activeSession->id = idGenerator->GenerateId();
		activeSession->type = type;
		sessionListDirty = true;

		if (database)
			database->Put(leveldb::WriteOptions(), "active_session", activeSession->id);
	}

	activeSession->solves.push_back(solve);
	activeSession->update.id = idGenerator->GenerateId();
	activeSession->update.date = time(NULL);
	activeSession->dirty = true;

	UpdateDatabaseForSession(activeSession);
}


void History::ResetSession()
{
	activeSession.reset();
	if (database)
		database->Delete(leveldb::WriteOptions(), "active_session");
}


void History::DeleteSession(shared_ptr<Session> session)
{
	for (auto i = sessions.begin(); i != sessions.end(); ++i)
	{
		if (*i == session)
		{
			sessions.erase(i);
			sessionListDirty = true;
			break;
		}
	}

	if (activeSession == session)
	{
		activeSession.reset();
		if (database)
			database->Delete(leveldb::WriteOptions(), "active_session");
	}

	if (database)
	{
		leveldb::WriteBatch batch;
		batch.Delete("session:" + session->id);

		set<string> solvesToDelete;
		for (auto& i : session->solves)
			solvesToDelete.insert(i.id);
		for (auto& i : sessions)
		{
			for (auto& j : i->solves)
			{
				if (solvesToDelete.count(j.id))
					solvesToDelete.erase(j.id);
			}
		}
		for (auto& i : solvesToDelete)
			batch.Delete("solve:" + i);

		if (sessionListDirty)
		{
			batch.Put("sessions", SerializeSessionList());
			sessionListDirty = false;
		}

		database->Write(leveldb::WriteOptions(), &batch);
	}
}


void History::SplitSessionAtSolve(const shared_ptr<Session>& session, size_t solveIdx)
{
	if ((solveIdx == 0) || (solveIdx >= session->solves.size()))
		return;

	for (auto i = sessions.begin(); i != sessions.end(); ++i)
	{
		if (*i == session)
		{
			++i;

			shared_ptr<Session> splitSession = make_shared<Session>();
			splitSession->type = session->type;
			splitSession->id = idGenerator->GenerateId();
			splitSession->name = session->name;
			splitSession->update.id = idGenerator->GenerateId();
			splitSession->update.date = time(NULL);
			splitSession->solves.insert(splitSession->solves.end(),
				session->solves.begin() + solveIdx, session->solves.end());
			splitSession->dirty = true;
			session->solves.erase(session->solves.begin() + solveIdx, session->solves.end());
			session->update.id = idGenerator->GenerateId();
			session->update.date = time(NULL);
			session->dirty = true;

			sessions.insert(i, splitSession);
			sessionListDirty = true;

			UpdateDatabaseForSessions(vector<shared_ptr<Session>> { session, splitSession });

			if (session == activeSession)
			{
				activeSession = splitSession;
				if (database)
					database->Put(leveldb::WriteOptions(), "active_session", activeSession->id);
			}
			return;
		}
	}
}


void History::MergeSessions(const std::shared_ptr<Session>& firstSession,
	const std::shared_ptr<Session>& secondSession, const std::string& name)
{
	if (firstSession->type != secondSession->type)
		return;

	secondSession->solves.insert(secondSession->solves.begin(),
		firstSession->solves.begin(), firstSession->solves.end());
	secondSession->name = name;
	secondSession->update.id = idGenerator->GenerateId();
	secondSession->update.date = time(NULL);
	secondSession->dirty = true;
	UpdateDatabaseForSession(secondSession);
	DeleteSession(firstSession);
}


string History::SerializeSolve(const Solve& solve)
{
	flatbuffers::FlatBufferBuilder builder;
	vector<uint8_t> scrambleList;
	for (auto i : solve.scramble.moves)
		scrambleList.push_back(i);
	auto scramble = builder.CreateVector(scrambleList);

	auto updateId = builder.CreateString(solve.update.id);
	auto updateSync = builder.CreateString(solve.update.sync);
	auto update = database::CreateUpdate(builder, updateId, solve.update.date, updateSync);

	database::CubeSolveBuilder solveBuilder(builder);
	solveBuilder.add_scramble(scramble);
	solveBuilder.add_created(solve.created);
	solveBuilder.add_update(update);
	solveBuilder.add_ok(solve.ok);
	solveBuilder.add_time(solve.time);
	solveBuilder.add_penalty(solve.penalty);
	auto solveData = solveBuilder.Finish();
	auto data = database::CreateData(builder, database::Contents_cube_solve, solveData.Union());
	database::FinishDataBuffer(builder, data);
	return string((const char*)builder.GetBufferPointer(), builder.GetSize());
}


string History::SerializeSolveList(const shared_ptr<Session>& session)
{
	flatbuffers::FlatBufferBuilder builder;
	vector<flatbuffers::Offset<flatbuffers::String>> list;
	for (auto& i: session->solves)
		list.push_back(builder.CreateString(i.id));
	auto listOffset = builder.CreateVector(list);
	auto listData = database::CreateSolveList(builder, listOffset);
	auto data = database::CreateData(builder, database::Contents_solve_list, listData.Union());
	database::FinishDataBuffer(builder, data);
	return string((const char*)builder.GetBufferPointer(), builder.GetSize());
}


string History::SerializeSession(const shared_ptr<Session>& session)
{
	flatbuffers::FlatBufferBuilder builder;
	auto name = builder.CreateString(session->name);

	auto updateId = builder.CreateString(session->update.id);
	auto updateSync = builder.CreateString(session->update.sync);
	auto update = database::CreateUpdate(builder, updateId, session->update.date, updateSync);

	database::SessionBuilder sessionBuilder(builder);
	sessionBuilder.add_type((database::SolveType)session->type);
	sessionBuilder.add_name(name);
	sessionBuilder.add_update(update);
	auto sessionData = sessionBuilder.Finish();
	auto data = database::CreateData(builder, database::Contents_session, sessionData.Union());
	database::FinishDataBuffer(builder, data);
	return string((const char*)builder.GetBufferPointer(), builder.GetSize());
	return "";
}


string History::SerializeSessionList()
{
	flatbuffers::FlatBufferBuilder builder;
	vector<flatbuffers::Offset<flatbuffers::String>> list;
	for (auto& i: sessions)
		list.push_back(builder.CreateString(i->id));
	auto listOffset = builder.CreateVector(list);
	auto listData = database::CreateSessionList(builder, listOffset);
	auto data = database::CreateData(builder, database::Contents_session_list, listData.Union());
	database::FinishDataBuffer(builder, data);
	return string((const char*)builder.GetBufferPointer(), builder.GetSize());
}


leveldb::Status History::DeserializeSolve(const string& data, Solve& solve)
{
	auto verifier = flatbuffers::Verifier((const uint8_t*)data.c_str(), data.size());
	if (!database::VerifyDataBuffer(verifier))
		return leveldb::Status::Corruption("Solve has invalid format");

	auto dataObj = database::GetData(data.c_str());
	if (dataObj->contents_type() != database::Contents_cube_solve)
		return leveldb::Status::Corruption("Solve data does not contain a solve");
	auto solveData = dataObj->contents_as_cube_solve();
	if (!solveData)
		return leveldb::Status::Corruption("Solve data does not contain a solve");

	auto scramble = solveData->scramble();
	if (scramble)
	{
		solve.scramble.moves.clear();
		for (auto i : *scramble)
			solve.scramble.moves.push_back((CubeMove)i);
	}

	solve.created = (time_t)solveData->created();
	solve.ok = solveData->ok();
	solve.time = solveData->time();
	solve.penalty = solveData->penalty();
	solve.dirty = false;

	auto update = solveData->update();
	if (update && update->id())
		solve.update.id = update->id()->str();
	else
		solve.update.id = idGenerator->GenerateId();
	if (update)
		solve.update.date = (time_t)update->time();
	else
		solve.update.date = time(NULL);
	if (update && update->sync())
		solve.update.sync = update->sync()->str();

	return leveldb::Status::OK();
}


leveldb::Status History::DeserializeSolveList(const string& data, vector<string>& list)
{
	auto verifier = flatbuffers::Verifier((const uint8_t*)data.c_str(), data.size());
	if (!database::VerifyDataBuffer(verifier))
		return leveldb::Status::Corruption("Solve list has invalid format");

	auto dataObj = database::GetData(data.c_str());
	if (dataObj->contents_type() != database::Contents_solve_list)
		return leveldb::Status::Corruption("Solve list data does not contain a solve list");
	auto solveListData = dataObj->contents_as_solve_list();
	if (!solveListData)
		return leveldb::Status::Corruption("Solve list data does not contain a solve list");
	auto solveList = solveListData->solves();
	if (!solveList)
		return leveldb::Status::Corruption("Solve list data does not contain a solve list");

	list.clear();
	for (auto i : *solveList)
	{
		if (i)
			list.push_back(i->str());
	}
	return leveldb::Status::OK();
}


leveldb::Status History::DeserializeSession(const string& data, const shared_ptr<Session>& session)
{
	auto verifier = flatbuffers::Verifier((const uint8_t*)data.c_str(), data.size());
	if (!database::VerifyDataBuffer(verifier))
		return leveldb::Status::Corruption("Solve has invalid format");

	auto dataObj = database::GetData(data.c_str());
	if (dataObj->contents_type() != database::Contents_session)
		return leveldb::Status::Corruption("Solve data does not contain a solve");
	auto sessionData = dataObj->contents_as_session();
	if (!sessionData)
		return leveldb::Status::Corruption("Solve data does not contain a solve");

	session->type = (SolveType)sessionData->type();
	if (sessionData->name())
		session->name = sessionData->name()->str();
	session->dirty = false;

	auto update = sessionData->update();
	if (update && update->id())
		session->update.id = update->id()->str();
	else
		session->update.id = idGenerator->GenerateId();
	if (update)
		session->update.date = (time_t)update->time();
	else
		session->update.date = time(NULL);
	if (update && update->sync())
		session->update.sync = update->sync()->str();

	return leveldb::Status::OK();
}


leveldb::Status History::DeserializeSessionList(const string& data, vector<string>& list)
{
	auto verifier = flatbuffers::Verifier((const uint8_t*)data.c_str(), data.size());
	if (!database::VerifyDataBuffer(verifier))
		return leveldb::Status::Corruption("Session list has invalid format");

	auto dataObj = database::GetData(data.c_str());
	if (dataObj->contents_type() != database::Contents_session_list)
		return leveldb::Status::Corruption("Session list data does not contain a solve list");
	auto sessionListData = dataObj->contents_as_session_list();
	if (!sessionListData)
		return leveldb::Status::Corruption("Session list data does not contain a solve list");
	auto sessionList = sessionListData->sessions();
	if (!sessionList)
		return leveldb::Status::Corruption("Session list data does not contain a solve list");

	list.clear();
	for (auto i : *sessionList)
	{
		if (i)
			list.push_back(i->str());
	}
	return leveldb::Status::OK();
}


void History::UpdateDatabaseForSession(const shared_ptr<Session>& session)
{
	UpdateDatabaseForSessions(vector<shared_ptr<Session>> { session });
}


void History::UpdateDatabaseForSessions(const vector<shared_ptr<Session>>& sessions)
{
	if (!database)
		return;

	leveldb::WriteBatch batch;

	for (auto& session : sessions)
	{
		if (session->dirty)
		{
			for (auto& i : session->solves)
			{
				if (!i.dirty)
					continue;
				batch.Put("solve:" + i.id, SerializeSolve(i));
				i.dirty = false;
			}
			batch.Put("session_solves:" + session->id, SerializeSolveList(session));
			batch.Put("session:" + session->id, SerializeSession(session));
			session->dirty = false;
		}
	}

	if (sessionListDirty)
	{
		batch.Put("sessions", SerializeSessionList());
		sessionListDirty = false;
	}

	database->Write(leveldb::WriteOptions(), &batch);
}
