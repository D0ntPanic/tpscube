#pragma once

#include <memory>
#include <string>
#include <functional>
#include <time.h>
#include "cubecommon.h"
#include "leveldb/db.h"

enum SolveType
{
	SOLVE_3X3X3 = 0,
	SOLVE_3X3X3_OH = 1,
	SOLVE_3X3X3_BF = 2,
	SOLVE_2X2X2 = 3,
	SOLVE_4X4X4 = 4,
	SOLVE_4X4X4_BF = 5,
	SOLVE_5X5X5 = 6,
	SOLVE_5X5X5_BF = 7
};

struct Update
{
	std::string id;
	time_t date;
	std::string sync;
};

struct Solve
{
	std::string id;
	CubeMoveSequence scramble;
	time_t created;
	Update update;
	bool ok;
	uint32_t time, penalty;
	bool dirty;
};

struct Session
{
	SolveType type;
	std::string id, name;
	Update update;
	std::vector<Solve> solves;
	bool dirty;

	int bestSolve();
	int bestAvgOf(size_t count);
	static int avgOf(const std::vector<int>& times);
	int avgOfLast(size_t count, bool ignoreDNF = false);
	int sessionAvg();
};

class IdGenerator
{
public:
	virtual std::string GenerateId() = 0;
};

struct History
{
	std::vector<std::shared_ptr<Session>> sessions;
	bool sessionListDirty = false;
	std::shared_ptr<Session> activeSession;
	leveldb::DB* database = nullptr;
	IdGenerator* idGenerator = nullptr;

	static History instance;

	leveldb::Status OpenDatabase(const std::string& path,
		const std::function<bool(size_t, size_t)>& progressFn);
	leveldb::Status OpenDatabase(const std::string& path);
	void CloseDatabase();
	bool IsDatabaseOpen();

	void RecordSolve(SolveType type, const Solve& solve);
	void ResetSession();
	void DeleteSession(std::shared_ptr<Session> session);
	void SplitSessionAtSolve(const std::shared_ptr<Session>& session, size_t solveIdx);
	void MergeSessions(const std::shared_ptr<Session>& firstSession,
		const std::shared_ptr<Session>& secondSession, const std::string& name);

	std::string SerializeSolve(const Solve& solve);
	std::string SerializeSolveList(const std::shared_ptr<Session>& session);
	std::string SerializeSession(const std::shared_ptr<Session>& session);
	std::string SerializeSessionList();

	leveldb::Status DeserializeSolve(const std::string& data, Solve& solve);
	leveldb::Status DeserializeSolveList(const std::string& data, std::vector<std::string>& list);
	leveldb::Status DeserializeSession(const std::string& data, const std::shared_ptr<Session>& session);
	leveldb::Status DeserializeSessionList(const std::string& data, std::vector<std::string>& list);

	void UpdateDatabaseForSession(const std::shared_ptr<Session>& session);
	void UpdateDatabaseForSessions(const std::vector<std::shared_ptr<Session>>& sessions);
};
