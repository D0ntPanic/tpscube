#pragma once

#include <string>
#include <vector>

enum CubeColor: uint8_t
{
	WHITE = 0,
	GREEN = 1,
	RED = 2,
	BLUE = 3,
	ORANGE = 4,
	YELLOW = 5
};

enum CubeFace
{
	TOP = 0,
	FRONT = 1,
	RIGHT = 2,
	BACK = 3,
	LEFT = 4,
	BOTTOM = 5
};

enum CubeRotationDirection
{
	CW = 0,
	CCW = 1
};

enum CubeMove: uint8_t
{
	MOVE_U = 0,
	MOVE_Up = 1,
	MOVE_U2 = 2,
	MOVE_F = 3,
	MOVE_Fp = 4,
	MOVE_F2 = 5,
	MOVE_R = 6,
	MOVE_Rp = 7,
	MOVE_R2 = 8,
	MOVE_B = 9,
	MOVE_Bp = 10,
	MOVE_B2 = 11,
	MOVE_L = 12,
	MOVE_Lp = 13,
	MOVE_L2 = 14,
	MOVE_D = 15,
	MOVE_Dp = 16,
	MOVE_D2 = 17
};

enum CubeCorner: uint8_t
{
	CORNER_URF = 0,
	CORNER_UFL = 1,
	CORNER_ULB = 2,
	CORNER_UBR = 3,
	CORNER_DFR = 4,
	CORNER_DLF = 5,
	CORNER_DBL = 6,
	CORNER_DRB = 7
};

enum CubeEdge: uint8_t
{
	EDGE_UR = 0,
	EDGE_UF = 1,
	EDGE_UL = 2,
	EDGE_UB = 3,
	EDGE_DR = 4,
	EDGE_DF = 5,
	EDGE_DL = 6,
	EDGE_DB = 7,
	EDGE_FR = 8,
	EDGE_FL = 9,
	EDGE_BL = 10,
	EDGE_BR = 11
};

class RandomSource;

struct CubeMoveSequence
{
	std::vector<CubeMove> moves;

	static std::string MoveToString(CubeMove move);
	static bool MoveFromString(const std::string& name, CubeMove& move);
	static CubeMove InvertedMove(CubeMove move);
	static CubeMove RandomMove(RandomSource& rng);
	static bool IsSameOuterBlock(CubeMove a, CubeMove b);
	std::string ToString() const;
	CubeMoveSequence Inverted() const;
	size_t GetOuterTurnCount() const;
};

struct TimedCubeMove
{
	CubeMove move;
	uint64_t timestamp;
};

struct TimedCubeMoveSequence
{
	std::vector<TimedCubeMove> moves;

	size_t GetOuterTurnCount() const;
};

int NChooseK(int n, int k);
