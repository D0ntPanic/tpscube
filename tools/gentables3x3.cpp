#include "../lib/cube3x3.cpp"
#include "../lib/cubecommon.cpp"

using namespace std;

int Cube3x3::m_cornerOrientationMoveTable[CORNER_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1];
int Cube3x3::m_cornerPermutationMoveTable[CORNER_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];
int Cube3x3::m_edgeOrientationMoveTable[EDGE_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1];
int Cube3x3::m_equatorialEdgeSliceMoveTable[EDGE_SLICE_INDEX_COUNT][MOVE_D2 + 1];
int Cube3x3::m_phase2EdgePermutationMoveTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];
int Cube3x3::m_phase2EquatorialEdgePermutationMoveTable[PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];
uint8_t Cube3x3::m_cornerOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT];
uint8_t Cube3x3::m_edgeOrientationPruneTable[EDGE_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT];
uint32_t Cube3x3::m_combinedOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_ORIENTATION_INDEX_COUNT / 8];
uint8_t Cube3x3::m_cornerPermutationPruneTable[CORNER_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
uint8_t Cube3x3::m_phase2EdgePermutationPruneTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
uint8_t Cube3x3::m_phase1CornerPermutationPruneTable[CORNER_PERMUTATION_INDEX_COUNT];

int g_cornerOrientationMoveTable[CORNER_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1];
int g_cornerPermutationMoveTable[CORNER_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];
int g_edgeOrientationMoveTable[EDGE_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1];
int g_equatorialEdgeSliceMoveTable[EDGE_SLICE_INDEX_COUNT][MOVE_D2 + 1];
int g_phase2EdgePermutationMoveTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];
int g_phase2EquatorialEdgePermutationMoveTable[PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1];

int g_cornerOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT];
int g_edgeOrientationPruneTable[EDGE_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT];
int g_combinedOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_ORIENTATION_INDEX_COUNT];
int g_cornerPermutationPruneTable[CORNER_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
int g_phase2EdgePermutationPruneTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];


void InitTables()
{
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
			g_cornerOrientationMoveTable[i][j] = -1;
		for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
			g_cornerPermutationMoveTable[i][j] = -1;
		for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
			g_edgeOrientationMoveTable[i][j] = -1;
		for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
			g_phase2EdgePermutationMoveTable[i][j] = -1;
		for (int i = 0; i < EDGE_SLICE_INDEX_COUNT; i++)
			g_equatorialEdgeSliceMoveTable[i][j] = -1;
		for (int i = 0; i < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; i++)
			g_phase2EquatorialEdgePermutationMoveTable[i][j] = -1;
	}

	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
			g_cornerOrientationPruneTable[i][j] = -1;
	for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
			g_edgeOrientationPruneTable[i][j] = -1;
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
		for (int j = 0; j < EDGE_ORIENTATION_INDEX_COUNT; j++)
			g_combinedOrientationPruneTable[i][j] = -1;
	for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
			g_cornerPermutationPruneTable[i][j] = -1;
	for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
			g_phase2EdgePermutationPruneTable[i][j] = -1;
	g_cornerOrientationPruneTable[0][0] = 0;
	g_edgeOrientationPruneTable[0][0] = 0;
	g_combinedOrientationPruneTable[0][0] = 0;
	g_cornerPermutationPruneTable[0][0] = 0;
	g_phase2EdgePermutationPruneTable[0][0] = 0;
}


int GetCornerOrientationMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
		{
			if (g_cornerOrientationMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetCornerPermutationMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
		{
			if (g_cornerPermutationMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetEdgeOrientationMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
		{
			if (g_edgeOrientationMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetEquatorialEdgeSliceMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < EDGE_SLICE_INDEX_COUNT; i++)
		{
			if (g_equatorialEdgeSliceMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetPhase2EdgePermutationMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
		{
			if (g_phase2EdgePermutationMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetPhase2EquatorialEdgePermutationMoveTableFillCount()
{
	int filled = 0;
	for (uint8_t j = MOVE_U; j <= MOVE_D2; j++)
	{
		for (int i = 0; i < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; i++)
		{
			if (g_phase2EquatorialEdgePermutationMoveTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetCornerOrientationPruneTableFillCount()
{
	int filled = 0;
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
	{
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
		{
			if (g_cornerOrientationPruneTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetEdgeOrientationPruneTableFillCount()
{
	int filled = 0;
	for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
	{
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
		{
			if (g_edgeOrientationPruneTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetCombinedOrientationPruneTableFillCount()
{
	int filled = 0;
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
	{
		for (int j = 0; j < EDGE_ORIENTATION_INDEX_COUNT; j++)
		{
			if (g_combinedOrientationPruneTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetCornerPermutationPruneTableFillCount()
{
	int filled = 0;
	for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
	{
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
		{
			if (g_cornerPermutationPruneTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


int GetPhase2EdgePermutationPruneTableFillCount()
{
	int filled = 0;
	for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
	{
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
		{
			if (g_phase2EdgePermutationPruneTable[i][j] != -1)
				filled++;
		}
	}
	return filled;
}


vector<Cube3x3> Phase1Move(vector<Cube3x3> cubes)
{
	vector<Cube3x3> nextCubes;
	for (auto& i : cubes)
	{
		for (uint8_t move = MOVE_U; move <= MOVE_D2; move++)
		{
			Cube3x3 cube = i;

			// Get old indicies so that we know where we came from
			int oldCornerOrientation = cube.GetCornerOrientationIndex();
			int oldCornerPermutation = cube.GetCornerPermutationIndex();
			int oldEdgeOrientation = cube.GetEdgeOrientationIndex();
			int oldEquatorialSlice = cube.GetEquatorialEdgeSliceIndex();

			// Get current move counts for this cube
			if (g_cornerOrientationPruneTable[oldCornerOrientation][oldEquatorialSlice] == -1)
			{
				printf("Cube state has no corner orientation move count:\n");
				Cube3x3Faces(i).PrintDebugState();
				exit(1);
			}
			if (g_edgeOrientationPruneTable[oldEdgeOrientation][oldEquatorialSlice] == -1)
			{
				printf("Cube state has no corner orientation move count:\n");
				Cube3x3Faces(i).PrintDebugState();
				exit(1);
			}
			if (g_combinedOrientationPruneTable[oldCornerOrientation][oldEdgeOrientation] == -1)
			{
				printf("Cube state has no combined orientation move count:\n");
				Cube3x3Faces(i).PrintDebugState();
				exit(1);
			}
			int cornerOrientMoveCount = g_cornerOrientationPruneTable[oldCornerOrientation][oldEquatorialSlice] + 1;
			int edgeOrientMoveCount = g_edgeOrientationPruneTable[oldEdgeOrientation][oldEquatorialSlice] + 1;
			int combinedOrientMoveCount = g_combinedOrientationPruneTable[oldCornerOrientation][oldEdgeOrientation] + 1;

			// Perform the move
			cube.Move((CubeMove)move);

			// Get new indicies for this state
			int newCornerOrientation = cube.GetCornerOrientationIndex();
			int newCornerPermutation = cube.GetCornerPermutationIndex();
			int newEdgeOrientation = cube.GetEdgeOrientationIndex();
			int newEquatorialSlice = cube.GetEquatorialEdgeSliceIndex();

			bool hasNewInfo = false;

			// Update corner orientation move table
			if (g_cornerOrientationMoveTable[oldCornerOrientation][move] == -1)
			{
				g_cornerOrientationMoveTable[oldCornerOrientation][move] = newCornerOrientation;
				hasNewInfo = true;
			}
			else if (g_cornerOrientationMoveTable[oldCornerOrientation][move] != newCornerOrientation)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old corner orientation move table entry: %d\n", g_cornerOrientationMoveTable[oldCornerOrientation][move]);
				printf("New corner orientation index (should match): %d\n", newCornerOrientation);
				exit(1);
			}

			// Update corner permutation move table
			if (g_cornerPermutationMoveTable[oldCornerPermutation][move] == -1)
			{
				g_cornerPermutationMoveTable[oldCornerPermutation][move] = newCornerPermutation;
				hasNewInfo = true;
			}
			else if (g_cornerPermutationMoveTable[oldCornerPermutation][move] != newCornerPermutation)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old corner permutation move table entry: %d\n", g_cornerPermutationMoveTable[oldCornerPermutation][move]);
				printf("New corner permutation index (should match): %d\n", newCornerPermutation);
				exit(1);
			}

			// Update edge orientation move table
			if (g_edgeOrientationMoveTable[oldEdgeOrientation][move] == -1)
			{
				g_edgeOrientationMoveTable[oldEdgeOrientation][move] = newEdgeOrientation;
				hasNewInfo = true;
			}
			else if (g_edgeOrientationMoveTable[oldEdgeOrientation][move] != newEdgeOrientation)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old edge move table entry: %d\n", g_edgeOrientationMoveTable[oldEdgeOrientation][move]);
				printf("New edge orientation index (should match): %d\n", newEdgeOrientation);
				exit(1);
			}

			// Update equatorial edge slice move table
			if (g_equatorialEdgeSliceMoveTable[oldEquatorialSlice][move] == -1)
			{
				g_equatorialEdgeSliceMoveTable[oldEquatorialSlice][move] = newEquatorialSlice;
				hasNewInfo = true;
			}
			else if (g_equatorialEdgeSliceMoveTable[oldEquatorialSlice][move] != newEquatorialSlice)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old equatorial slice move table entry: %d\n", g_equatorialEdgeSliceMoveTable[oldEquatorialSlice][move]);
				printf("New equatorial slice index (should match): %d\n", newEquatorialSlice);
				exit(1);
			}

			// Update prune tables to keep track of minimum number of moves to reach this state from solved
			if ((g_cornerOrientationPruneTable[newCornerOrientation][newEquatorialSlice] == -1) ||
				(cornerOrientMoveCount < g_cornerOrientationPruneTable[newCornerOrientation][newEquatorialSlice]))
			{
				g_cornerOrientationPruneTable[newCornerOrientation][newEquatorialSlice] = cornerOrientMoveCount;
				hasNewInfo = true;
			}
			if ((g_edgeOrientationPruneTable[newEdgeOrientation][newEquatorialSlice] == -1) ||
				(edgeOrientMoveCount < g_edgeOrientationPruneTable[newEdgeOrientation][newEquatorialSlice]))
			{
				g_edgeOrientationPruneTable[newEdgeOrientation][newEquatorialSlice] = edgeOrientMoveCount;
				hasNewInfo = true;
			}
			if ((g_combinedOrientationPruneTable[newCornerOrientation][newEdgeOrientation] == -1) ||
				(combinedOrientMoveCount < g_combinedOrientationPruneTable[newCornerOrientation][newEdgeOrientation]))
			{
				g_combinedOrientationPruneTable[newCornerOrientation][newEdgeOrientation] = combinedOrientMoveCount;
				hasNewInfo = true;
			}

			// If there was new information discovered with this state, add it to the queue for processing
			if (hasNewInfo)
				nextCubes.push_back(cube);
		}
	}
	return nextCubes;
}


vector<Cube3x3> Phase2Move(vector<Cube3x3> cubes)
{
	vector<Cube3x3> nextCubes;
	for (auto& i : cubes)
	{
		for (uint8_t move = MOVE_U; move <= MOVE_D2; move++)
		{
			// Phase 2 contains the moves (U, D, F2, R2, B2, L2)
			if ((move == MOVE_F) || (move == MOVE_Fp) ||
				(move == MOVE_R) || (move == MOVE_Rp) ||
				(move == MOVE_B) || (move == MOVE_Bp) ||
				(move == MOVE_L) || (move == MOVE_Lp))
				continue;

			Cube3x3 cube = i;

			// Get old indicies so that we know where we came from
			int oldCornerPermutation = cube.GetCornerPermutationIndex();
			int oldEdgePermutation = cube.GetPhase2EdgePermutationIndex();
			int oldEquatorialEdgePermutation = cube.GetPhase2EquatorialEdgePermutationIndex();

			// Get current move counts for this cube
			if (g_cornerPermutationPruneTable[oldCornerPermutation][oldEquatorialEdgePermutation] == -1)
			{
				printf("Cube state has no corner permutation move count:\n");
				Cube3x3Faces(i).PrintDebugState();
				exit(1);
			}
			if (g_phase2EdgePermutationPruneTable[oldEdgePermutation][oldEquatorialEdgePermutation] == -1)
			{
				printf("Cube state has no edge permutation move count:\n");
				Cube3x3Faces(i).PrintDebugState();
				exit(1);
			}
			int cornerPermuteMoveCount = g_cornerPermutationPruneTable[oldCornerPermutation][oldEquatorialEdgePermutation] + 1;
			int edgePermuteMoveCount = g_phase2EdgePermutationPruneTable[oldEdgePermutation][oldEquatorialEdgePermutation] + 1;

			// Perform the move
			cube.Move((CubeMove)move);

			// Get new indicies for this state
			int newCornerPermutation = cube.GetCornerPermutationIndex();
			int newEdgePermutation = cube.GetPhase2EdgePermutationIndex();
			int newEquatorialEdgePermutation = cube.GetPhase2EquatorialEdgePermutationIndex();

			bool hasNewInfo = false;

			// Update edge permutation move table
			if (g_phase2EdgePermutationMoveTable[oldEdgePermutation][move] == -1)
			{
				g_phase2EdgePermutationMoveTable[oldEdgePermutation][move] = newEdgePermutation;
				hasNewInfo = true;
			}
			else if (g_phase2EdgePermutationMoveTable[oldEdgePermutation][move] != newEdgePermutation)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old edge permutation move table entry: %d\n", g_phase2EdgePermutationMoveTable[oldEdgePermutation][move]);
				printf("New edge permutation index (should match): %d\n", newEdgePermutation);
				exit(1);
			}

			// Update equatorial edge slice move table
			if (g_phase2EquatorialEdgePermutationMoveTable[oldEquatorialEdgePermutation][move] == -1)
			{
				g_phase2EquatorialEdgePermutationMoveTable[oldEquatorialEdgePermutation][move] = newEquatorialEdgePermutation;
				hasNewInfo = true;
			}
			else if (g_phase2EquatorialEdgePermutationMoveTable[oldEquatorialEdgePermutation][move] != newEquatorialEdgePermutation)
			{
				// Sanity check failed, something is wrong with the index calculation
				printf("Old state:\n");
				Cube3x3Faces(i).PrintDebugState();
				printf("Move: %s\n", CubeMoveSequence::MoveToString((CubeMove)move).c_str());
				printf("New state:\n");
				Cube3x3Faces(cube).PrintDebugState();
				printf("Old equatorial edge permutation move table entry: %d\n",
					g_phase2EquatorialEdgePermutationMoveTable[oldEquatorialEdgePermutation][move]);
				printf("New equatorial edge permutation index (should match): %d\n", newEquatorialEdgePermutation);
				exit(1);
			}

			// Update prune tables to keep track of minimum number of moves to reach this state from solved
			if ((g_cornerPermutationPruneTable[newCornerPermutation][newEquatorialEdgePermutation] == -1) ||
				(cornerPermuteMoveCount < g_cornerPermutationPruneTable[newCornerPermutation][newEquatorialEdgePermutation]))
			{
				g_cornerPermutationPruneTable[newCornerPermutation][newEquatorialEdgePermutation] = cornerPermuteMoveCount;
				hasNewInfo = true;
			}
			if ((g_phase2EdgePermutationPruneTable[newEdgePermutation][newEquatorialEdgePermutation] == -1) ||
				(edgePermuteMoveCount < g_phase2EdgePermutationPruneTable[newEdgePermutation][newEquatorialEdgePermutation]))
			{
				g_phase2EdgePermutationPruneTable[newEdgePermutation][newEquatorialEdgePermutation] = edgePermuteMoveCount;
				hasNewInfo = true;
			}

			// If there was new information discovered with this state, add it to the queue for processing
			if (hasNewInfo)
				nextCubes.push_back(cube);
		}
	}
	return nextCubes;
}


int main()
{
	InitTables();

	// Generate phase 1 move tables and prune tables
	printf("Generating phase 1 tables...\n");
	vector<Cube3x3> activeCubes = { Cube3x3() };
	int i = 0;
	while (activeCubes.size() > 0)
	{
		printf("    Phase 1 move %d...\n", ++i);
		activeCubes = Phase1Move(activeCubes);
		printf("        %d active cube states\n", (int)activeCubes.size());
		printf("        %d / %d corner orientation move table\n", GetCornerOrientationMoveTableFillCount(),
			CORNER_ORIENTATION_INDEX_COUNT * (MOVE_D2 + 1));
		printf("        %d / %d corner permutation move table\n", GetCornerPermutationMoveTableFillCount(),
			CORNER_PERMUTATION_INDEX_COUNT * (MOVE_D2 + 1));
		printf("        %d / %d edge orientation move table\n", GetEdgeOrientationMoveTableFillCount(),
			EDGE_ORIENTATION_INDEX_COUNT * (MOVE_D2 + 1));
		printf("        %d / %d equatorial edge slice move table\n", GetEquatorialEdgeSliceMoveTableFillCount(),
			EDGE_SLICE_INDEX_COUNT * (MOVE_D2 + 1));
		printf("        %d / %d corner orientation prune table\n", GetCornerOrientationPruneTableFillCount(),
			CORNER_ORIENTATION_INDEX_COUNT * EDGE_SLICE_INDEX_COUNT);
		printf("        %d / %d edge orientation prune table\n", GetEdgeOrientationPruneTableFillCount(),
			EDGE_ORIENTATION_INDEX_COUNT * EDGE_SLICE_INDEX_COUNT);
		printf("        %d / %d combined orientation prune table\n", GetCombinedOrientationPruneTableFillCount(),
			CORNER_ORIENTATION_INDEX_COUNT * EDGE_ORIENTATION_INDEX_COUNT);
	}

	// Generate phase 2 move tables and prune tables
	printf("Generating phase 2 tables...\n");
	activeCubes = vector<Cube3x3> { Cube3x3() };
	i = 0;
	while (activeCubes.size() > 0)
	{
		printf("    Phase 2 move %d...\n", ++i);
		activeCubes = Phase2Move(activeCubes);
		printf("        %d active cube states\n", (int)activeCubes.size());
		printf("        %d / %d edge permutation move table\n", GetPhase2EdgePermutationMoveTableFillCount(),
			PHASE_2_EDGE_PERMUTATION_INDEX_COUNT * (MOVE_D2 + 1 - 8));
		printf("        %d / %d equatorial edge slice move table\n", GetPhase2EquatorialEdgePermutationMoveTableFillCount(),
			PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT * (MOVE_D2 + 1 - 8));
		printf("        %d / %d corner permutation prune table\n", GetCornerPermutationPruneTableFillCount(),
			CORNER_PERMUTATION_INDEX_COUNT * PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT);
		printf("        %d / %d edge permutation prune table\n", GetPhase2EdgePermutationPruneTableFillCount(),
			PHASE_2_EDGE_PERMUTATION_INDEX_COUNT * PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT);
	}

	// Output move tables
	FILE* fp = fopen("../lib/cube3x3move_generated.cpp", "w");
	fprintf(fp, "// This file was autogenerated by tools/gentables3x3.cpp\n");
	fprintf(fp, "#include \"cube3x3.h\"\n\n");
	fprintf(fp, "int Cube3x3::m_cornerOrientationMoveTable[CORNER_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_cornerOrientationMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < CORNER_ORIENTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "int Cube3x3::m_cornerPermutationMoveTable[CORNER_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_cornerPermutationMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < CORNER_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "int Cube3x3::m_edgeOrientationMoveTable[EDGE_ORIENTATION_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_edgeOrientationMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < EDGE_ORIENTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n");
	fprintf(fp, "int Cube3x3::m_equatorialEdgeSliceMoveTable[EDGE_SLICE_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < EDGE_SLICE_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_equatorialEdgeSliceMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < EDGE_SLICE_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n");
	fprintf(fp, "int Cube3x3::m_phase2EdgePermutationMoveTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_phase2EdgePermutationMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n");
	fprintf(fp, "int Cube3x3::m_phase2EquatorialEdgePermutationMoveTable[PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT][MOVE_D2 + 1] = {\n");
	for (int i = 0; i < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < (MOVE_D2 + 1); j++)
		{
			fprintf(fp, "%d", g_phase2EquatorialEdgePermutationMoveTable[i][j]);
			if ((j + 1) < (MOVE_D2 + 1))
				fprintf(fp, ", ");
		}
		fprintf(fp, "}");
		if ((i + 1) < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n");
	fclose(fp);

	// Output prune tables
	fp = fopen("../lib/cube3x3prune_generated.cpp", "w");
	fprintf(fp, "// This file was autogenerated by tools/gentables3x3.cpp\n");
	fprintf(fp, "#include \"cube3x3.h\"\n\n");
	fprintf(fp, "uint8_t Cube3x3::m_cornerOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT] = {\n");
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{\n\t\t");
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
		{
			if ((j != 0) && ((j % 50) == 0))
				fprintf(fp, "\n\t\t");
			fprintf(fp, "%d", g_cornerOrientationPruneTable[i][j]);
			if ((j + 1) < EDGE_SLICE_INDEX_COUNT)
				fprintf(fp, ",");
		}
		fprintf(fp, "}");
		if ((i + 1) < CORNER_ORIENTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "uint8_t Cube3x3::m_edgeOrientationPruneTable[EDGE_ORIENTATION_INDEX_COUNT][EDGE_SLICE_INDEX_COUNT] = {\n");
	for (int i = 0; i < EDGE_ORIENTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{\n\t\t");
		for (int j = 0; j < EDGE_SLICE_INDEX_COUNT; j++)
		{
			if ((j != 0) && ((j % 50) == 0))
				fprintf(fp, "\n\t\t");
			fprintf(fp, "%d", g_edgeOrientationPruneTable[i][j]);
			if ((j + 1) < EDGE_SLICE_INDEX_COUNT)
				fprintf(fp, ",");
		}
		fprintf(fp, "}");
		if ((i + 1) < EDGE_ORIENTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "uint32_t Cube3x3::m_combinedOrientationPruneTable[CORNER_ORIENTATION_INDEX_COUNT][EDGE_ORIENTATION_INDEX_COUNT / 8] = {\n");
	for (int i = 0; i < CORNER_ORIENTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{\n\t\t");
		for (int j = 0; j < EDGE_ORIENTATION_INDEX_COUNT; j += 8)
		{
			if ((j != 0) && ((j % 64) == 0))
				fprintf(fp, "\n\t\t");
			uint32_t value = (uint32_t)g_combinedOrientationPruneTable[i][j] |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 1] << 4) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 2] << 8) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 3] << 12) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 4] << 16) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 5] << 20) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 6] << 24) |
				((uint32_t)g_combinedOrientationPruneTable[i][j + 7] << 28);
			fprintf(fp, "0x%x", value);
			if ((j + 8) < EDGE_ORIENTATION_INDEX_COUNT)
				fprintf(fp, ",");
		}
		fprintf(fp, "}");
		if ((i + 1) < CORNER_ORIENTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "uint8_t Cube3x3::m_cornerPermutationPruneTable[CORNER_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT] = {\n");
	for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
		{
			fprintf(fp, "%d", g_cornerPermutationPruneTable[i][j]);
			if ((j + 1) < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT)
				fprintf(fp, ",");
		}
		fprintf(fp, "}");
		if ((i + 1) < CORNER_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "uint8_t Cube3x3::m_phase2EdgePermutationPruneTable[PHASE_2_EDGE_PERMUTATION_INDEX_COUNT][PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT] = {\n");
	for (int i = 0; i < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT; i++)
	{
		fprintf(fp, "\t{");
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
		{
			fprintf(fp, "%d", g_phase2EdgePermutationPruneTable[i][j]);
			if ((j + 1) < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT)
				fprintf(fp, ",");
		}
		fprintf(fp, "}");
		if ((i + 1) < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
		fprintf(fp, "\n");
	}
	fprintf(fp, "};\n\n");
	fprintf(fp, "uint8_t Cube3x3::m_phase1CornerPermutationPruneTable[CORNER_PERMUTATION_INDEX_COUNT] = {\n\t");
	for (int i = 0; i < CORNER_PERMUTATION_INDEX_COUNT; i++)
	{
		if ((i != 0) && ((i % 50) == 0))
			fprintf(fp, "\n\t");
		int minValue = g_cornerPermutationPruneTable[i][0];
		for (int j = 0; j < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT; j++)
		{
			if (g_cornerPermutationPruneTable[i][j] < minValue)
				minValue = g_cornerPermutationPruneTable[i][j];
		}
		fprintf(fp, "%d", minValue);
		if ((i + 1) < CORNER_PERMUTATION_INDEX_COUNT)
			fprintf(fp, ",");
	}
	fprintf(fp, "};\n\n");
	fclose(fp);

	return 0;
}
