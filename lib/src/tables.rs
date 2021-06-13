use crate::common::{Color, Face};
use crate::cube3x3x3::{Corner3x3x3, CornerPiece3x3x3, Cube3x3x3Faces, Edge3x3x3, EdgePiece3x3x3};

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
