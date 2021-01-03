#pragma once

#include <memory>
#include <string>
#include <functional>
#include <map>
#include <time.h>
#include <leveldb/db.h>
#include "cube3x3.h"

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

enum SolveState
{
	SOLVESTATE_INITIAL = 0,
	SOLVESTATE_CROSS = 1,
	SOLVESTATE_F2L_FIRST_PAIR = 2,
	SOLVESTATE_F2L_SECOND_PAIR = 3,
	SOLVESTATE_F2L_THIRD_PAIR = 4,
	SOLVESTATE_F2L_COMPLETE = 5,
	SOLVESTATE_OLL_CROSS = 6,
	SOLVESTATE_OLL_COMPLETE = 7,
	SOLVESTATE_PLL_CORNERS = 8,
	SOLVESTATE_SOLVED = 9
};

struct DetailedSplit
{
	uint32_t phaseStartTime;
	uint32_t firstMoveTime;
	uint32_t finishTime;
	size_t moveCount;
};

struct DetailedSplitTimes
{
	DetailedSplit cross;
	DetailedSplit f2lPair[4];
	DetailedSplit ollCross;
	DetailedSplit ollFinish;
	DetailedSplit pllCorner;
	DetailedSplit pllFinish;
	size_t moveCount;
	uint32_t idleTime;
	float tps;
	float etps;
};

struct Solve
{
	std::string id;
	CubeMoveSequence scramble;
	time_t created;
	Update update;
	bool ok;
	uint32_t time, penalty;
	std::string solveDevice;
	TimedCubeMoveSequence solveMoves;
	uint32_t crossTime = 0;
	uint32_t f2lPairTimes[4] = {0, 0, 0, 0};
	uint32_t ollCrossTime = 0;
	uint32_t ollFinishTime = 0;
	uint32_t pllCornerTime = 0;
	bool dirty;

	void GenerateSplitTimesFromMoves();
	DetailedSplitTimes GenerateDetailedSplitTimes() const;
	void RecordSplitTimeForSolveState(SolveState state, int timestamp);
	static DetailedSplit* GetSplitForSolveState(SolveState state, DetailedSplitTimes* splits);

	static SolveState TransitionSolveState(const Cube3x3& cube, SolveState currentState);
	static bool WhiteCrossValid(const Cube3x3Faces& faces);
	static int GetF2LPairCount(const Cube3x3Faces& faces);
	static bool IsF2LSolved(const Cube3x3Faces& faces);
	static bool YellowCrossValid(const Cube3x3Faces& faces);
	static bool LastLayerOriented(const Cube3x3Faces& faces);
	static bool LastLayerCornersValid(const Cube3x3Faces& faces);

	bool operator==(const Solve& other) const;
	bool operator!=(const Solve& other) const;
};

struct Session
{
	SolveType type;
	std::string id, name;
	Update update;
	std::vector<Solve> solves;
	bool dirty;

	int bestSolve(Solve* solve = nullptr);
	int bestAvgOf(size_t count, int* start = nullptr);
	static int avgOf(const std::vector<int>& times);
	int avgOfLast(size_t count, bool ignoreDNF = false);
	int sessionAvg();

	static std::map<SolveType, std::string> solveTypeNames;
	static std::string GetSolveTypeName(SolveType type);
	static bool GetSolveTypeByName(const std::string& name, SolveType& result);
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
