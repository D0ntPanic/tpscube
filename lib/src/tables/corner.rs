use crate::common::{Color, Corner, CornerPiece, CubeFace};
use crate::cube2x2x2::Cube2x2x2Faces;
use crate::cube3x3x3::Cube3x3x3Faces;
use crate::cube4x4x4::Cube4x4x4Faces;

// Table for rotating the corners in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece goes, where it comes
// from and the adjustment to the orientation (corner twist).
pub(crate) const CUBE_CORNER_PIECE_ROTATION: [[[(Corner, CornerPiece); 4]; 6]; 2] = [
    // CW
    [
        // Top
        [
            (
                Corner::URF,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 0,
                },
            ),
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 0,
                },
            ),
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 0,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Corner::URF,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 1,
                },
            ),
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 2,
                },
            ),
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 2,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::DFR,
                    orientation: 1,
                },
            ),
        ],
        // Right
        [
            (
                Corner::URF,
                CornerPiece {
                    piece: Corner::DFR,
                    orientation: 2,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 1,
                },
            ),
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 1,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 2,
                },
            ),
        ],
        // Back
        [
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 1,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 2,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 2,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 1,
                },
            ),
        ],
        // Left
        [
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 1,
                },
            ),
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 2,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 2,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 1,
                },
            ),
        ],
        // Bottom
        [
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 0,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 0,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 0,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::DFR,
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
                Corner::URF,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 0,
                },
            ),
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 0,
                },
            ),
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 0,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Corner::URF,
                CornerPiece {
                    piece: Corner::DFR,
                    orientation: 1,
                },
            ),
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 2,
                },
            ),
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 2,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 1,
                },
            ),
        ],
        // Right
        [
            (
                Corner::URF,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 2,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 1,
                },
            ),
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::URF,
                    orientation: 1,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::DFR,
                    orientation: 2,
                },
            ),
        ],
        // Back
        [
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 1,
                },
            ),
            (
                Corner::UBR,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 2,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 2,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::UBR,
                    orientation: 1,
                },
            ),
        ],
        // Left
        [
            (
                Corner::UFL,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 1,
                },
            ),
            (
                Corner::ULB,
                CornerPiece {
                    piece: Corner::UFL,
                    orientation: 2,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 2,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::ULB,
                    orientation: 1,
                },
            ),
        ],
        // Bottom
        [
            (
                Corner::DFR,
                CornerPiece {
                    piece: Corner::DRB,
                    orientation: 0,
                },
            ),
            (
                Corner::DLF,
                CornerPiece {
                    piece: Corner::DFR,
                    orientation: 0,
                },
            ),
            (
                Corner::DBL,
                CornerPiece {
                    piece: Corner::DLF,
                    orientation: 0,
                },
            ),
            (
                Corner::DRB,
                CornerPiece {
                    piece: Corner::DBL,
                    orientation: 0,
                },
            ),
        ],
    ],
];

// Table of adjacent faces on corners for cubes in face color format
macro_rules! cube_corner_adjacency {
    ($name: ident, $faces: ident, $n: expr) => {
        pub(crate) const $name: [[[usize; 2]; 4]; 6] = [
            // Top
            [
                [
                    $faces::idx(CubeFace::Left, 0, 0),
                    $faces::idx(CubeFace::Back, 0, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Back, 0, 0),
                    $faces::idx(CubeFace::Right, 0, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Front, 0, 0),
                    $faces::idx(CubeFace::Left, 0, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Right, 0, 0),
                    $faces::idx(CubeFace::Front, 0, $n - 1),
                ],
            ],
            // Front
            [
                [
                    $faces::idx(CubeFace::Left, 0, $n - 1),
                    $faces::idx(CubeFace::Top, $n - 1, 0),
                ],
                [
                    $faces::idx(CubeFace::Top, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Right, 0, 0),
                ],
                [
                    $faces::idx(CubeFace::Bottom, 0, 0),
                    $faces::idx(CubeFace::Left, $n - 1, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Right, $n - 1, 0),
                    $faces::idx(CubeFace::Bottom, 0, $n - 1),
                ],
            ],
            // Right
            [
                [
                    $faces::idx(CubeFace::Front, 0, $n - 1),
                    $faces::idx(CubeFace::Top, $n - 1, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Top, 0, $n - 1),
                    $faces::idx(CubeFace::Back, 0, 0),
                ],
                [
                    $faces::idx(CubeFace::Bottom, 0, $n - 1),
                    $faces::idx(CubeFace::Front, $n - 1, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Back, $n - 1, 0),
                    $faces::idx(CubeFace::Bottom, $n - 1, $n - 1),
                ],
            ],
            // Back
            [
                [
                    $faces::idx(CubeFace::Right, 0, $n - 1),
                    $faces::idx(CubeFace::Top, 0, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Top, 0, 0),
                    $faces::idx(CubeFace::Left, 0, 0),
                ],
                [
                    $faces::idx(CubeFace::Bottom, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Right, $n - 1, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Left, $n - 1, 0),
                    $faces::idx(CubeFace::Bottom, $n - 1, 0),
                ],
            ],
            // Left
            [
                [
                    $faces::idx(CubeFace::Back, 0, $n - 1),
                    $faces::idx(CubeFace::Top, 0, 0),
                ],
                [
                    $faces::idx(CubeFace::Top, $n - 1, 0),
                    $faces::idx(CubeFace::Front, 0, 0),
                ],
                [
                    $faces::idx(CubeFace::Bottom, $n - 1, 0),
                    $faces::idx(CubeFace::Back, $n - 1, $n - 1),
                ],
                [
                    $faces::idx(CubeFace::Front, $n - 1, 0),
                    $faces::idx(CubeFace::Bottom, 0, 0),
                ],
            ],
            // Bottom
            [
                [
                    $faces::idx(CubeFace::Left, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Front, $n - 1, 0),
                ],
                [
                    $faces::idx(CubeFace::Front, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Right, $n - 1, 0),
                ],
                [
                    $faces::idx(CubeFace::Back, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Left, $n - 1, 0),
                ],
                [
                    $faces::idx(CubeFace::Right, $n - 1, $n - 1),
                    $faces::idx(CubeFace::Back, $n - 1, 0),
                ],
            ],
        ];
    };
}

cube_corner_adjacency!(CUBE2_CORNER_ADJACENCY, Cube2x2x2Faces, 2);
cube_corner_adjacency!(CUBE3_CORNER_ADJACENCY, Cube3x3x3Faces, 3);
cube_corner_adjacency!(CUBE4_CORNER_ADJACENCY, Cube4x4x4Faces, 4);

// Table for rotation of corners in face color format. Each entry is the
// index of the corner where the new color comes from. Corners are numbered
// as follows: (0, 0), (0, n-1), (n-1, 0), (n-1, n-1)
pub(crate) const CUBE_CORNER_ROTATION: [[usize; 4]; 2] = [
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
macro_rules! cube_corner_indicies {
    ($name: ident, $faces: ident, $n: expr) => {
        pub(crate) const $name: [[usize; 3]; 8] = [
            // URF
            [
                $faces::idx(CubeFace::Top, $n - 1, $n - 1),
                $faces::idx(CubeFace::Right, 0, 0),
                $faces::idx(CubeFace::Front, 0, $n - 1),
            ],
            // UFL
            [
                $faces::idx(CubeFace::Top, $n - 1, 0),
                $faces::idx(CubeFace::Front, 0, 0),
                $faces::idx(CubeFace::Left, 0, $n - 1),
            ],
            // ULB
            [
                $faces::idx(CubeFace::Top, 0, 0),
                $faces::idx(CubeFace::Left, 0, 0),
                $faces::idx(CubeFace::Back, 0, $n - 1),
            ],
            // UBR
            [
                $faces::idx(CubeFace::Top, 0, $n - 1),
                $faces::idx(CubeFace::Back, 0, 0),
                $faces::idx(CubeFace::Right, 0, $n - 1),
            ],
            // DFR
            [
                $faces::idx(CubeFace::Bottom, 0, $n - 1),
                $faces::idx(CubeFace::Front, $n - 1, $n - 1),
                $faces::idx(CubeFace::Right, $n - 1, 0),
            ],
            // DLF
            [
                $faces::idx(CubeFace::Bottom, 0, 0),
                $faces::idx(CubeFace::Left, $n - 1, $n - 1),
                $faces::idx(CubeFace::Front, $n - 1, 0),
            ],
            // DBL
            [
                $faces::idx(CubeFace::Bottom, $n - 1, 0),
                $faces::idx(CubeFace::Back, $n - 1, $n - 1),
                $faces::idx(CubeFace::Left, $n - 1, 0),
            ],
            // DRB
            [
                $faces::idx(CubeFace::Bottom, $n - 1, $n - 1),
                $faces::idx(CubeFace::Right, $n - 1, $n - 1),
                $faces::idx(CubeFace::Back, $n - 1, 0),
            ],
        ];
    };
}

cube_corner_indicies!(CUBE2_CORNER_INDICIES, Cube2x2x2Faces, 2);
cube_corner_indicies!(CUBE3_CORNER_INDICIES, Cube3x3x3Faces, 3);
cube_corner_indicies!(CUBE4_CORNER_INDICIES, Cube4x4x4Faces, 4);

pub(crate) const CUBE_CORNER_COLORS: [[Color; 3]; 8] = [
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

pub(crate) const CUBE_LAST_LAYER_CORNERS: [[Corner; 4]; 6] = [
    // Top
    [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR],
    // Front
    [Corner::URF, Corner::UFL, Corner::DFR, Corner::DLF],
    // Right
    [Corner::URF, Corner::UBR, Corner::DFR, Corner::DRB],
    // Back
    [Corner::ULB, Corner::UBR, Corner::DBL, Corner::DRB],
    // Left
    [Corner::UFL, Corner::ULB, Corner::DLF, Corner::DBL],
    // Bottom
    [Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
];
