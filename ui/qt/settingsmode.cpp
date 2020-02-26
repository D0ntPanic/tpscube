#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QFileDialog>
#include <QtWidgets/QMessageBox>
#include <QtWidgets/QProgressDialog>
#include <QtCore/QJsonDocument>
#include <QtCore/QJsonObject>
#include <QtCore/QJsonArray>
#include <time.h>
#include <set>
#include <algorithm>
#include "settingsmode.h"

using namespace std;


struct SolveSource
{
	shared_ptr<Session> session;
	size_t solveIndex;
};


SettingsMode::SettingsMode(QWidget* parent): QScrollArea(parent)
{
	setWidgetResizable(true);
	setFrameStyle(QFrame::NoFrame);

	QWidget* container = new QWidget();
	container->setBackgroundRole(QPalette::Base);
	container->setAutoFillBackground(true);

	QVBoxLayout* layout = new QVBoxLayout();

	Heading* importExportHeading = new Heading("Import / export solve history");
	layout->addWidget(importExportHeading);

	ClickableLabel* exportButton = new ClickableLabel("Export solve history...",
		Theme::content, Theme::blue, [this]() { exportSolves(); });
	exportButton->setCursor(Qt::PointingHandCursor);
	layout->addWidget(exportButton);
	ClickableLabel* importButton = new ClickableLabel("Import and merge solve history...",
		Theme::content, Theme::blue, [this]() { importSolves(); });
	importButton->setCursor(Qt::PointingHandCursor);
	layout->addWidget(importButton);

	layout->addStretch(1);
	container->setLayout(layout);
	setWidget(container);
}


void SettingsMode::exportSolves()
{
	QString path = QFileDialog::getSaveFileName(this, "Export solve history", QString(),
		"JSON files (*.json)");
	if (path.isNull())
		return;

	int count = 0;
	for (auto& i : History::instance.sessions)
		count += (int)i->solves.size();
	QProgressDialog progress("Exporting solves...", "Abort", 0, count, this);
	progress.setWindowModality(Qt::WindowModal);

	int current = 0;
	QJsonObject solveData;
	QJsonArray sessions;
	for (auto& i : History::instance.sessions)
	{
		QJsonObject session;
		session["type"] = QString::fromStdString(Session::GetSolveTypeName(i->type));
		session["name"] = QString::fromStdString(i->name);
		session["id"] = QString::fromStdString(i->id);
		session["updated"] = (qint64)i->update.date;

		QJsonArray solves;
		for (auto& j : i->solves)
		{
			QJsonObject solve;
			solve["id"] = QString::fromStdString(j.id);
			solve["timestamp"] = (qint64)j.created;
			solve["updated"] = (qint64)j.update.date;
			solve["scramble"] = QString::fromStdString(j.scramble.ToString());
			solve["ok"] = j.ok;
			solve["time"] = (int)j.time;
			solve["penalty"] = (int)j.penalty;
			if (j.solveDevice.size() != 0)
				solve["device"] = QString::fromStdString(j.solveDevice);
			if (j.solveMoves.moves.size() != 0)
				solve["solve"] = QString::fromStdString(j.solveMoves.ToString());
			if (j.crossTime != 0)
				solve["cross"] = (int)j.crossTime;
			if (j.f2lPairTimes[0] != 0)
				solve["pair1"] = (int)j.f2lPairTimes[0];
			if (j.f2lPairTimes[1] != 0)
				solve["pair2"] = (int)j.f2lPairTimes[1];
			if (j.f2lPairTimes[2] != 0)
				solve["pair3"] = (int)j.f2lPairTimes[2];
			if (j.f2lPairTimes[3] != 0)
				solve["f2l"] = (int)j.f2lPairTimes[3];
			if (j.ollCrossTime != 0)
				solve["ollcross"] = (int)j.ollCrossTime;
			if (j.ollFinishTime != 0)
				solve["oll"] = (int)j.ollFinishTime;
			if (j.pllCornerTime != 0)
				solve["pllcorner"] = (int)j.pllCornerTime;
			solves.append(solve);

			progress.setValue(++current);
			if (progress.wasCanceled())
				return;
		}
		session["solves"] = solves;
		sessions.append(session);
	}
	solveData["sessions"] = sessions;

	QJsonDocument doc(solveData);
	QByteArray result = doc.toJson();

	QFile f(path);
	if (!f.open(QIODevice::WriteOnly))
	{
		QMessageBox::critical(this, "Error", "Unable to export solve file:\n" + f.errorString());
		return;
	}
	if (f.write(result) != result.size())
	{
		QMessageBox::critical(this, "Error", "Unable to export solve file:\n" + f.errorString());
		return;
	}
	f.close();
}


bool SettingsMode::importNativeJson(const QJsonObject& solveData, vector<shared_ptr<Session>>& result)
{
	QJsonArray sessions = solveData["sessions"].toArray();
	int totalImportedSolves = 0;
	for (auto sessionValue : sessions)
	{
		QJsonObject session = sessionValue.toObject();
		shared_ptr<Session> importedSession = make_shared<Session>();
		importedSession->name = session["name"].toString().toStdString();
		importedSession->id = session["id"].toString().toStdString();
		importedSession->update.date = (qint64)session["updated"].toDouble();
		importedSession->update.id = History::instance.idGenerator->GenerateId();
		importedSession->dirty = true;
		if (importedSession->id.size() == 0)
		{
			QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid session ID");
			return false;
		}
		if (!Session::GetSolveTypeByName(session["type"].toString().toStdString(), importedSession->type))
		{
			QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid session type '" +
				session["type"].toString() + "'");
			return false;
		}

		QJsonArray solves = session["solves"].toArray();
		for (auto solveValue : solves)
		{
			QJsonObject solve = solveValue.toObject();
			Solve importedSolve;
			importedSolve.id = solve["id"].toString().toStdString();
			if (importedSolve.id.size() == 0)
			{
				QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid solve ID");
				return false;
			}
			importedSolve.created = (qint64)solve["timestamp"].toDouble();
			if (!CubeMoveSequence::FromString(solve["scramble"].toString().toStdString(),
				importedSolve.scramble))
			{
				QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid scramble");
				return false;
			}
			importedSolve.ok = solve["ok"].toBool();
			importedSolve.time = solve["time"].toInt();
			importedSolve.penalty = solve["penalty"].toInt();
			importedSolve.solveDevice = solve["device"].toString().toStdString();
			TimedCubeMoveSequence::FromString(solve["solve"].toString().toStdString(),
				importedSolve.solveMoves);
			importedSolve.crossTime = solve["cross"].toInt();
			importedSolve.f2lPairTimes[0] = solve["pair1"].toInt();
			importedSolve.f2lPairTimes[1] = solve["pair2"].toInt();
			importedSolve.f2lPairTimes[2] = solve["pair3"].toInt();
			importedSolve.f2lPairTimes[3] = solve["f2l"].toInt();
			importedSolve.ollCrossTime = solve["ollcross"].toInt();
			importedSolve.ollFinishTime = solve["oll"].toInt();
			importedSolve.pllCornerTime = solve["pllcorner"].toInt();
			importedSolve.update.date = (qint64)solve["updated"].toDouble();
			importedSolve.update.id = History::instance.idGenerator->GenerateId();
			importedSolve.dirty = true;
			importedSession->solves.push_back(importedSolve);
			totalImportedSolves++;
		}

		result.push_back(importedSession);
	}
	return true;
}


bool SettingsMode::importCstimerJson(const QJsonObject& solveData, vector<shared_ptr<Session>>& result)
{
	QJsonParseError error;
	QJsonDocument sessionData = QJsonDocument::fromJson(
		solveData["properties"].toObject()["sessionData"].toString().toUtf8());
	if (sessionData.isNull())
	{
		QMessageBox::critical(this, "Error", "Unable to parse solve file:\n" + error.errorString());
		return false;
	}

	for (auto i : solveData.keys())
	{
		if (!i.startsWith("session"))
			continue;
		QString sessionIndex = i.mid(7);
		QString solveType = sessionData[sessionIndex].toObject()["opt"].toObject()["scrType"].toString();
		SolveType type;
		if ((solveType.isNull()) || (solveType == ""))
			type = SOLVE_3X3X3;
		else if (solveType == "333oh")
			type = SOLVE_3X3X3_OH;
		else if (solveType == "333bld")
			type = SOLVE_3X3X3_BF;
		else if (solveType == "222")
			type = SOLVE_2X2X2;
		else if (solveType == "444")
			type = SOLVE_4X4X4;
		else if (solveType == "444bld")
			type = SOLVE_4X4X4_BF;
		else if (solveType == "555")
			type = SOLVE_5X5X5;
		else if (solveType == "555bld")
			type = SOLVE_5X5X5_BF;
		else
			continue;

		shared_ptr<Session> importedSession = make_shared<Session>();
		importedSession->type = type;
		importedSession->id = ("cstimer:" + sessionIndex).toStdString();
		importedSession->update.date = time(NULL);
		importedSession->update.id = History::instance.idGenerator->GenerateId();
		importedSession->dirty = true;

		QJsonArray solveArray = solveData[i].toArray();
		for (auto solve : solveArray)
		{
			Solve importedSolve;
			importedSolve.penalty = solve.toArray()[0].toArray()[0].toInt();
			importedSolve.time = solve.toArray()[0].toArray()[1].toInt();
			if (!CubeMoveSequence::FromString(solve.toArray()[1].toString().toStdString(),
				importedSolve.scramble))
			{
				QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid scramble");
				return false;
			}
			importedSolve.created = (qint64)solve.toArray()[3].toDouble();
			importedSolve.id = QString("cstimer:%1").arg((qulonglong)importedSolve.created).toStdString();
			if (!TimedCubeMoveSequence::FromString(solve.toArray()[4].toArray()[0].toString().toStdString(),
				importedSolve.solveMoves))
			{
				QMessageBox::critical(this, "Error", "Unable to parse solve file:\nInvalid solve sequence");
				return false;
			}
			importedSolve.ok = (importedSolve.penalty != (uint32_t)-1);
			if (!importedSolve.ok)
				importedSolve.penalty = 0;
			if (importedSolve.solveMoves.moves.size() != 0)
				importedSolve.GenerateSplitTimesFromMoves();
			importedSolve.update.date = time(NULL);
			importedSolve.update.id = History::instance.idGenerator->GenerateId();
			importedSolve.dirty = true;
			importedSession->solves.push_back(importedSolve);
		}

		result.push_back(importedSession);
	}
	return true;
}


void SettingsMode::importSolves()
{
	QString path = QFileDialog::getOpenFileName(this, "Import solve history", QString(),
		"JSON files (*.json *.txt)");
	if (path.isNull())
		return;

	QFile f(path);
	if (!f.open(QIODevice::ReadOnly))
	{
		QMessageBox::critical(this, "Error", "Unable to import solve file:\n" + f.errorString());
		return;
	}
	QByteArray rawData = f.readAll();
	f.close();

	QJsonParseError error;
	QJsonDocument doc = QJsonDocument::fromJson(rawData, &error);
	if (doc.isNull())
	{
		QMessageBox::critical(this, "Error", "Unable to parse solve file:\n" + error.errorString());
		return;
	}

	// Parse solve data in file
	QJsonObject solveData = doc.object();
	vector<shared_ptr<Session>> importedSessions;

	if (solveData.contains("properties") && solveData["properties"].toObject().contains("sessionData"))
	{
		if (!importCstimerJson(solveData, importedSessions))
			return;
	}
	else
	{
		if (!importNativeJson(solveData, importedSessions))
			return;
	}

	int totalImportedSolves = 0;
	for (auto& i : importedSessions)
		totalImportedSolves += (int)i->solves.size();

	int newSolves = 0;
	int updatedSolves = 0;
	int newSessions = 0;

	// Organize existing solves by date and solve time
	map<string, SolveSource> existingSolveIds;
	map<string, shared_ptr<Session>> existingSessions;
	map<time_t, map<int, SolveSource>> existingSolves;
	for (auto& i : History::instance.sessions)
	{
		existingSessions[i->id] = i;
		for (size_t j = 0; j < i->solves.size(); j++)
		{
			const Solve& solve = i->solves[j];
			existingSolveIds[solve.id] = SolveSource { i, j };
			existingSolves[solve.created][solve.ok ? (solve.time - solve.penalty) : -1] =
				SolveSource { i, j };
		}
	}

	// Find imported solves that already exist
	set<shared_ptr<Session>> updatedSessions;
	for (auto& i : importedSessions)
	{
		for (size_t j = 0; j < i->solves.size(); j++)
		{
			const Solve& importedSolve = i->solves[j];

			auto idIter = existingSolveIds.find(importedSolve.id);
			if (idIter != existingSolveIds.end())
			{
				// Solve exists by ID, see if it might be updated
				Solve& existingSolve = idIter->second.session->solves[idIter->second.solveIndex];
				if ((importedSolve.update.date > existingSolve.update.date) &&
					(importedSolve != existingSolve))
				{
					// Solve has updates, use the new information
					existingSolve = importedSolve;
					existingSolve.dirty = true;
					updatedSessions.insert(idIter->second.session);
					updatedSolves++;
				}
			}
			else
			{
				// Check for existing solve by creation date and solve time
				auto createdIter = existingSolves.find(importedSolve.created);
				if (createdIter == existingSolves.end())
					continue;
				auto timeIter = createdIter->second.find(importedSolve.ok ?
					(importedSolve.time - importedSolve.penalty) : -1);
				if (timeIter == createdIter->second.end())
					continue;

				// Solve already exists, see if it might be updated
				Solve& existingSolve = timeIter->second.session->solves[timeIter->second.solveIndex];
				if ((importedSolve.update.date > existingSolve.update.date) &&
					(importedSolve != existingSolve))
				{
					// Solve has updates, use the new information
					existingSolve = importedSolve;
					existingSolve.dirty = true;
					updatedSessions.insert(timeIter->second.session);
					updatedSolves++;
				}
			}

			// Solve existed already, erase solve from solves to import
			i->solves.erase(i->solves.begin() + j);
			j--;
		}
	}

	// Create new solves (and new sessions if they belong to new sessions)
	for (auto& i : importedSessions)
	{
		if (i->solves.size() == 0)
			continue;

		auto existingIter = existingSessions.find(i->id);
		if (existingIter == existingSessions.end())
		{
			// Session did not exist, this is a new session
			History::instance.sessions.push_back(i);
			History::instance.sessionListDirty = true;
			History::instance.UpdateDatabaseForSession(i);
			newSessions++;
			newSolves += (int)i->solves.size();
			continue;
		}

		// Session already exists, add new solves to it
		existingIter->second->solves.insert(existingIter->second->solves.end(),
			i->solves.begin(), i->solves.end());
		sort(existingIter->second->solves.begin(), existingIter->second->solves.end(),
			[](const Solve& a, const Solve& b) {
				if (a.created < b.created)
					return true;
				if (a.created > b.created)
					return false;
				return a.update.date < b.update.date;
			});
		updatedSessions.insert(existingIter->second);
		newSolves += (int)i->solves.size();
	}

	// Check for updated session names
	for (auto& i : importedSessions)
	{
		auto existingIter = existingSessions.find(i->id);
		if (existingIter == existingSessions.end())
			continue;

		if ((i->update.date > existingIter->second->update.date) &&
			(i->name != existingIter->second->name))
		{
			// Name has been updated
			existingIter->second->name = i->name;
			updatedSessions.insert(existingIter->second);
		}
	}

	// Save updated sessions to database
	for (auto& i : updatedSessions)
	{
		i->update.date = time(NULL);
		i->update.id = History::instance.idGenerator->GenerateId();
		i->dirty = true;
		History::instance.UpdateDatabaseForSession(i);
	}

	QString message = QString::asprintf("File contained %d solve(s) in %d sessions(s).",
		totalImportedSolves, (int)importedSessions.size());

	if (newSessions != 0)
		message += QString::asprintf("\nImported %d new session(s).", (int)newSessions);
	if (updatedSessions.size() != 0)
		message += QString::asprintf("\nUpdated %d session(s).", (int)updatedSessions.size());
	if (newSolves != 0)
		message += QString::asprintf("\nImported %d new solves(s).", (int)newSolves);
	if (updatedSolves != 0)
		message += QString::asprintf("\nUpdated %d solves(s).", (int)updatedSolves);

	if (!newSolves && !updatedSolves && !newSessions && (updatedSessions.size() == 0))
		message += "\nNo new information was imported.";

	QMessageBox::information(this, "Import Complete", message);
}
