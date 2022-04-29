use crate::common::CubeFace;
use crate::cube3x3x3::{Cube3x3x3Faces, FaceRowOrColumn};

// Table of the parts of F2L pairs for each possible cross color
pub(crate) const CUBE3_F2L_PAIRS: [[[usize; 5]; 4]; 6] = [
    // Top
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 1, 2),
        ],
    ],
    // Front
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Top, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 2),
        ],
    ],
    // Right
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 1),
        ],
    ],
    // Back
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 1, 2),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Top, 1, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 1, 0),
        ],
    ],
    // Left
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 0, 1),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Top, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Left, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Top, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 0, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 1),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 1),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 1),
        ],
    ],
    // Bottom
    [
        [
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Front, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Bottom, 0, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Front, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Right, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Left, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Left, 1, 0),
        ],
        [
            Cube3x3x3Faces::idx(CubeFace::Bottom, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 2, 2),
            Cube3x3x3Faces::idx(CubeFace::Right, 1, 2),
            Cube3x3x3Faces::idx(CubeFace::Back, 2, 0),
            Cube3x3x3Faces::idx(CubeFace::Back, 1, 0),
        ],
    ],
];

/// Layout of edges around last layer according to which face is the last layer
pub(crate) const CUBE3_LAST_LAYER_EDGE: [[FaceRowOrColumn; 4]; 6] = [
    [
        FaceRowOrColumn::RowRightToLeft(CubeFace::Back, 0),
        FaceRowOrColumn::RowRightToLeft(CubeFace::Right, 0),
        FaceRowOrColumn::RowRightToLeft(CubeFace::Front, 0),
        FaceRowOrColumn::RowRightToLeft(CubeFace::Left, 0),
    ],
    [
        FaceRowOrColumn::RowLeftToRight(CubeFace::Top, 2),
        FaceRowOrColumn::ColumnTopDown(CubeFace::Right, 0),
        FaceRowOrColumn::RowRightToLeft(CubeFace::Bottom, 0),
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Left, 2),
    ],
    [
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Top, 2),
        FaceRowOrColumn::ColumnTopDown(CubeFace::Back, 0),
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Bottom, 2),
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Front, 2),
    ],
    [
        FaceRowOrColumn::RowRightToLeft(CubeFace::Top, 0),
        FaceRowOrColumn::ColumnTopDown(CubeFace::Left, 0),
        FaceRowOrColumn::RowLeftToRight(CubeFace::Bottom, 2),
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Right, 2),
    ],
    [
        FaceRowOrColumn::ColumnTopDown(CubeFace::Top, 0),
        FaceRowOrColumn::ColumnTopDown(CubeFace::Front, 0),
        FaceRowOrColumn::ColumnTopDown(CubeFace::Bottom, 0),
        FaceRowOrColumn::ColumnBottomUp(CubeFace::Back, 2),
    ],
    [
        FaceRowOrColumn::RowLeftToRight(CubeFace::Front, 2),
        FaceRowOrColumn::RowLeftToRight(CubeFace::Right, 2),
        FaceRowOrColumn::RowLeftToRight(CubeFace::Back, 2),
        FaceRowOrColumn::RowLeftToRight(CubeFace::Left, 2),
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
