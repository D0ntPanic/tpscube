#include "cubecommon.h"
#include "scramble.h"

using namespace std;


string CubeMoveSequence::MoveToString(CubeMove move)
{
	switch (move)
	{
	case MOVE_U: return "U";
	case MOVE_Up: return "U'";
	case MOVE_U2: return "U2";
	case MOVE_F: return "F";
	case MOVE_Fp: return "F'";
	case MOVE_F2: return "F2";
	case MOVE_R: return "R";
	case MOVE_Rp: return "R'";
	case MOVE_R2: return "R2";
	case MOVE_B: return "B";
	case MOVE_Bp: return "B'";
	case MOVE_B2: return "B2";
	case MOVE_L: return "L";
	case MOVE_Lp: return "L'";
	case MOVE_L2: return "L2";
	case MOVE_D: return "D";
	case MOVE_Dp: return "D'";
	case MOVE_D2: return "D2";
	default: return "";
	}
}


CubeMove CubeMoveSequence::InvertedMove(CubeMove move)
{
	static CubeMove inverted[MOVE_D2 + 1] = {
		MOVE_Up, // MOVE_U
		MOVE_U, // MOVE_Up
		MOVE_U2, // MOVE_U2
		MOVE_Fp, // MOVE_F
		MOVE_F, // MOVE_Fp
		MOVE_F2, // MOVE_F2
		MOVE_Rp, // MOVE_R
		MOVE_R, // MOVE_Rp
		MOVE_R2, // MOVE_R2
		MOVE_Bp, // MOVE_B
		MOVE_B, // MOVE_Bp
		MOVE_B2, // MOVE_B2
		MOVE_Lp, // MOVE_L
		MOVE_L, // MOVE_Lp
		MOVE_L2, // MOVE_L2
		MOVE_Dp, // MOVE_D
		MOVE_D, // MOVE_Dp
		MOVE_D2 // MOVE_D2
	};
	return inverted[move];
}


CubeMove CubeMoveSequence::RandomMove(RandomSource& rng)
{
	return (CubeMove)rng.Next(MOVE_D2 + 1);
}


bool CubeMoveSequence::IsSameOuterBlock(CubeMove a, CubeMove b)
{
	switch (a)
	{
	case MOVE_U:
	case MOVE_Up:
	case MOVE_U2:
		return (b == MOVE_U) || (b == MOVE_Up) || (b == MOVE_U2);
	case MOVE_F:
	case MOVE_Fp:
	case MOVE_F2:
		return (b == MOVE_F) || (b == MOVE_Fp) || (b == MOVE_F2);
	case MOVE_R:
	case MOVE_Rp:
	case MOVE_R2:
		return (b == MOVE_R) || (b == MOVE_Rp) || (b == MOVE_R2);
	case MOVE_B:
	case MOVE_Bp:
	case MOVE_B2:
		return (b == MOVE_B) || (b == MOVE_Bp) || (b == MOVE_B2);
	case MOVE_L:
	case MOVE_Lp:
	case MOVE_L2:
		return (b == MOVE_L) || (b == MOVE_Lp) || (b == MOVE_L2);
	case MOVE_D:
	case MOVE_Dp:
	case MOVE_D2:
		return (b == MOVE_D) || (b == MOVE_Dp) || (b == MOVE_D2);
	default:
		return false;
	}
}


string CubeMoveSequence::ToString() const
{
	string result;
	for (auto i : moves)
	{
		if (result.size() > 0)
			result += string(" ");
		result += MoveToString(i);
	}
	return result;
}


CubeMoveSequence CubeMoveSequence::Inverted() const
{
	CubeMoveSequence result;
	for (auto i = moves.rbegin(); i != moves.rend(); ++i)
		result.moves.push_back(InvertedMove(*i));
	return result;
}


size_t CubeMoveSequence::GetOuterTurnCount() const
{
	if (moves.size() == 0)
		return 0;
	size_t result = 1;
	for (size_t i = 1; i < moves.size(); i++)
	{
		if (!IsSameOuterBlock(moves[i - 1], moves[i]))
			result++;
	}
	return result;
}


size_t TimedCubeMoveSequence::GetOuterTurnCount() const
{
	if (moves.size() == 0)
		return 0;
	size_t result = 1;
	for (size_t i = 1; i < moves.size(); i++)
	{
		if (!CubeMoveSequence::IsSameOuterBlock(moves[i - 1].move, moves[i].move))
			result++;
	}
	return result;
}


int NChooseK(int n, int k)
{
	if (n < k)
		return 0;
	if (k > (n / 2))
		k = n - k;
	int result = 1;
	int denom = 1;
	for (int i = n; i > (n - k); i--)
	{
		result *= i;
		result /= denom++;
	}
	return result;
}
