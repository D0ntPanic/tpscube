#include <stdexcept>
#include "cubecommon.h"
#include "scramble.h"

using namespace std;


vector<string> SplitString(const string& text)
{
	vector<string> result;
	for (size_t pos = 0; pos < text.size(); )
	{
		size_t next = text.find(' ', pos);
		if (next == string::npos)
		{
			result.push_back(text.substr(pos));
			break;
		}
		if (next == pos)
		{
			pos++;
			continue;
		}
		result.push_back(text.substr(pos, next - pos));
		pos = next + 1;
	}
	return result;
}


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


bool CubeMoveSequence::MoveFromString(const string& name, CubeMove& move)
{
	if (name == "U")
		move = MOVE_U;
	else if (name == "U'")
		move = MOVE_Up;
	else if (name == "U2")
		move = MOVE_U2;
	else if (name == "F")
		move = MOVE_F;
	else if (name == "F'")
		move = MOVE_Fp;
	else if (name == "F2")
		move = MOVE_F2;
	else if (name == "R")
		move = MOVE_R;
	else if (name == "R'")
		move = MOVE_Rp;
	else if (name == "R2")
		move = MOVE_R2;
	else if (name == "B")
		move = MOVE_B;
	else if (name == "B'")
		move = MOVE_Bp;
	else if (name == "B2")
		move = MOVE_B2;
	else if (name == "L")
		move = MOVE_L;
	else if (name == "L'")
		move = MOVE_Lp;
	else if (name == "L2")
		move = MOVE_L2;
	else if (name == "D")
		move = MOVE_D;
	else if (name == "D'")
		move = MOVE_Dp;
	else if (name == "D2")
		move = MOVE_D2;
	else
		return false;
	return true;
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


CubeFace CubeMoveSequence::GetMoveFace(CubeMove move)
{
	switch (move)
	{
	case MOVE_U:
	case MOVE_Up:
	case MOVE_U2:
		return TOP;
	case MOVE_F:
	case MOVE_Fp:
	case MOVE_F2:
		return FRONT;
	case MOVE_R:
	case MOVE_Rp:
	case MOVE_R2:
		return RIGHT;
	case MOVE_B:
	case MOVE_Bp:
	case MOVE_B2:
		return BACK;
	case MOVE_L:
	case MOVE_Lp:
	case MOVE_L2:
		return LEFT;
	case MOVE_D:
	case MOVE_Dp:
	case MOVE_D2:
		return BOTTOM;
	default:
		return TOP;
	}
}


int CubeMoveSequence::GetMoveDirection(CubeMove move)
{
	switch (move)
	{
	case MOVE_U:
	case MOVE_F:
	case MOVE_R:
	case MOVE_B:
	case MOVE_L:
	case MOVE_D:
		return 1;
	case MOVE_Up:
	case MOVE_Fp:
	case MOVE_Rp:
	case MOVE_Bp:
	case MOVE_Lp:
	case MOVE_Dp:
		return -1;
	case MOVE_U2:
	case MOVE_F2:
	case MOVE_R2:
	case MOVE_B2:
	case MOVE_L2:
	case MOVE_D2:
		return 2;
	default:
		return 0;
	}
}


CubeMove CubeMoveSequence::GetMoveForFaceAndDirection(CubeFace face, int dir)
{
	switch (face)
	{
	case TOP:
		switch (dir)
		{
		case 1:
			return MOVE_U;
		case -1:
			return MOVE_Up;
		default:
			return MOVE_U2;
		}
	case FRONT:
		switch (dir)
		{
		case 1:
			return MOVE_F;
		case -1:
			return MOVE_Fp;
		default:
			return MOVE_F2;
		}
	case RIGHT:
		switch (dir)
		{
		case 1:
			return MOVE_R;
		case -1:
			return MOVE_Rp;
		default:
			return MOVE_R2;
		}
	case BACK:
		switch (dir)
		{
		case 1:
			return MOVE_B;
		case -1:
			return MOVE_Bp;
		default:
			return MOVE_B2;
		}
	case LEFT:
		switch (dir)
		{
		case 1:
			return MOVE_L;
		case -1:
			return MOVE_Lp;
		default:
			return MOVE_L2;
		}
	case BOTTOM:
		switch (dir)
		{
		case 1:
			return MOVE_D;
		case -1:
			return MOVE_Dp;
		default:
			return MOVE_D2;
		}
	default:
		return MOVE_U;
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


bool CubeMoveSequence::FromString(const string& text, CubeMoveSequence& result)
{
	vector<string> parts = SplitString(text);
	result.moves.clear();
	for (auto& i : parts)
	{
		CubeMove move;
		if (!CubeMoveSequence::MoveFromString(i, move))
			return false;
		result.moves.push_back(move);
	}
	return true;
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


string TimedCubeMoveSequence::ToString() const
{
	string result;
	for (auto& i: moves)
	{
		if (result.size() != 0)
			result += " ";
		result += CubeMoveSequence::MoveToString(i.move);
		char timeStr[64];
		sprintf(timeStr, "@%d", (int)i.timestamp);
		result += timeStr;
	}
	return result;
}


bool TimedCubeMoveSequence::FromString(const string& text, TimedCubeMoveSequence& result)
{
	vector<string> parts = SplitString(text);
	result.moves.clear();
	for (auto& i : parts)
	{
		size_t atPos = i.find('@');
		if (atPos == string::npos)
			return false;

		TimedCubeMove move;
		if (!CubeMoveSequence::MoveFromString(i.substr(0, atPos), move.move))
			return false;

		try
		{
			move.timestamp = stoll(i.substr(atPos + 1));
		}
		catch (invalid_argument&)
		{
			return false;
		}
		catch (out_of_range&)
		{
			return false;
		}

		result.moves.push_back(move);
	}
	return true;
}


AnimatedMoveSequence::AnimatedMoveSequence()
{
}


AnimatedMoveSequence::AnimatedMoveSequence(const TimedCubeMoveSequence& timedMoves)
{
	// Compute default TPS for animated solve (average TPS for solve)
	float defaultTPS = 2.0f;
	if ((timedMoves.moves.size() > 1) && (timedMoves.moves[0].timestamp !=
		timedMoves.moves[timedMoves.moves.size() - 1].timestamp))
	{
		defaultTPS = (float)(timedMoves.moves.size() - 1) /
			(((float)timedMoves.moves[timedMoves.moves.size() - 1].timestamp -
			(float)timedMoves.moves[0].timestamp) / 1000.0f);
	}

	for (size_t i = 0; i < timedMoves.moves.size(); i++)
	{
		if (i == (timedMoves.moves.size() - 1))
		{
			// Last move, use averaged TPS from previous animations
			AnimatedCubeMove move;
			move.move = timedMoves.moves[i].move;
			move.timestamp = timedMoves.moves[i].timestamp;
			move.tps = defaultTPS;
			moves.push_back(move);
			continue;
		}

		// Not last move, first check for back to back single turns and change them
		// into 180 degree turns (F F -> F2)
		CubeMove curMove = timedMoves.moves[i].move;
		CubeMove nextMove = timedMoves.moves[i + 1].move;
		AnimatedCubeMove move;
		move.timestamp = timedMoves.moves[i].timestamp;
		if ((curMove == nextMove) && (CubeMoveSequence::GetMoveDirection(curMove) != 2))
		{
			// Same move repeated twice, join the moves into a single smoother animation
			move.move = CubeMoveSequence::GetMoveForFaceAndDirection(
				CubeMoveSequence::GetMoveFace(curMove), 2);
			if (((i + 2) >= timedMoves.moves.size()) ||
				(timedMoves.moves[i + 2].timestamp == move.timestamp))
			{
				move.tps = defaultTPS;
			}
			else
			{
				move.tps = 2.0f / (((float)timedMoves.moves[i + 2].timestamp -
					(float)move.timestamp) / 1000.0f);
			}
			i++;
		}
		else
		{
			// Not the same moves, animate the move individually
			move.move = curMove;
			if (timedMoves.moves[i + 1].timestamp == move.timestamp)
			{
				move.tps = defaultTPS;
			}
			else
			{
				move.tps = 1.0f / (((float)timedMoves.moves[i + 1].timestamp -
					(float)move.timestamp) / 1000.0f);
			}
		}

		// Cap animated TPS to reasonable ranges based on prior TPS
		if (move.tps > (defaultTPS * 4.0f))
			move.tps = defaultTPS * 4.0f;
		else if (move.tps < (defaultTPS / 2.0f))
			move.tps = defaultTPS / 2.0f;
		moves.push_back(move);
		defaultTPS = (move.tps + defaultTPS) / 2.0f;
	}
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
