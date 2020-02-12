#include <string.h>
#include <stdio.h>
#include "cube3x3.h"

#define FACE_START(face) ((face) * 9)
#define FACE_OFFSET(row, col) (((row) * 3) + (col))
#define IDX(face, row, col) (FACE_START(face) + FACE_OFFSET(row, col))
#define FACE_FOR_IDX(i) ((CubeFace)((i) / 9))
#define ROW_FOR_IDX(i) (((i) / 3) % 3)
#define COL_FOR_IDX(i) ((i) % 3)

using namespace std;


// Table for rotating the corners in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece comes from and the
// adjustment to the orientation (corner twist).
CubePiece Cube3x3::m_cornerRotation[2][6][8] = {
	// CW
	{
		// Top
		{
			/* URF */ {CORNER_UBR, 0}, /* UFL */ {CORNER_URF, 0},
			/* ULB */ {CORNER_UFL, 0}, /* UBR */ {CORNER_ULB, 0},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_DRB, 0}
		},
		// Front
		{
			/* URF */ {CORNER_UFL, 1}, /* UFL */ {CORNER_DLF, 2},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_URF, 2}, /* DLF */ {CORNER_DFR, 1},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_DRB, 0}
		},
		// Right
		{
			/* URF */ {CORNER_DFR, 2}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_URF, 1},
			/* DFR */ {CORNER_DRB, 1}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_UBR, 2}
		},
		// Back
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_UBR, 1}, /* UBR */ {CORNER_DRB, 2},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_ULB, 2}, /* DRB */ {CORNER_DBL, 1}
		},
		// Left
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_ULB, 1},
			/* ULB */ {CORNER_DBL, 2}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_UFL, 2},
			/* DBL */ {CORNER_DLF, 1}, /* DRB */ {CORNER_DRB, 0}
		},
		// Bottom
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_DLF, 0}, /* DLF */ {CORNER_DBL, 0},
			/* DBL */ {CORNER_DRB, 0}, /* DRB */ {CORNER_DFR, 0}
		}
	},
	// CCW
	{
		// Top
		{
			/* URF */ {CORNER_UFL, 0}, /* UFL */ {CORNER_ULB, 0},
			/* ULB */ {CORNER_UBR, 0}, /* UBR */ {CORNER_URF, 0},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_DRB, 0}
		},
		// Front
		{
			/* URF */ {CORNER_DFR, 1}, /* UFL */ {CORNER_URF, 2},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_DLF, 2}, /* DLF */ {CORNER_UFL, 1},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_DRB, 0}
		},
		// Right
		{
			/* URF */ {CORNER_UBR, 2}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_DRB, 1},
			/* DFR */ {CORNER_URF, 1}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_DBL, 0}, /* DRB */ {CORNER_DFR, 2}
		},
		// Back
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_DBL, 1}, /* UBR */ {CORNER_ULB, 2},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_DLF, 0},
			/* DBL */ {CORNER_DRB, 2}, /* DRB */ {CORNER_UBR, 1}
		},
		// Left
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_DLF, 1},
			/* ULB */ {CORNER_UFL, 2}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_DFR, 0}, /* DLF */ {CORNER_DBL, 2},
			/* DBL */ {CORNER_ULB, 1}, /* DRB */ {CORNER_DRB, 0}
		},
		// Bottom
		{
			/* URF */ {CORNER_URF, 0}, /* UFL */ {CORNER_UFL, 0},
			/* ULB */ {CORNER_ULB, 0}, /* UBR */ {CORNER_UBR, 0},
			/* DFR */ {CORNER_DRB, 0}, /* DLF */ {CORNER_DFR, 0},
			/* DBL */ {CORNER_DLF, 0}, /* DRB */ {CORNER_DBL, 0}
		}
	}
};

// Table for rotating the edges in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece comes from and the
// adjustment to the orientation (edge flip).
CubePiece Cube3x3::m_edgeRotation[2][6][12] = {
	// CW
	{
		// Top
		{
			/* UR */ {EDGE_UB, 0}, /* UF */ {EDGE_UR, 0}, /* UL */ {EDGE_UF, 0}, /* UB */ {EDGE_UL, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Front
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_FL, 1}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_FR, 1}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_UF, 1}, /* FL */ {EDGE_DF, 1}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Right
		{
			/* UR */ {EDGE_FR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_BR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_DR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_UR, 0}
		},
		// Back
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_BR, 1},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_BL, 1},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_UB, 1}, /* BR */ {EDGE_DB, 1}
		},
		// Left
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_BL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_FL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_UL, 0}, /* BL */ {EDGE_DL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Bottom
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DF, 0}, /* DF */ {EDGE_DL, 0}, /* DL */ {EDGE_DB, 0}, /* DB */ {EDGE_DR, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		}
	},
	// CCW
	{
		// Top
		{
			/* UR */ {EDGE_UF, 0}, /* UF */ {EDGE_UL, 0}, /* UL */ {EDGE_UB, 0}, /* UB */ {EDGE_UR, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Front
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_FR, 1}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_FL, 1}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_DF, 1}, /* FL */ {EDGE_UF, 1}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Right
		{
			/* UR */ {EDGE_BR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_FR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_UR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_DR, 0}
		},
		// Back
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_BL, 1},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_DL, 0}, /* DB */ {EDGE_BR, 1},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_DB, 1}, /* BR */ {EDGE_UB, 1}
		},
		// Left
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_FL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DR, 0}, /* DF */ {EDGE_DF, 0}, /* DL */ {EDGE_BL, 0}, /* DB */ {EDGE_DB, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_DL, 0}, /* BL */ {EDGE_UL, 0}, /* BR */ {EDGE_BR, 0}
		},
		// Bottom
		{
			/* UR */ {EDGE_UR, 0}, /* UF */ {EDGE_UF, 0}, /* UL */ {EDGE_UL, 0}, /* UB */ {EDGE_UB, 0},
			/* DR */ {EDGE_DB, 0}, /* DF */ {EDGE_DR, 0}, /* DL */ {EDGE_DF, 0}, /* DB */ {EDGE_DL, 0},
			/* FR */ {EDGE_FR, 0}, /* FL */ {EDGE_FL, 0}, /* BL */ {EDGE_BL, 0}, /* BR */ {EDGE_BR, 0}
		}
	}
};

CubeColor Cube3x3::m_cornerColors[8][3] = {
	{WHITE, RED, GREEN}, // URF
	{WHITE, GREEN, ORANGE}, // UFL
	{WHITE, ORANGE, BLUE}, // ULB
	{WHITE, BLUE, RED}, // UBR
	{YELLOW, GREEN, RED}, // DFR
	{YELLOW, ORANGE, GREEN}, // DLF
	{YELLOW, BLUE, ORANGE}, // DBL
	{YELLOW, RED, BLUE} // DRB
};

CubeColor Cube3x3::m_edgeColors[12][2] = {
	{WHITE, RED}, // UR
	{WHITE, GREEN}, // UF
	{WHITE, ORANGE}, // UL
	{WHITE, BLUE}, // UB
	{YELLOW, RED}, // DR
	{YELLOW, GREEN}, // DF
	{YELLOW, ORANGE}, // DL
	{YELLOW, BLUE}, // DB
	{GREEN, RED}, // FR
	{GREEN, ORANGE}, // FL
	{BLUE, ORANGE}, // BL
	{BLUE, RED} // BR
};

// Set of moves possible as the first move in phase 1 (all moves)
Cube3x3::PossibleSearchMoves Cube3x3::m_possiblePhase1Moves = {
	18, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}
};

// Set of moves that are allowed in phase 1 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
Cube3x3::PossibleSearchMoves Cube3x3::m_possiblePhase1FollowupMoves[MOVE_D2 + 1] = {
	{15, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // U
	{15, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Up
	{15, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // U2
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // F
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Fp
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // F2
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // R
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Rp
	{15, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2, MOVE_B, MOVE_Bp, MOVE_B2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // R2
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // B
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Bp
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_L, MOVE_Lp, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // B2
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // L
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Lp
	{12, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F, MOVE_Fp, MOVE_F2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // L2
	{12, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_L, MOVE_Lp, MOVE_L2}}, // D
	{12, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_L, MOVE_Lp, MOVE_L2}}, // Dp
	{12, {MOVE_F, MOVE_Fp, MOVE_F2, MOVE_R, MOVE_Rp, MOVE_R2,
		MOVE_B, MOVE_Bp, MOVE_B2, MOVE_L, MOVE_Lp, MOVE_L2}} // D2
};

// Set of moves possible as the second move in phase 2 (valid for the phase 2 move set U D F2 R2 B2 L2)
Cube3x3::PossibleSearchMoves Cube3x3::m_possiblePhase2Moves = {
	10, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}
};

// Set of moves that are allowed in phase 2 following each given moves. For example, U should never follow U'.
// Also, avoid move sequences like U D U by forcing opposite faces to be turned only in a single order.
Cube3x3::PossibleSearchMoves Cube3x3::m_possiblePhase2FollowupMoves[MOVE_D2 + 1] = {
	{7, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // U
	{7, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Up
	{7, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // U2
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // F
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Fp
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // F2
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // R
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Rp
	{9, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // R2
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // B
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Bp
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_R2, MOVE_L2, MOVE_D, MOVE_Dp, MOVE_D2}}, // B2
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // L
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // Lp
	{8, {MOVE_U, MOVE_Up, MOVE_U2, MOVE_F2, MOVE_B2, MOVE_D, MOVE_Dp, MOVE_D2}}, // L2
	{4, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2}}, // D
	{4, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2}}, // Dp
	{4, {MOVE_F2, MOVE_R2, MOVE_B2, MOVE_L2}} // D2
};

// Table of adjacent faces on corners for cubes in face color format
uint8_t Cube3x3Faces::m_cornerAdjacency[6][4][2] = {
	// Top
	{
		{IDX(LEFT, 0, 0), IDX(BACK, 0, 2)}, {IDX(BACK, 0, 0), IDX(RIGHT, 0, 2)},
		{IDX(FRONT, 0, 0), IDX(LEFT, 0, 2)}, {IDX(RIGHT, 0, 0), IDX(FRONT, 0, 2)}
	},
	// Front
	{
		{IDX(LEFT, 0, 2), IDX(TOP, 2, 0)}, {IDX(TOP, 2, 2), IDX(RIGHT, 0, 0)},
		{IDX(BOTTOM, 0, 0), IDX(LEFT, 2, 2)}, {IDX(RIGHT, 2, 0), IDX(BOTTOM, 0, 2)}
	},
	// Right
	{
		{IDX(FRONT, 0, 2), IDX(TOP, 2, 2)}, {IDX(TOP, 0, 2), IDX(BACK, 0, 0)},
		{IDX(BOTTOM, 0, 2), IDX(FRONT, 2, 2)}, {IDX(BACK, 2, 0), IDX(BOTTOM, 2, 2)}
	},
	// Back
	{
		{IDX(RIGHT, 0, 2), IDX(TOP, 0, 2)}, {IDX(TOP, 0, 0), IDX(LEFT, 0, 0)},
		{IDX(BOTTOM, 2, 2), IDX(RIGHT, 2, 2)}, {IDX(LEFT, 2, 0), IDX(BOTTOM, 2, 0)}
	},
	// Left
	{
		{IDX(BACK, 0, 2), IDX(TOP, 0, 0)}, {IDX(TOP, 2, 0), IDX(FRONT, 0, 0)},
		{IDX(BOTTOM, 2, 0), IDX(BACK, 2, 2)}, {IDX(FRONT, 2, 0), IDX(BOTTOM, 0, 0)}
	},
	// Bottom
	{
		{IDX(LEFT, 2, 2), IDX(FRONT, 2, 0)}, {IDX(FRONT, 2, 2), IDX(RIGHT, 2, 0)},
		{IDX(BACK, 2, 2), IDX(LEFT, 2, 0)}, {IDX(RIGHT, 2, 2), IDX(BACK, 2, 0)}
	}
};

// Table of adjacent faces on edges for cubes in face color format
uint8_t Cube3x3Faces::m_edgeAdjacency[6][4] = {
	// Top
	{
		IDX(BACK, 0, 1),
		IDX(LEFT, 0, 1), IDX(RIGHT, 0, 1),
		IDX(FRONT, 0, 1)
	},
	// Front
	{
		IDX(TOP, 2, 1),
		IDX(LEFT, 1, 2), IDX(RIGHT, 1, 0),
		IDX(BOTTOM, 0, 1)
	},
	// Right
	{
		IDX(TOP, 1, 2),
		IDX(FRONT, 1, 2), IDX(BACK, 1, 0),
		IDX(BOTTOM, 1, 2)
	},
	// Back
	{
		IDX(TOP, 0, 1),
		IDX(RIGHT, 1, 2), IDX(LEFT, 1, 0),
		IDX(BOTTOM, 2, 1)
	},
	// Left
	{
		IDX(TOP, 1, 0),
		IDX(BACK, 1, 2), IDX(FRONT, 1, 0),
		IDX(BOTTOM, 1, 0)
	},
	// Bottom
	{
		IDX(FRONT, 2, 1),
		IDX(LEFT, 2, 1), IDX(RIGHT, 2, 1),
		IDX(BACK, 2, 1)
	}
};

// Table for rotation of a face in face color format. Each entry is the
// index on a face where the new color comes from.
uint8_t Cube3x3Faces::m_faceRotation[2][9] = {
	// CW
	{
		FACE_OFFSET(2, 0), FACE_OFFSET(1, 0), FACE_OFFSET(0, 0),
		FACE_OFFSET(2, 1), FACE_OFFSET(1, 1), FACE_OFFSET(0, 1),
		FACE_OFFSET(2, 2), FACE_OFFSET(1, 2), FACE_OFFSET(0, 2)
	},
	// CCW
	{
		FACE_OFFSET(0, 2), FACE_OFFSET(1, 2), FACE_OFFSET(2, 2),
		FACE_OFFSET(0, 1), FACE_OFFSET(1, 1), FACE_OFFSET(2, 1),
		FACE_OFFSET(0, 0), FACE_OFFSET(1, 0), FACE_OFFSET(2, 0)
	}
};

// Table for rotation of edges in face color format. Each entry is the
// index of the edge where the new color comes from. Edges are numbered
// as follows: (0, 1), (1, 0), (1, 2), (2, 1)
uint8_t Cube3x3Faces::m_edgeRotation[2][4] = {
	// CW
	{2, 0, 3, 1},
	// CCW
	{1, 3, 0, 2}
};

// Table for rotation of corners in face color format. Each entry is the
// index of the corner where the new color comes from. Corners are numbered
// as follows: (0, 0), (0, 1), (2, 0), (2, 2)
uint8_t Cube3x3Faces::m_cornerRotation[2][4] = {
	// CW
	{1, 3, 0, 2},
	// CCW
	{2, 0, 3, 1}
};

// Table for converting piece format to face color format. First level of
// the array is the corner index in piece format, and the second level of
// the array is for each of the 3 faces on a corner (in the same order as
// the orientation member, which is clockwise if looking straight at the
// corner).
uint8_t Cube3x3Faces::m_cornerIndicies[8][3] = {
	{IDX(TOP, 2, 2), IDX(RIGHT, 0, 0), IDX(FRONT, 0, 2)}, // URF
	{IDX(TOP, 2, 0), IDX(FRONT, 0, 0), IDX(LEFT, 0, 2)}, // UFL
	{IDX(TOP, 0, 0), IDX(LEFT, 0, 0), IDX(BACK, 0, 2)}, // ULB
	{IDX(TOP, 0, 2), IDX(BACK, 0, 0), IDX(RIGHT, 0, 2)}, // UBR
	{IDX(BOTTOM, 0, 2), IDX(FRONT, 2, 2), IDX(RIGHT, 2, 0)}, // DFR
	{IDX(BOTTOM, 0, 0), IDX(LEFT, 2, 2), IDX(FRONT, 2, 0)}, // DLF
	{IDX(BOTTOM, 2, 0), IDX(BACK, 2, 2), IDX(LEFT, 2, 0)}, // DBL
	{IDX(BOTTOM, 2, 2), IDX(RIGHT, 2, 2), IDX(BACK, 2, 0)} // DRB
};

// Table for converting piece format to face color format. First level of
// the array is the edge index in piece format, and the second level of
// the array is for each of the 2 faces on an edge.
uint8_t Cube3x3Faces::m_edgeIndicies[12][2] = {
	{IDX(TOP, 1, 2), IDX(RIGHT, 0, 1)}, // UR
	{IDX(TOP, 2, 1), IDX(FRONT, 0, 1)}, // UF
	{IDX(TOP, 1, 0), IDX(LEFT, 0, 1)}, // UL
	{IDX(TOP, 0, 1), IDX(BACK, 0, 1)}, // UB
	{IDX(BOTTOM, 1, 2), IDX(RIGHT, 2, 1)}, // DR
	{IDX(BOTTOM, 0, 1), IDX(FRONT, 2, 1)}, // DF
	{IDX(BOTTOM, 1, 0), IDX(LEFT, 2, 1)}, // DL
	{IDX(BOTTOM, 2, 1), IDX(BACK, 2, 1)}, // DB
	{IDX(FRONT, 1, 2), IDX(RIGHT, 1, 0)}, // FR
	{IDX(FRONT, 1, 0), IDX(LEFT, 1, 2)}, // FL
	{IDX(BACK, 1, 2), IDX(LEFT, 1, 0)}, // BL
	{IDX(BACK, 1, 0), IDX(RIGHT, 1, 2)} // BR
};


// Generic implementation of Move() function that calls the Rotate() function
// for any compatible cube representation.
template <class T>
static void MoveWithRotation(T& cube, CubeMove move)
{
	switch (move)
	{
	case MOVE_U:
		cube.Rotate(TOP, CW);
		break;
	case MOVE_Up:
		cube.Rotate(TOP, CCW);
		break;
	case MOVE_U2:
		cube.Rotate(TOP, CW);
		cube.Rotate(TOP, CW);
		break;
	case MOVE_F:
		cube.Rotate(FRONT, CW);
		break;
	case MOVE_Fp:
		cube.Rotate(FRONT, CCW);
		break;
	case MOVE_F2:
		cube.Rotate(FRONT, CW);
		cube.Rotate(FRONT, CW);
		break;
	case MOVE_R:
		cube.Rotate(RIGHT, CW);
		break;
	case MOVE_Rp:
		cube.Rotate(RIGHT, CCW);
		break;
	case MOVE_R2:
		cube.Rotate(RIGHT, CW);
		cube.Rotate(RIGHT, CW);
		break;
	case MOVE_B:
		cube.Rotate(BACK, CW);
		break;
	case MOVE_Bp:
		cube.Rotate(BACK, CCW);
		break;
	case MOVE_B2:
		cube.Rotate(BACK, CW);
		cube.Rotate(BACK, CW);
		break;
	case MOVE_L:
		cube.Rotate(LEFT, CW);
		break;
	case MOVE_Lp:
		cube.Rotate(LEFT, CCW);
		break;
	case MOVE_L2:
		cube.Rotate(LEFT, CW);
		cube.Rotate(LEFT, CW);
		break;
	case MOVE_D:
		cube.Rotate(BOTTOM, CW);
		break;
	case MOVE_Dp:
		cube.Rotate(BOTTOM, CCW);
		break;
	case MOVE_D2:
		cube.Rotate(BOTTOM, CW);
		cube.Rotate(BOTTOM, CW);
		break;
	default:
		break;
	}
}


bool CubePiece::operator==(const CubePiece& other) const
{
	return (piece == other.piece) && (orientation == other.orientation);
}


bool CubePiece::operator!=(const CubePiece& other) const
{
	return (piece != other.piece) || (orientation != other.orientation);
}


Cube3x3::Cube3x3()
{
	for (uint8_t i = 0 ; i < 8; i++)
		m_corners[i] = CubePiece { i, 0 };
	for (uint8_t i = 0 ; i < 12; i++)
		m_edges[i] = CubePiece { i, 0 };
}


Cube3x3::Cube3x3(const Cube3x3Faces& cube)
{
	for (uint8_t i = 0; i < 8; i++)
	{
		CubeColor cornerColors[3] = {
			cube.GetCornerColor((CubeCorner)i, 0),
			cube.GetCornerColor((CubeCorner)i, 1),
			cube.GetCornerColor((CubeCorner)i, 2)};

		// Find this corner piece and orientation
		for (uint8_t j = 0; j < 8; j++)
		{
			if ((cornerColors[0] == m_cornerColors[j][0]) &&
				(cornerColors[1] == m_cornerColors[j][1]) &&
				(cornerColors[2] == m_cornerColors[j][2]))
			{
				m_corners[i] = CubePiece { j, 0 };
				break;
			}
			else if ((cornerColors[1] == m_cornerColors[j][0]) &&
				(cornerColors[2] == m_cornerColors[j][1]) &&
				(cornerColors[0] == m_cornerColors[j][2]))
			{
				m_corners[i] = CubePiece { j, 1 };
				break;
			}
			else if ((cornerColors[2] == m_cornerColors[j][0]) &&
				(cornerColors[0] == m_cornerColors[j][1]) &&
				(cornerColors[1] == m_cornerColors[j][2]))
			{
				m_corners[i] = CubePiece { j, 2 };
				break;
			}
		}
	}

	for (uint8_t i = 0; i < 12; i++)
	{
		CubeColor edgeColors[2] = {
			cube.GetEdgeColor((CubeEdge)i, 0),
			cube.GetEdgeColor((CubeEdge)i, 1)};

		// Find this edge piece and orientation
		for (uint8_t j = 0; j < 12; j++)
		{
			if ((edgeColors[0] == m_edgeColors[j][0]) &&
				(edgeColors[1] == m_edgeColors[j][1]))
			{
				m_edges[i] = CubePiece { j, 0 };
				break;
			}
			else if ((edgeColors[1] == m_edgeColors[j][0]) &&
				(edgeColors[0] == m_edgeColors[j][1]))
			{
				m_edges[i] = CubePiece { j, 1 };
				break;
			}
		}
	}
}


void Cube3x3::Rotate(CubeFace face, CubeRotationDirection dir)
{
	// Save existing cube state so that it can be looked up during rotation
	CubePiece oldCorners[8];
	CubePiece oldEdges[12];
	memcpy(oldCorners, m_corners, sizeof(oldCorners));
	memcpy(oldEdges, m_edges, sizeof(oldEdges));

	// Apply corner movement using lookup table
	for (size_t i = 0; i < 8; i++)
	{
		CubePiece src = m_cornerRotation[dir][face][i];
		m_corners[i] = CubePiece { oldCorners[src.piece].piece,
			(uint8_t)((oldCorners[src.piece].orientation + src.orientation) % 3) };
	}

	// Apply edge movement using lookup table
	for (size_t i = 0; i < 12; i++)
	{
		CubePiece src = m_edgeRotation[dir][face][i];
		m_edges[i] = CubePiece { oldEdges[src.piece].piece,
			(uint8_t)(oldEdges[src.piece].orientation ^ src.orientation) };
	}
}


void Cube3x3::Move(CubeMove move)
{
	MoveWithRotation(*this, move);
}


void Cube3x3::GenerateRandomState(RandomSource& rng)
{
	// Randomize the corner pieces
	for (int i = 0; i < 7; i++)
	{
		int n = rng.Next(8);
		if (i != n)
		{
			// Must swap two corners at a time to avoid parity violation
			swap(m_corners[i], m_corners[n]);
			swap(m_corners[6], m_corners[7]);
		}
	}

	// Randomize the edge pieces
	for (int i = 0; i < 11; i++)
	{
		int n = rng.Next(12);
		if (i != n)
		{
			// Must swap two edges at a time to avoid parity violation
			swap(m_edges[i], m_edges[n]);
			swap(m_edges[10], m_edges[11]);
		}
	}

	// Randomize the corner orientations
	int cornerOrientationSum = 0;
	for (int i = 0; i < 7; i++)
	{
		m_corners[i].orientation = rng.Next(3);
		cornerOrientationSum += m_corners[i].orientation;
	}

	// Randomize the edge orientations
	int edgeOrientationSum = 0;
	for (int i = 0; i < 11; i++)
	{
		m_edges[i].orientation = rng.Next(2);
		edgeOrientationSum += m_edges[i].orientation;
	}

	// Make sure all corner orientations add up to a multiple of 3 (otherwise it is not solvable)
	m_corners[7].orientation = (3 - (cornerOrientationSum % 3)) % 3;

	// Make sure all edge orientations add up to a multiple of 2 (otherwise it is not solvable)
	m_edges[11].orientation = edgeOrientationSum & 1;
}


bool Cube3x3::IsSolved()
{
	// Cube is solved if every piece is in its identity position and
	// there are no orientation changes
	for (uint8_t i = 0; i < 8; i++)
	{
		if (m_corners[i].piece != i)
			return false;
		if (m_corners[i].orientation != 0)
			return false;
	}
	for (uint8_t i = 0; i < 12; i++)
	{
		if (m_edges[i].piece != i)
			return false;
		if (m_edges[i].orientation != 0)
			return false;
	}
	return true;
}


bool Cube3x3::operator==(const Cube3x3& cube) const
{
	for (size_t i = 0; i < 8; i++)
	{
		if (m_corners[i] != cube.m_corners[i])
			return false;
	}
	for (size_t i = 0; i < 12; i++)
	{
		if (m_edges[i] != cube.m_edges[i])
			return false;
	}
	return true;
}


bool Cube3x3::operator!=(const Cube3x3& cube) const
{
	return !(*this == cube);
}


int Cube3x3::GetCornerOrientationIndex()
{
	// Index for the corner orientations is a simple base 3 integer representation. The
	// zero index is the solved state. Note that the last corner is not represented in
	// the index as its value is implicit (all corner orientations must add to a
	// multiple of 3).
	int result = 0;
	for (size_t i = 0; i < 7; i++)
		result = (result * 3) + m_corners[i].orientation;
	return result;
}


int Cube3x3::GetCornerPermutationIndex()
{
	// Index for the corner permutations is the representation of the state in the
	// factorial number system (each digit in the number decreases in base, with the
	// digits representing the index of the choice in the remaining possible choices).
	int result = 0;
	for (size_t i = 0; i < 7; i++)
	{
		// Get index in set of remaining options by checking how many of the entries
		// are greater than this one (which is the index in the sorted list of
		// remaining options)
		int cur = 0;
		for (size_t j = i + 1; j < 8; j++)
		{
			if (m_corners[i].piece > m_corners[j].piece)
				cur++;
		}
		result = (result + cur) * (7 - i);
	}
	return result;
}


int Cube3x3::GetEdgeOrientationIndex()
{
	// Index for the edge orientations is a simple binary integer representation. The
	// zero index is the solved state. Note that the last edge is not represented in
	// the index as its value is implicit (all edge orientations must add to a
	// multiple of 2).
	int result = 0;
	for (size_t i = 0; i < 11; i++)
		result = (result * 2) + m_edges[i].orientation;
	return result;
}


int Cube3x3::GetPhase2EdgePermutationIndex()
{
	// Index for the edge permutations is the representation of the state in the
	// factorial number system (each digit in the number decreases in base, with the
	// digits representing the index of the choice in the remaining possible choices).
	// This is the phase 2 edge permutation index, which does not include the edges
	// in the equatorial slice (significantly reducing the count).
	int result = 0;
	for (size_t i = 0; i < 7; i++)
	{
		// Get index in set of remaining options by checking how many of the entries
		// are greater than this one (which is the index in the sorted list of
		// remaining options)
		int cur = 0;
		for (size_t j = i + 1; j < 8; j++)
		{
			if (m_edges[i].piece > m_edges[j].piece)
				cur++;
		}
		result = (result + cur) * (7 - i);
	}
	return result;
}


int Cube3x3::GetEquatorialEdgeSliceIndex()
{
	// Find the positions of the edge pieces that belong in the equatorial slice. For this
	// index, it does not matter what order these pieces are in, so which piece it is can
	// be ignored. The four positions should be in sorted order so that the permutations
	// of ordering do not matter. This allows the index to be generated using the
	// combinatorial number system. Represent in a way that the equatorial slice members
	// have position 0-3 when they are in the equatorial slice (this allows the zero
	// index to represent the solved state).
	int edgePiecePos[4];
	int j = 0;
	for (int i = 0; i < 12; i++)
	{
		if ((m_edges[(i + EDGE_FR) % 12].piece >= EDGE_FR) &&
			(m_edges[(i + EDGE_FR) % 12].piece <= EDGE_BR))
			edgePiecePos[j++] = i;
	}

	// Compute an index using the combinatorial number system. This will be an integer
	// between zero (solved) and NChooseK(12, 4).
	return NChooseK(edgePiecePos[0], 1) + NChooseK(edgePiecePos[1], 2) +
		NChooseK(edgePiecePos[2], 3) + NChooseK(edgePiecePos[3], 4);
}


int Cube3x3::GetPhase2EquatorialEdgePermutationIndex()
{
	// This index is only valid for phase 2 (equatorial edge pieces are already in the slice
	// but not necessarily in the proper places). Index for the edge permutations is the
	// representation of the state in the factorial number system (each digit in the number
	// decreases in base, with the digits representing the index of the choice in the
	// remaining possible choices).
	int result = 0;
	for (size_t i = 0; i < 3; i++)
	{
		// Get index in set of remaining options by checking how many of the entries
		// are greater than this one (which is the index in the sorted list of
		// remaining options)
		int cur = 0;
		for (size_t j = i + 1; j < 4; j++)
		{
			if (m_edges[i + EDGE_FR].piece > m_edges[j + EDGE_FR].piece)
				cur++;
		}
		result = (result + cur) * (3 - i);
	}
	return result;
}


void Cube3x3::SearchPhase1(const Cube3x3& initialState, const Phase1IndexCube& cube,
	int depth, SearchMoveSequence& moves, CubeMoveSequence& bestSolution, int& maxMoves, bool optimal)
{
	if (depth == 0)
	{
		// At the requested depth, check for solutions
		if ((cube.cornerOrientation == 0) && (cube.edgeOrientation == 0) &&
			(cube.equatorialEdgeSlice == 0))
		{
			// If the last move is not R, R', L, L', F, F', B, or B', the search will be repeated in a
			// different phase 2 search. Ignore sequences that fail this check.
			CubeMove lastMove = moves.moves[moves.count - 1];
			if ((lastMove != MOVE_R) && (lastMove != MOVE_Rp) && (lastMove != MOVE_L) && (lastMove != MOVE_Lp) &&
				(lastMove != MOVE_F) && (lastMove != MOVE_Fp) && (lastMove != MOVE_B) && (lastMove != MOVE_Bp))
				return;

			// Translate cube state into phase 2 index form
			Cube3x3 cubeState = initialState;
			for (int i = 0; i < moves.count; i++)
				cubeState.Move(moves.moves[i]);
			Phase2IndexCube phase2Cube;
			phase2Cube.cornerPermutation = cubeState.GetCornerPermutationIndex();
			phase2Cube.edgePermutation = cubeState.GetPhase2EdgePermutationIndex();
			phase2Cube.equatorialEdgePermutation = cubeState.GetPhase2EquatorialEdgePermutationIndex();

			// Search for phase 2 solution using iterative deepening. Do not go beyond the maximum
			// number of moves for the whole solve.
			for (int i = 0; i < (maxMoves - moves.count); i++)
			{
				SearchPhase2(phase2Cube, i, moves, bestSolution, maxMoves, optimal);
				if ((!optimal) && (bestSolution.moves.size() != 0))
					return;
			}
		}
		return;
	}

	if (moves.count >= maxMoves)
		return;

	if (m_cornerOrientationPruneTable[cube.cornerOrientation][cube.equatorialEdgeSlice] > depth)
		return;
	if (m_edgeOrientationPruneTable[cube.edgeOrientation][cube.equatorialEdgeSlice] > depth)
		return;

	// Need to go deeper. Don't bother to check for solutions in this case. We are using iterative
	// deepening, which means that any intermediate solution must have been passed to phase 2
	// already, so we don't want to repeat the work. Iterate through the possible moves.
	int moveIdx = moves.count++;
	const PossibleSearchMoves* possibleMoves;
	if (moveIdx == 0)
		possibleMoves = &m_possiblePhase1Moves;
	else
		possibleMoves = &m_possiblePhase1FollowupMoves[moves.moves[moveIdx - 1]];
	for (int i = 0; i < possibleMoves->count; i++)
	{
		CubeMove move = possibleMoves->moves[i];
		moves.moves[moveIdx] = move;

		// Use move tables to transition to the next state for this move
		Phase1IndexCube newCube;
		newCube.cornerOrientation = m_cornerOrientationMoveTable[cube.cornerOrientation][move];
		newCube.edgeOrientation = m_edgeOrientationMoveTable[cube.edgeOrientation][move];
		newCube.equatorialEdgeSlice = m_equatorialEdgeSliceMoveTable[cube.equatorialEdgeSlice][move];

		// Proceed further into phase 1
		SearchPhase1(initialState, newCube, depth - 1, moves, bestSolution, maxMoves, optimal);

		if ((!optimal) && (bestSolution.moves.size() != 0))
			break;
		if (moves.count > maxMoves)
			break;
	}
	moves.count--;
}


void Cube3x3::SearchPhase2(const Phase2IndexCube& cube, int depth, SearchMoveSequence& moves,
	CubeMoveSequence& bestSolution, int& maxMoves, bool optimal)
{
	if ((cube.cornerPermutation == 0) && (cube.edgePermutation == 0) && (cube.equatorialEdgePermutation == 0))
	{
		if ((bestSolution.moves.size() == 0) || (moves.count < (int)bestSolution.moves.size()))
		{
			bestSolution.moves = vector<CubeMove>(&moves.moves[0], &moves.moves[moves.count]);
			maxMoves = moves.count - 1;
		}
		return;
	}

	if (moves.count >= maxMoves)
		return;

	if (depth > 0)
	{
		if (m_cornerPermutationPruneTable[cube.cornerPermutation][cube.equatorialEdgePermutation] > depth)
			return;
		if (m_phase2EdgePermutationPruneTable[cube.edgePermutation][cube.equatorialEdgePermutation] > depth)
			return;

		// Need to go deeper. Iterate through the possible moves.
		int moveIdx = moves.count++;
		const PossibleSearchMoves* possibleMoves;
		if (moveIdx == 0)
			possibleMoves = &m_possiblePhase2Moves;
		else
			possibleMoves = &m_possiblePhase2FollowupMoves[moves.moves[moveIdx - 1]];
		for (int i = 0; i < possibleMoves->count; i++)
		{
			CubeMove move = possibleMoves->moves[i];
			moves.moves[moveIdx] = move;

			// Use move tables to transition to the next state for this move
			Phase2IndexCube newCube;
			newCube.cornerPermutation = m_cornerPermutationMoveTable[cube.cornerPermutation][move];
			newCube.edgePermutation = m_phase2EdgePermutationMoveTable[cube.edgePermutation][move];
			newCube.equatorialEdgePermutation = m_phase2EquatorialEdgePermutationMoveTable[cube.equatorialEdgePermutation][move];

			// Proceed further into phase 2
			SearchPhase2(newCube, depth - 1, moves, bestSolution, maxMoves, optimal);

			if ((!optimal) && (bestSolution.moves.size() != 0))
				break;
			if (moves.count > maxMoves)
				break;
		}
		moves.count--;
	}
}


CubeMoveSequence Cube3x3::Solve(bool optimal)
{
	int maxMoves = MAX_3X3_SOLUTION_MOVES;
	CubeMoveSequence bestSolution;

	// If already solved, solution is zero moves
	if (IsSolved())
		return bestSolution;

	Phase1IndexCube cube;
	cube.cornerOrientation = GetCornerOrientationIndex();
	cube.edgeOrientation = GetEdgeOrientationIndex();
	cube.equatorialEdgeSlice = GetEquatorialEdgeSliceIndex();

	SearchMoveSequence moves;
	moves.count = 0;

	for (int depth = 1; (depth <= MAX_3x3_PHASE_1_MOVES) && (depth <= maxMoves); depth++)
		SearchPhase1(*this, cube, depth, moves, bestSolution, maxMoves, optimal);
	return bestSolution;
}


Cube3x3Faces::Cube3x3Faces()
{
	for (size_t i = 0; i < 9; i++)
	{
		m_state[FACE_START(TOP) + i] = WHITE;
		m_state[FACE_START(FRONT) + i] = GREEN;
		m_state[FACE_START(RIGHT) + i] = RED;
		m_state[FACE_START(BACK) + i] = BLUE;
		m_state[FACE_START(LEFT) + i] = ORANGE;
		m_state[FACE_START(BOTTOM) + i] = YELLOW;
	}
}


Cube3x3Faces::Cube3x3Faces(const Cube3x3& cube)
{
	m_state[IDX(TOP, 1, 1)] = WHITE;
	m_state[IDX(FRONT, 1, 1)] = GREEN;
	m_state[IDX(RIGHT, 1, 1)] = RED;
	m_state[IDX(BACK, 1, 1)] = BLUE;
	m_state[IDX(LEFT, 1, 1)] = ORANGE;
	m_state[IDX(BOTTOM, 1, 1)] = YELLOW;

	// Translate corner pieces into face colors
	for (size_t i = 0; i < 8; i++)
	{
		const CubePiece& piece = cube.Corner((CubeCorner)i);
		for (size_t j = 0; j < 3; j++)
		{
			uint8_t dest = m_cornerIndicies[i][j];
			uint8_t src = m_cornerIndicies[piece.piece][(j + 3 - piece.orientation) % 3];
			CubeFace face = FACE_FOR_IDX(src);
			m_state[dest] = m_state[IDX(face, 1, 1)];
		}
	}

	// Translate edge pieces into face colors
	for (size_t i = 0; i < 12; i++)
	{
		const CubePiece& piece = cube.Edge((CubeEdge)i);
		for (size_t j = 0; j < 2; j++)
		{
			uint8_t dest = m_edgeIndicies[i][j];
			uint8_t src = m_edgeIndicies[piece.piece][j ^ piece.orientation];
			CubeFace face = FACE_FOR_IDX(src);
			m_state[dest] = m_state[IDX(face, 1, 1)];
		}
	}
}


void Cube3x3Faces::Rotate(CubeFace face, CubeRotationDirection dir)
{
	// Rotate colors on face itself
	CubeColor rotatedColors[9];
	for (size_t i = 0; i < 9; i++)
		rotatedColors[i] = m_state[FACE_START(face) + m_faceRotation[dir][i]];
	for (size_t i = 0; i < 9; i++)
		m_state[FACE_START(face) + i] = rotatedColors[i];

	// Collect colors on edges and corners
	CubeColor adjacentEdgeColors[4];
	CubeColor adjacentCornerColors[4][2];
	for (size_t i = 0; i < 4; i++)
	{
		adjacentEdgeColors[i] = m_state[m_edgeAdjacency[face][i]];
		adjacentCornerColors[i][0] = m_state[m_cornerAdjacency[face][i][0]];
		adjacentCornerColors[i][1] = m_state[m_cornerAdjacency[face][i][1]];
	}
	// Rotate colors on edges and corners
	for (size_t i = 0; i < 4; i++)
	{
		size_t j = m_edgeRotation[dir][i];
		size_t k = m_cornerRotation[dir][i];
		m_state[m_edgeAdjacency[face][j]] = adjacentEdgeColors[i];
		m_state[m_cornerAdjacency[face][k][0]] = adjacentCornerColors[i][0];
		m_state[m_cornerAdjacency[face][k][1]] = adjacentCornerColors[i][1];
	}
}


void Cube3x3Faces::Move(CubeMove move)
{
	MoveWithRotation(*this, move);
}


CubeColor Cube3x3Faces::GetColor(CubeFace face, uint8_t row, uint8_t col) const
{
	return m_state[IDX(face, row, col)];
}


CubeColor Cube3x3Faces::GetCornerColor(CubeCorner corner, size_t i) const
{
	return m_state[m_cornerIndicies[corner][i]];
}


CubeColor Cube3x3Faces::GetEdgeColor(CubeEdge edge, size_t i) const
{
	return m_state[m_edgeIndicies[edge][i]];
}


bool Cube3x3Faces::IsSolved() const
{
	for (size_t i = 0; i < 6; i++)
	{
		for (size_t j = 0; j < 9; j++)
		{
			// All colors on a face must match center
			if (m_state[FACE_START(i) + j] != m_state[IDX(i, 1, 1)])
				return false;
		}
	}
	return true;
}


bool Cube3x3Faces::operator==(const Cube3x3Faces& cube) const
{
	for (size_t i = 0; i < 6 * 9; i++)
	{
		if (m_state[i] != cube.m_state[i])
			return false;
	}
	return true;
}


bool Cube3x3Faces::operator!=(const Cube3x3Faces& cube) const
{
	return !(*this == cube);
}


CubeMoveSequence Cube3x3Faces::Solve(bool optimal)
{
	return Cube3x3(*this).Solve(optimal);
}


void Cube3x3Faces::PrintDebugState() const
{
	char debugState[9][13];
	memset(debugState, ' ', sizeof(debugState));
	for (size_t i = 0; i < 9; i++)
		debugState[i][12] = 0;
	static int faceX[6] = {3, 3, 6, 9, 0, 3};
	static int faceY[6] = {0, 3, 3, 3, 3, 6};
	for (size_t i = 0; i < 6; i++)
	{
		for (size_t j = 0; j < 3; j++)
		{
			for (size_t k = 0; k < 3; k++)
			{
				CubeColor color = m_state[IDX(i, j, k)];
				switch (color)
				{
				case WHITE:
					debugState[faceY[i] + j][faceX[i] + k] = 'W';
					break;
				case GREEN:
					debugState[faceY[i] + j][faceX[i] + k] = 'G';
					break;
				case RED:
					debugState[faceY[i] + j][faceX[i] + k] = 'R';
					break;
				case BLUE:
					debugState[faceY[i] + j][faceX[i] + k] = 'B';
					break;
				case ORANGE:
					debugState[faceY[i] + j][faceX[i] + k] = 'O';
					break;
				case YELLOW:
					debugState[faceY[i] + j][faceX[i] + k] = 'Y';
					break;
				default:
					debugState[faceY[i] + j][faceX[i] + k] = '-';
					break;
				}
			}
		}
	}
	for (size_t i = 0; i < 9; i++)
		fprintf(stderr, "%s\n", debugState[i]);
}


CubeMoveSequence Cube3x3RandomStateScramble::GetScramble(RandomSource& rng)
{
	CubeMoveSequence result;
	while (true)
	{
		Cube3x3 cube;
		cube.GenerateRandomState(rng);
		result = cube.Solve().Inverted();
		if (result.moves.size() >= 4)
			return result;
	}
}
