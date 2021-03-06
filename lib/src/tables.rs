use crate::common::{Color, Face};
use crate::cube3x3x3::{
    Corner3x3x3, CornerPiece3x3x3, Cube3x3x3Faces, Edge3x3x3, EdgePiece3x3x3, FaceRowOrColumn,
};

#[cfg(not(feature = "no_solver"))]
use crate::common::Move;

// Table for rotating the corners in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece comes from and the
// adjustment to the orientation (corner twist).
pub(crate) const CUBE3_CORNER_PIECE_ROTATION: [[[CornerPiece3x3x3; 8]; 6]; 2] = [
    // CW
    [
        // Top
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Front
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 1,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 2,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 2,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 1,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Right
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 2,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 1,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 1,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 2,
            },
        ],
        // Back
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 1,
            },
            // UBR/
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 2,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 2,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 1,
            },
        ],
        // Left
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 1,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 2,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 2,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 1,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Bottom
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
        ],
    ],
    // CCW
    [
        // Top
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Front
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 1,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 2,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 2,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 1,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Right
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 2,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 1,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 1,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 2,
            },
        ],
        // Back
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 1,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 2,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 2,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 1,
            },
        ],
        // Left
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 1,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 2,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 2,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 1,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
        ],
        // Bottom
        [
            // URF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::URF,
                orientation: 0,
            },
            // UFL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UFL,
                orientation: 0,
            },
            // ULB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::ULB,
                orientation: 0,
            },
            // UBR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::UBR,
                orientation: 0,
            },
            // DFR
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DRB,
                orientation: 0,
            },
            // DLF
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DFR,
                orientation: 0,
            },
            // DBL
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DLF,
                orientation: 0,
            },
            // DRB
            CornerPiece3x3x3 {
                piece: Corner3x3x3::DBL,
                orientation: 0,
            },
        ],
    ],
];

// Table for rotating the edges in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece comes from and the
// adjustment to the orientation (edge flip).
pub(crate) const CUBE3_EDGE_PIECE_ROTATION: [[[EdgePiece3x3x3; 12]; 6]; 2] = [
    // CW
    [
        // Top
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Front
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 1,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 1,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 1,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 1,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Right
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
        ],
        // Back
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 1,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 1,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 1,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 1,
            },
        ],
        // Left
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Bottom
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
    ],
    // CCW
    [
        // Top
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Front
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 1,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 1,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 1,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 1,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Right
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
        ],
        // Back
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 1,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 1,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 1,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 1,
            },
        ],
        // Left
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
        // Bottom
        [
            // UR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UR,
                orientation: 0,
            },
            // UF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UF,
                orientation: 0,
            },
            // UL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UL,
                orientation: 0,
            },
            // UB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::UB,
                orientation: 0,
            },
            // DR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DB,
                orientation: 0,
            },
            // DF
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DR,
                orientation: 0,
            },
            // DL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DF,
                orientation: 0,
            },
            // DB
            EdgePiece3x3x3 {
                piece: Edge3x3x3::DL,
                orientation: 0,
            },
            // FR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FR,
                orientation: 0,
            },
            // FL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::FL,
                orientation: 0,
            },
            // BL
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BL,
                orientation: 0,
            },
            // BR
            EdgePiece3x3x3 {
                piece: Edge3x3x3::BR,
                orientation: 0,
            },
        ],
    ],
];

// Table of adjacent faces on corners for cubes in face color format
pub(crate) const CUBE3_CORNER_ADJACENCY: [[[usize; 2]; 4]; 6] = [
    // Top
    [
        [
            Cube3x3x3Faces::idx(Face::Left, 0, 0),
            Cube3x3x3Faces::idx(Face::Back, 0, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 0, 0),
            Cube3x3x3Faces::idx(Face::Right, 0, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 0, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 2),
        ],
    ],
    // Front
    [
        [
            Cube3x3x3Faces::idx(Face::Left, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 2, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 0, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 2, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
        ],
    ],
    // Right
    [
        [
            Cube3x3x3Faces::idx(Face::Front, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 2, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 0, 2),
            Cube3x3x3Faces::idx(Face::Back, 0, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
        ],
    ],
    // Back
    [
        [
            Cube3x3x3Faces::idx(Face::Right, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 0, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Left, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
        ],
    ],
    // Left
    [
        [
            Cube3x3x3Faces::idx(Face::Back, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 0, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 2, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
            Cube3x3x3Faces::idx(Face::Back, 2, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
        ],
    ],
    // Bottom
    [
        [
            Cube3x3x3Faces::idx(Face::Left, 2, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 2, 2),
            Cube3x3x3Faces::idx(Face::Left, 2, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 2, 2),
            Cube3x3x3Faces::idx(Face::Back, 2, 0),
        ],
    ],
];

// Table of the parts of F2L pairs for each possible cross color
pub(crate) const CUBE3_F2L_PAIRS: [[[usize; 5]; 4]; 6] = [
    // Top
    [
        [
            Cube3x3x3Faces::idx(Face::Top, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 1, 0),
            Cube3x3x3Faces::idx(Face::Back, 0, 2),
            Cube3x3x3Faces::idx(Face::Back, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 0, 2),
            Cube3x3x3Faces::idx(Face::Back, 0, 0),
            Cube3x3x3Faces::idx(Face::Back, 1, 0),
            Cube3x3x3Faces::idx(Face::Right, 0, 2),
            Cube3x3x3Faces::idx(Face::Right, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 2, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 0),
            Cube3x3x3Faces::idx(Face::Front, 1, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 2),
            Cube3x3x3Faces::idx(Face::Left, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Top, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 0, 0),
            Cube3x3x3Faces::idx(Face::Right, 1, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 2),
            Cube3x3x3Faces::idx(Face::Front, 1, 2),
        ],
    ],
    // Front
    [
        [
            Cube3x3x3Faces::idx(Face::Front, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 2),
            Cube3x3x3Faces::idx(Face::Left, 0, 1),
            Cube3x3x3Faces::idx(Face::Top, 2, 0),
            Cube3x3x3Faces::idx(Face::Top, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 2, 2),
            Cube3x3x3Faces::idx(Face::Top, 1, 2),
            Cube3x3x3Faces::idx(Face::Right, 0, 0),
            Cube3x3x3Faces::idx(Face::Right, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 1, 0),
            Cube3x3x3Faces::idx(Face::Left, 2, 2),
            Cube3x3x3Faces::idx(Face::Left, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Front, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 0),
            Cube3x3x3Faces::idx(Face::Right, 2, 1),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
            Cube3x3x3Faces::idx(Face::Bottom, 1, 2),
        ],
    ],
    // Right
    [
        [
            Cube3x3x3Faces::idx(Face::Right, 0, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 2),
            Cube3x3x3Faces::idx(Face::Front, 0, 1),
            Cube3x3x3Faces::idx(Face::Top, 2, 2),
            Cube3x3x3Faces::idx(Face::Top, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 0, 1),
            Cube3x3x3Faces::idx(Face::Back, 0, 0),
            Cube3x3x3Faces::idx(Face::Back, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 1),
            Cube3x3x3Faces::idx(Face::Front, 2, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Right, 2, 2),
            Cube3x3x3Faces::idx(Face::Back, 2, 0),
            Cube3x3x3Faces::idx(Face::Back, 2, 1),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 1),
        ],
    ],
    // Back
    [
        [
            Cube3x3x3Faces::idx(Face::Back, 0, 0),
            Cube3x3x3Faces::idx(Face::Right, 0, 2),
            Cube3x3x3Faces::idx(Face::Right, 0, 1),
            Cube3x3x3Faces::idx(Face::Top, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 0, 0),
            Cube3x3x3Faces::idx(Face::Top, 1, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
            Cube3x3x3Faces::idx(Face::Bottom, 1, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Back, 2, 2),
            Cube3x3x3Faces::idx(Face::Left, 2, 0),
            Cube3x3x3Faces::idx(Face::Left, 2, 1),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 1, 0),
        ],
    ],
    // Left
    [
        [
            Cube3x3x3Faces::idx(Face::Left, 0, 0),
            Cube3x3x3Faces::idx(Face::Back, 0, 2),
            Cube3x3x3Faces::idx(Face::Back, 0, 1),
            Cube3x3x3Faces::idx(Face::Top, 0, 0),
            Cube3x3x3Faces::idx(Face::Top, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Left, 0, 2),
            Cube3x3x3Faces::idx(Face::Top, 2, 0),
            Cube3x3x3Faces::idx(Face::Top, 2, 1),
            Cube3x3x3Faces::idx(Face::Front, 0, 0),
            Cube3x3x3Faces::idx(Face::Front, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Left, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 2, 1),
            Cube3x3x3Faces::idx(Face::Back, 2, 2),
            Cube3x3x3Faces::idx(Face::Back, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(Face::Left, 2, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 0),
            Cube3x3x3Faces::idx(Face::Front, 2, 1),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
            Cube3x3x3Faces::idx(Face::Bottom, 0, 1),
        ],
    ],
    // Bottom
    [
        [
            Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
            Cube3x3x3Faces::idx(Face::Left, 2, 2),
            Cube3x3x3Faces::idx(Face::Left, 1, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 0),
            Cube3x3x3Faces::idx(Face::Front, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
            Cube3x3x3Faces::idx(Face::Front, 2, 2),
            Cube3x3x3Faces::idx(Face::Front, 1, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 0),
            Cube3x3x3Faces::idx(Face::Right, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
            Cube3x3x3Faces::idx(Face::Back, 2, 2),
            Cube3x3x3Faces::idx(Face::Back, 1, 2),
            Cube3x3x3Faces::idx(Face::Left, 2, 0),
            Cube3x3x3Faces::idx(Face::Left, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 2, 2),
            Cube3x3x3Faces::idx(Face::Right, 1, 2),
            Cube3x3x3Faces::idx(Face::Back, 2, 0),
            Cube3x3x3Faces::idx(Face::Back, 1, 0),
        ],
    ],
];

// Table of adjacent faces on edges for cubes in face color format
pub(crate) const CUBE3_EDGE_ADJACENCY: [[usize; 4]; 6] = [
    // Top
    [
        Cube3x3x3Faces::idx(Face::Back, 0, 1),
        Cube3x3x3Faces::idx(Face::Left, 0, 1),
        Cube3x3x3Faces::idx(Face::Right, 0, 1),
        Cube3x3x3Faces::idx(Face::Front, 0, 1),
    ],
    // Front
    [
        Cube3x3x3Faces::idx(Face::Top, 2, 1),
        Cube3x3x3Faces::idx(Face::Left, 1, 2),
        Cube3x3x3Faces::idx(Face::Right, 1, 0),
        Cube3x3x3Faces::idx(Face::Bottom, 0, 1),
    ],
    // Right
    [
        Cube3x3x3Faces::idx(Face::Top, 1, 2),
        Cube3x3x3Faces::idx(Face::Front, 1, 2),
        Cube3x3x3Faces::idx(Face::Back, 1, 0),
        Cube3x3x3Faces::idx(Face::Bottom, 1, 2),
    ],
    // Back
    [
        Cube3x3x3Faces::idx(Face::Top, 0, 1),
        Cube3x3x3Faces::idx(Face::Right, 1, 2),
        Cube3x3x3Faces::idx(Face::Left, 1, 0),
        Cube3x3x3Faces::idx(Face::Bottom, 2, 1),
    ],
    // Left
    [
        Cube3x3x3Faces::idx(Face::Top, 1, 0),
        Cube3x3x3Faces::idx(Face::Back, 1, 2),
        Cube3x3x3Faces::idx(Face::Front, 1, 0),
        Cube3x3x3Faces::idx(Face::Bottom, 1, 0),
    ],
    // Bottom
    [
        Cube3x3x3Faces::idx(Face::Front, 2, 1),
        Cube3x3x3Faces::idx(Face::Left, 2, 1),
        Cube3x3x3Faces::idx(Face::Right, 2, 1),
        Cube3x3x3Faces::idx(Face::Back, 2, 1),
    ],
];

// Table for rotation of a face in face color format. Each entry is the
// index on a face where the new color comes from.
pub(crate) const CUBE3_FACE_ROTATION: [[usize; 9]; 2] = [
    // CW
    [
        Cube3x3x3Faces::face_offset(2, 0),
        Cube3x3x3Faces::face_offset(1, 0),
        Cube3x3x3Faces::face_offset(0, 0),
        Cube3x3x3Faces::face_offset(2, 1),
        Cube3x3x3Faces::face_offset(1, 1),
        Cube3x3x3Faces::face_offset(0, 1),
        Cube3x3x3Faces::face_offset(2, 2),
        Cube3x3x3Faces::face_offset(1, 2),
        Cube3x3x3Faces::face_offset(0, 2),
    ],
    // CCW
    [
        Cube3x3x3Faces::face_offset(0, 2),
        Cube3x3x3Faces::face_offset(1, 2),
        Cube3x3x3Faces::face_offset(2, 2),
        Cube3x3x3Faces::face_offset(0, 1),
        Cube3x3x3Faces::face_offset(1, 1),
        Cube3x3x3Faces::face_offset(2, 1),
        Cube3x3x3Faces::face_offset(0, 0),
        Cube3x3x3Faces::face_offset(1, 0),
        Cube3x3x3Faces::face_offset(2, 0),
    ],
];

// Table for rotation of edges in face color format. Each entry is the
// index of the edge where the new color comes from. Edges are numbered
// as follows: (0, 1), (1, 0), (1, 2), (2, 1)
pub(crate) const CUBE3_EDGE_ROTATION: [[usize; 4]; 2] = [
    // CW
    [2, 0, 3, 1],
    // CCW
    [1, 3, 0, 2],
];

// Table for rotation of corners in face color format. Each entry is the
// index of the corner where the new color comes from. Corners are numbered
// as follows: (0, 0), (0, 1), (2, 0), (2, 2)
pub(crate) const CUBE3_CORNER_ROTATION: [[usize; 4]; 2] = [
    // CW
    [1, 3, 0, 2],
    // CCW
    [2, 0, 3, 1],
];

// Table for converting piece format to face color format. First level of
// the array is the corner index in piece format, and the second level of
// the array is for each of the 3 faces on a corner (in the same order as
// the orientation member, which is clockwise if looking straight at the
// corner).
pub(crate) const CUBE3_CORNER_INDICIES: [[usize; 3]; 8] = [
    // URF
    [
        Cube3x3x3Faces::idx(Face::Top, 2, 2),
        Cube3x3x3Faces::idx(Face::Right, 0, 0),
        Cube3x3x3Faces::idx(Face::Front, 0, 2),
    ],
    // UFL
    [
        Cube3x3x3Faces::idx(Face::Top, 2, 0),
        Cube3x3x3Faces::idx(Face::Front, 0, 0),
        Cube3x3x3Faces::idx(Face::Left, 0, 2),
    ],
    // ULB
    [
        Cube3x3x3Faces::idx(Face::Top, 0, 0),
        Cube3x3x3Faces::idx(Face::Left, 0, 0),
        Cube3x3x3Faces::idx(Face::Back, 0, 2),
    ],
    // UBR
    [
        Cube3x3x3Faces::idx(Face::Top, 0, 2),
        Cube3x3x3Faces::idx(Face::Back, 0, 0),
        Cube3x3x3Faces::idx(Face::Right, 0, 2),
    ],
    // DFR
    [
        Cube3x3x3Faces::idx(Face::Bottom, 0, 2),
        Cube3x3x3Faces::idx(Face::Front, 2, 2),
        Cube3x3x3Faces::idx(Face::Right, 2, 0),
    ],
    // DLF
    [
        Cube3x3x3Faces::idx(Face::Bottom, 0, 0),
        Cube3x3x3Faces::idx(Face::Left, 2, 2),
        Cube3x3x3Faces::idx(Face::Front, 2, 0),
    ],
    // DBL
    [
        Cube3x3x3Faces::idx(Face::Bottom, 2, 0),
        Cube3x3x3Faces::idx(Face::Back, 2, 2),
        Cube3x3x3Faces::idx(Face::Left, 2, 0),
    ],
    // DRB
    [
        Cube3x3x3Faces::idx(Face::Bottom, 2, 2),
        Cube3x3x3Faces::idx(Face::Right, 2, 2),
        Cube3x3x3Faces::idx(Face::Back, 2, 0),
    ],
];

// Table for converting piece format to face color format. First level of
// the array is the edge index in piece format, and the second level of
// the array is for each of the 2 faces on an edge.
pub(crate) const CUBE3_EDGE_INDICIES: [[usize; 2]; 12] = [
    // UR
    [
        Cube3x3x3Faces::idx(Face::Top, 1, 2),
        Cube3x3x3Faces::idx(Face::Right, 0, 1),
    ],
    // UF
    [
        Cube3x3x3Faces::idx(Face::Top, 2, 1),
        Cube3x3x3Faces::idx(Face::Front, 0, 1),
    ],
    // UL
    [
        Cube3x3x3Faces::idx(Face::Top, 1, 0),
        Cube3x3x3Faces::idx(Face::Left, 0, 1),
    ],
    // UB
    [
        Cube3x3x3Faces::idx(Face::Top, 0, 1),
        Cube3x3x3Faces::idx(Face::Back, 0, 1),
    ],
    // DR
    [
        Cube3x3x3Faces::idx(Face::Bottom, 1, 2),
        Cube3x3x3Faces::idx(Face::Right, 2, 1),
    ],
    // DF
    [
        Cube3x3x3Faces::idx(Face::Bottom, 0, 1),
        Cube3x3x3Faces::idx(Face::Front, 2, 1),
    ],
    // DL
    [
        Cube3x3x3Faces::idx(Face::Bottom, 1, 0),
        Cube3x3x3Faces::idx(Face::Left, 2, 1),
    ],
    // DB
    [
        Cube3x3x3Faces::idx(Face::Bottom, 2, 1),
        Cube3x3x3Faces::idx(Face::Back, 2, 1),
    ],
    // FR
    [
        Cube3x3x3Faces::idx(Face::Front, 1, 2),
        Cube3x3x3Faces::idx(Face::Right, 1, 0),
    ],
    // FL
    [
        Cube3x3x3Faces::idx(Face::Front, 1, 0),
        Cube3x3x3Faces::idx(Face::Left, 1, 2),
    ],
    // BL
    [
        Cube3x3x3Faces::idx(Face::Back, 1, 2),
        Cube3x3x3Faces::idx(Face::Left, 1, 0),
    ],
    // BR
    [
        Cube3x3x3Faces::idx(Face::Back, 1, 0),
        Cube3x3x3Faces::idx(Face::Right, 1, 2),
    ],
];

pub(crate) const CUBE3_CORNER_COLORS: [[Color; 3]; 8] = [
    // URF
    [Color::White, Color::Red, Color::Green],
    // UFL
    [Color::White, Color::Green, Color::Orange],
    // ULB
    [Color::White, Color::Orange, Color::Blue],
    // UBR
    [Color::White, Color::Blue, Color::Red],
    // DFR
    [Color::Yellow, Color::Green, Color::Red],
    // DLF
    [Color::Yellow, Color::Orange, Color::Green],
    // DBL
    [Color::Yellow, Color::Blue, Color::Orange],
    // DRB
    [Color::Yellow, Color::Red, Color::Blue],
];

pub(crate) const CUBE3_EDGE_COLORS: [[Color; 2]; 12] = [
    // UR
    [Color::White, Color::Red],
    // UF
    [Color::White, Color::Green],
    // UL
    [Color::White, Color::Orange],
    // UB
    [Color::White, Color::Blue],
    // DR
    [Color::Yellow, Color::Red],
    // DF
    [Color::Yellow, Color::Green],
    // DL
    [Color::Yellow, Color::Orange],
    // DB
    [Color::Yellow, Color::Blue],
    // FR
    [Color::Green, Color::Red],
    // FL
    [Color::Green, Color::Orange],
    // BL
    [Color::Blue, Color::Orange],
    // BR
    [Color::Blue, Color::Red],
];

// Set of moves possible as the first move in phase 1 (all moves)
#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_1_MOVES: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R,
    Move::Rp,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L,
    Move::Lp,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

// Set of moves that are allowed in phase 1 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_1_FOLLOWUP_MOVES: [&'static [Move]; Move::count_3x3x3()] = [
    // U
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Up
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // U2
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // F
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Fp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // F2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // R
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Rp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // R2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // B
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Bp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // B2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::L,
        Move::Lp,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // L
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Lp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // L2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F,
        Move::Fp,
        Move::F2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // D
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
    ],
    // Dp
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
    ],
    // D2
    &[
        Move::F,
        Move::Fp,
        Move::F2,
        Move::R,
        Move::Rp,
        Move::R2,
        Move::B,
        Move::Bp,
        Move::B2,
        Move::L,
        Move::Lp,
        Move::L2,
    ],
];

// If the last move is not R, R', L, L', F, F', B, or B', the search will be repeated in a
// different phase 2 search. Ignore sequences that fail this check.
#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_1_LAST_MOVES: &'static [Move] = &[
    Move::F,
    Move::Fp,
    Move::R,
    Move::Rp,
    Move::B,
    Move::Bp,
    Move::L,
    Move::Lp,
];

#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_1_LAST_FOLLOWUP_MOVES: [&'static [Move]; Move::count_3x3x3()] = [
    // U
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::Lp,
    ],
    // Up
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::Lp,
    ],
    // U2
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::Lp,
    ],
    // F
    &[Move::R, Move::Rp, Move::B, Move::Bp, Move::L, Move::Lp],
    // Fp
    &[Move::R, Move::Rp, Move::B, Move::Bp, Move::L, Move::Lp],
    // F2
    &[Move::R, Move::Rp, Move::B, Move::Bp, Move::L, Move::Lp],
    // R
    &[Move::F, Move::Fp, Move::B, Move::Bp, Move::L, Move::Lp],
    // Rp
    &[Move::F, Move::Fp, Move::B, Move::Bp, Move::L, Move::Lp],
    // R2
    &[Move::F, Move::Fp, Move::B, Move::Bp, Move::L, Move::Lp],
    // B
    &[Move::R, Move::Rp, Move::L, Move::Lp],
    // Bp
    &[Move::R, Move::Rp, Move::L, Move::Lp],
    // B2
    &[Move::R, Move::Rp, Move::L, Move::Lp],
    // L
    &[Move::F, Move::Fp, Move::B, Move::Bp],
    // Lp
    &[Move::F, Move::Fp, Move::B, Move::Bp],
    // L2
    &[Move::F, Move::Fp, Move::B, Move::Bp],
    // D
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::Lp,
    ],
    // Dp
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::Lp,
    ],
    // D2
    &[
        Move::F,
        Move::Fp,
        Move::R,
        Move::Rp,
        Move::B,
        Move::Bp,
        Move::L,
        Move::L,
    ],
];

// Set of moves possible as the second move in phase 2 (valid for the phase 2 move set U D F2 R2 B2 L2)
#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_2_MOVES: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

// Set of moves that are allowed in phase 2 following each given moves. For example, U should never follow U'.
// Also, avoid move sequences like U D U by forcing opposite faces to be turned only in a single order.
#[cfg(not(feature = "no_solver"))]
pub(crate) const POSSIBLE_PHASE_2_FOLLOWUP_MOVES: [&'static [Move]; Move::count_3x3x3()] = [
    // U
    &[
        Move::F2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Up
    &[
        Move::F2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // U2
    &[
        Move::F2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // F
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Fp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // F2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // R
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Rp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // R2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // B
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Bp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // B2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::R2,
        Move::L2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // L
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // Lp
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // L2
    &[
        Move::U,
        Move::Up,
        Move::U2,
        Move::F2,
        Move::B2,
        Move::D,
        Move::Dp,
        Move::D2,
    ],
    // D
    &[Move::F2, Move::R2, Move::B2, Move::L2],
    // Dp
    &[Move::F2, Move::R2, Move::B2, Move::L2],
    // D2
    &[Move::F2, Move::R2, Move::B2, Move::L2],
];

/// Layout of edges around last layer according to which face is the last layer
pub(crate) const CUBE3_LAST_LAYER_EDGE: [[FaceRowOrColumn; 4]; 6] = [
    [
        FaceRowOrColumn::RowRightToLeft(Face::Back, 0),
        FaceRowOrColumn::RowRightToLeft(Face::Right, 0),
        FaceRowOrColumn::RowRightToLeft(Face::Front, 0),
        FaceRowOrColumn::RowRightToLeft(Face::Left, 0),
    ],
    [
        FaceRowOrColumn::RowLeftToRight(Face::Top, 2),
        FaceRowOrColumn::ColumnTopDown(Face::Right, 0),
        FaceRowOrColumn::RowRightToLeft(Face::Bottom, 0),
        FaceRowOrColumn::ColumnBottomUp(Face::Left, 2),
    ],
    [
        FaceRowOrColumn::ColumnBottomUp(Face::Top, 2),
        FaceRowOrColumn::ColumnTopDown(Face::Back, 0),
        FaceRowOrColumn::ColumnBottomUp(Face::Bottom, 2),
        FaceRowOrColumn::ColumnBottomUp(Face::Front, 2),
    ],
    [
        FaceRowOrColumn::RowRightToLeft(Face::Top, 0),
        FaceRowOrColumn::ColumnTopDown(Face::Left, 0),
        FaceRowOrColumn::RowLeftToRight(Face::Bottom, 2),
        FaceRowOrColumn::ColumnBottomUp(Face::Right, 2),
    ],
    [
        FaceRowOrColumn::ColumnTopDown(Face::Top, 0),
        FaceRowOrColumn::ColumnTopDown(Face::Front, 0),
        FaceRowOrColumn::ColumnTopDown(Face::Bottom, 0),
        FaceRowOrColumn::ColumnBottomUp(Face::Back, 2),
    ],
    [
        FaceRowOrColumn::RowLeftToRight(Face::Front, 2),
        FaceRowOrColumn::RowLeftToRight(Face::Right, 2),
        FaceRowOrColumn::RowLeftToRight(Face::Back, 2),
        FaceRowOrColumn::RowLeftToRight(Face::Left, 2),
    ],
];

/// 3x3x3 OLL cases, each case is represented as a bit vector packed into a u32. The
/// bits represent if the color matches the last layer or not. Order is as follows,
/// looking down on the last layer:
///
///    20 19 18
/// 17 16 15 14 13
/// 12 11 10  9  8
///  7  6  5  4  3
///     2  1  0
///
/// When using this array you must check all four rotations, as only one rotation is
/// of each case is represented here.
pub(crate) const CUBE3_OLL_CASES: [u32; 57] = [
    0b010_10001_10101_10001_010,
    0b111_00000_10101_10001_010,
    0b110_00001_10101_01000_011,
    0b011_10000_10101_00010_110,
    0b000_01101_01101_10000_011,
    0b000_10110_10110_00001_110,
    0b100_00101_01101_01000_011,
    0b001_10100_10110_00010_110,
    0b001_10100_01101_00010_110,
    0b110_00010_01101_10100_001,
    0b100_00110_01101_10000_011,
    0b001_01100_10110_00001_110,
    0b110_00001_01110_01000_011,
    0b011_10000_01110_00010_110,
    0b010_01001_01110_10000_011,
    0b010_10010_01110_00001_110,
    0b010_01001_10101_00010_110,
    0b010_01010_10101_00000_111,
    0b010_01010_10101_10001_010,
    0b010_01010_10101_01010_010,
    0b101_00100_01110_00100_101,
    0b001_10100_01110_10100_001,
    0b101_00100_01110_01110_000,
    0b100_00110_01110_00110_100,
    0b000_10110_01110_01100_001,
    0b000_10110_01110_00101_100,
    0b100_00101_01110_01100_001,
    0b000_01110_01101_01010_010,
    0b100_00110_01101_00010_110,
    0b000_10101_01101_01010_010,
    0b100_00110_10110_00010_110,
    0b001_01100_01101_01000_011,
    0b110_00010_01110_00010_110,
    0b010_10001_01110_01010_010,
    0b010_01001_10110_00110_100,
    0b001_01100_10110_10010_010,
    0b000_01101_01101_00010_110,
    0b100_00110_01101_01001_010,
    0b110_00010_01110_01001_010,
    0b011_01000_01110_10010_010,
    0b101_00100_01101_01010_010,
    0b010_01010_01101_00100_101,
    0b000_10110_10110_10010_010,
    0b000_01101_01101_01001_010,
    0b010_10010_01110_10010_010,
    0b000_01101_10101_01101_000,
    0b100_00101_10110_00001_110,
    0b001_10100_01101_10000_011,
    0b001_10100_10110_10000_011,
    0b011_10000_10110_10100_001,
    0b110_00001_01110_00001_110,
    0b100_00101_10101_00101_100,
    0b101_00100_10110_00000_111,
    0b101_00100_01101_00000_111,
    0b111_00000_01110_00000_111,
    0b010_10001_01110_10001_010,
    0b010_01010_01110_01010_010,
];

/// 3x3x3 PLL cases. Represented with each hex digit as the sorted index of
/// each color in the last layer (clockwise order). The actual colors
/// themselves are not defined here and should be detected, this table only
/// represents the correct grouping of colors for each case.
pub(crate) const CUBE3_PLL_CASES: [[u16; 4]; 21] = [
    [0x045, 0x19b, 0x267, 0x38a],
    [0x05a, 0x126, 0x348, 0x79b],
    [0x057, 0x138, 0x246, 0x9ab],
    [0x05a, 0x138, 0x267, 0x49b],
    [0x057, 0x19b, 0x26a, 0x348],
    [0x057, 0x126, 0x38a, 0x49b],
    [0x045, 0x138, 0x26a, 0x79b],
    [0x045, 0x126, 0x378, 0x9ab],
    [0x015, 0x267, 0x348, 0x9ab],
    [0x05a, 0x19b, 0x246, 0x378],
    [0x015, 0x246, 0x38a, 0x79b],
    [0x015, 0x26a, 0x378, 0x49b],
    [0x08a, 0x159, 0x246, 0x37b],
    [0x018, 0x267, 0x34b, 0x59a],
    [0x078, 0x126, 0x3ab, 0x459],
    [0x078, 0x13b, 0x246, 0x59a],
    [0x078, 0x159, 0x26a, 0x34b],
    [0x027, 0x168, 0x35a, 0x49b],
    [0x012, 0x357, 0x49b, 0x68a],
    [0x012, 0x35a, 0x468, 0x79b],
    [0x024, 0x135, 0x68a, 0x79b],
];
