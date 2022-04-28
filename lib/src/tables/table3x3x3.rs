use crate::common::{Color, CubeFace};
use crate::cube3x3x3::{Cube3x3x3Faces, Edge3x3x3, EdgePiece3x3x3};

// Table for rotating the edges in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece goes, where it comes
// from and the adjustment to the orientation (edge flip).
pub(crate) const CUBE3_EDGE_PIECE_ROTATION: [[[(Edge3x3x3, EdgePiece3x3x3); 4]; 6]; 2] = [
    // CW
    [
        // Top
        [
            (
                Edge3x3x3::UR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UB,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UF,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UL,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Edge3x3x3::UF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FL,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::DF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FR,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::FR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UF,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::FL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DF,
                    orientation: 1,
                },
            ),
        ],
        // Right
        [
            (
                Edge3x3x3::UR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::FR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::BR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UR,
                    orientation: 0,
                },
            ),
        ],
        // Back
        [
            (
                Edge3x3x3::UB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BR,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::DB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BL,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::BL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UB,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::BR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DB,
                    orientation: 1,
                },
            ),
        ],
        // Left
        [
            (
                Edge3x3x3::UL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::FL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::BL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DL,
                    orientation: 0,
                },
            ),
        ],
        // Bottom
        [
            (
                Edge3x3x3::DR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DF,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DB,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DR,
                    orientation: 0,
                },
            ),
        ],
    ],
    // CCW
    [
        // Top
        [
            (
                Edge3x3x3::UR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UF,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UB,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::UB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UR,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Edge3x3x3::UF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FR,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::DF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FL,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::FR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DF,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::FL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UF,
                    orientation: 1,
                },
            ),
        ],
        // Right
        [
            (
                Edge3x3x3::UR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::FR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::BR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DR,
                    orientation: 0,
                },
            ),
        ],
        // Back
        [
            (
                Edge3x3x3::UB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BL,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::DB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BR,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::BL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DB,
                    orientation: 1,
                },
            ),
            (
                Edge3x3x3::BR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UB,
                    orientation: 1,
                },
            ),
        ],
        // Left
        [
            (
                Edge3x3x3::UL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::FL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::BL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::FL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DL,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::BL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::UL,
                    orientation: 0,
                },
            ),
        ],
        // Bottom
        [
            (
                Edge3x3x3::DR,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DB,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DF,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DR,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DL,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DF,
                    orientation: 0,
                },
            ),
            (
                Edge3x3x3::DB,
                EdgePiece3x3x3 {
                    piece: Edge3x3x3::DL,
                    orientation: 0,
                },
            ),
        ],
    ],
];

// Table of adjacent faces on edges for cubes in face color format
pub(crate) const CUBE3_EDGE_ADJACENCY: [[usize; 4]; 6] = [
    // Top
    [
        Cube3x3x3Faces::idx(CubeFace::Back, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Left, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Right, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Front, 0, 1),
    ],
    // Front
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Left, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Right, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 1),
    ],
    // Right
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Front, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Back, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 2),
    ],
    // Back
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Right, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Left, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 1),
    ],
    // Left
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Back, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Front, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 0),
    ],
    // Bottom
    [
        Cube3x3x3Faces::idx(CubeFace::Front, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Left, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Right, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Back, 2, 1),
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

// Table for converting piece format to face color format. First level of
// the array is the edge index in piece format, and the second level of
// the array is for each of the 2 faces on an edge.
pub(crate) const CUBE3_EDGE_INDICIES: [[usize; 2]; 12] = [
    // UR
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Right, 0, 1),
    ],
    // UF
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Front, 0, 1),
    ],
    // UL
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Left, 0, 1),
    ],
    // UB
    [
        Cube3x3x3Faces::idx(CubeFace::Top, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Back, 0, 1),
    ],
    // DR
    [
        Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Right, 2, 1),
    ],
    // DF
    [
        Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 1),
        Cube3x3x3Faces::idx(CubeFace::Front, 2, 1),
    ],
    // DL
    [
        Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Left, 2, 1),
    ],
    // DB
    [
        Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 1),
        Cube3x3x3Faces::idx(CubeFace::Back, 2, 1),
    ],
    // FR
    [
        Cube3x3x3Faces::idx(CubeFace::Front, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Right, 1, 0),
    ],
    // FL
    [
        Cube3x3x3Faces::idx(CubeFace::Front, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Left, 1, 2),
    ],
    // BL
    [
        Cube3x3x3Faces::idx(CubeFace::Back, 1, 2),
        Cube3x3x3Faces::idx(CubeFace::Left, 1, 0),
    ],
    // BR
    [
        Cube3x3x3Faces::idx(CubeFace::Back, 1, 0),
        Cube3x3x3Faces::idx(CubeFace::Right, 1, 2),
    ],
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
