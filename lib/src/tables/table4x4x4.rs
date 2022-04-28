use crate::common::{Color, CubeFace};
use crate::cube4x4x4::{Cube4x4x4, Cube4x4x4Faces, Edge4x4x4, EdgePiece4x4x4};

// Table for rotating the edges in piece format. Rotations are organized by
// the face being rotated. Each entry is where the piece goes, where it comes
// from and the adjustment to the orientation (edge flip).
pub(crate) const CUBE4_EDGE_PIECE_ROTATION: [[[(Edge4x4x4, EdgePiece4x4x4); 8]; 6]; 2] = [
    // CW
    [
        // Top
        [
            (
                Edge4x4x4::URB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::URF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::ULF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::ULB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULB,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Edge4x4x4::UFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFR,
                    orientation: 0,
                },
            ),
        ],
        // Right
        [
            (
                Edge4x4x4::URB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::URF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DRF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DRB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URF,
                    orientation: 1,
                },
            ),
        ],
        // Back
        [
            (
                Edge4x4x4::UBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBR,
                    orientation: 0,
                },
            ),
        ],
        // Left
        [
            (
                Edge4x4x4::ULF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::ULB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DLB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DLF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLF,
                    orientation: 1,
                },
            ),
        ],
        // Bottom
        [
            (
                Edge4x4x4::DRF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DRB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DLB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DLF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRB,
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
                Edge4x4x4::URB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::URF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::ULF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::ULB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URF,
                    orientation: 0,
                },
            ),
        ],
        // Front
        [
            (
                Edge4x4x4::UFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UFL,
                    orientation: 0,
                },
            ),
        ],
        // Right
        [
            (
                Edge4x4x4::URB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::URF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DRF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DRB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RFD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::URB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::RBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRB,
                    orientation: 1,
                },
            ),
        ],
        // Back
        [
            (
                Edge4x4x4::UBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::UBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBU,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::RBD,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::LBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::RBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::UBL,
                    orientation: 0,
                },
            ),
        ],
        // Left
        [
            (
                Edge4x4x4::ULF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::ULB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LFU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DLB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBU,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::DLF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::LBD,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LFU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LFD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLB,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LBU,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULF,
                    orientation: 1,
                },
            ),
            (
                Edge4x4x4::LBD,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::ULB,
                    orientation: 1,
                },
            ),
        ],
        // Bottom
        [
            (
                Edge4x4x4::DRF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DRB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DBL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRF,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DFR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DRB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DLB,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFL,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DLF,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DFR,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBR,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLB,
                    orientation: 0,
                },
            ),
            (
                Edge4x4x4::DBL,
                EdgePiece4x4x4 {
                    piece: Edge4x4x4::DLF,
                    orientation: 0,
                },
            ),
        ],
    ],
];

// Table for rotating the inner edge slices in piece format. Rotations are organized
// by the face being rotated. The entries are the pieces in clockwise order.
pub(crate) const CUBE4_SLICED_EDGE_PIECE_ROTATION: [[Edge4x4x4; 4]; 6] = [
    // Top
    [
        Edge4x4x4::LBU,
        Edge4x4x4::RBU,
        Edge4x4x4::RFU,
        Edge4x4x4::LFU,
    ],
    // Front
    [
        Edge4x4x4::ULF,
        Edge4x4x4::URF,
        Edge4x4x4::DRF,
        Edge4x4x4::DLF,
    ],
    // Right
    [
        Edge4x4x4::UFR,
        Edge4x4x4::UBR,
        Edge4x4x4::DBR,
        Edge4x4x4::DFR,
    ],
    // Back
    [
        Edge4x4x4::URB,
        Edge4x4x4::ULB,
        Edge4x4x4::DLB,
        Edge4x4x4::DRB,
    ],
    // Left
    [
        Edge4x4x4::UBL,
        Edge4x4x4::UFL,
        Edge4x4x4::DFL,
        Edge4x4x4::DBL,
    ],
    // Bottom
    [
        Edge4x4x4::LFD,
        Edge4x4x4::RFD,
        Edge4x4x4::RBD,
        Edge4x4x4::LBD,
    ],
];

// Table for rotating the inner slice center pieces. Rotations are organized
// by the face being rotated. The entries are the pieces in clockwise order.
// This table provides the piece indexes in piece format.
pub(crate) const CUBE4_SLICED_CENTER_PIECE_ROTATION: [[usize; 8]; 6] = [
    // Top
    [
        Cube4x4x4::center_idx(CubeFace::Back, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Back, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Right, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Right, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Front, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Front, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Left, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Left, 0, 0),
    ],
    // Front
    [
        Cube4x4x4::center_idx(CubeFace::Top, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Top, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Right, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Right, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Bottom, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Left, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Left, 0, 1),
    ],
    // Right
    [
        Cube4x4x4::center_idx(CubeFace::Top, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Top, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Back, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Back, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Bottom, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Front, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Front, 0, 1),
    ],
    // Back
    [
        Cube4x4x4::center_idx(CubeFace::Top, 0, 1),
        Cube4x4x4::center_idx(CubeFace::Top, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Left, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Left, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Right, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Right, 0, 1),
    ],
    // Left
    [
        Cube4x4x4::center_idx(CubeFace::Top, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Top, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Front, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Front, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 0, 0),
        Cube4x4x4::center_idx(CubeFace::Bottom, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Back, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Back, 0, 1),
    ],
    // Bottom
    [
        Cube4x4x4::center_idx(CubeFace::Front, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Front, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Right, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Right, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Back, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Back, 1, 1),
        Cube4x4x4::center_idx(CubeFace::Left, 1, 0),
        Cube4x4x4::center_idx(CubeFace::Left, 1, 1),
    ],
];

// Table for rotating the inner slice center pieces. Rotations are organized
// by the face being rotated. The entries are the pieces in clockwise order.
// This table provides the piece indexes in face format.
pub(crate) const CUBE4_SLICED_CENTER_FACE_ROTATION: [[usize; 8]; 6] = [
    // Top
    [
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 1),
    ],
    // Front
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 2),
    ],
    // Right
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 2),
    ],
    // Back
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 2),
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 2),
    ],
    // Left
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 2),
    ],
    // Bottom
    [
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 2),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 2),
    ],
];

// Table for rotation of centers. Centers are numbered from top left to bottom right
// in row major order. Each entry is the index of the center where the new color
// comes from.
pub(crate) const CUBE4_CENTER_PIECE_ROTATION: [[usize; 4]; 2] = [
    // CW
    [2, 0, 3, 1],
    // CCW
    [1, 3, 0, 2],
];

// Table of adjacent faces on edges for cubes in face color format
pub(crate) const CUBE4_EDGE_ADJACENCY: [[usize; 8]; 6] = [
    // Top
    [
        Cube4x4x4Faces::idx(CubeFace::Back, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Right, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Left, 0, 1),
    ],
    // Front
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Top, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 3),
    ],
    // Right
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 3),
    ],
    // Back
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Top, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 3),
    ],
    // Left
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 3),
    ],
    // Bottom
    [
        Cube4x4x4Faces::idx(CubeFace::Front, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Right, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Right, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Left, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Left, 3, 2),
    ],
];

// Table for rotation of a face in face color format. Each entry is the
// index on a face where the new color comes from.
pub(crate) const CUBE4_FACE_ROTATION: [[usize; 16]; 2] = [
    // CW
    [
        Cube4x4x4Faces::face_offset(3, 0),
        Cube4x4x4Faces::face_offset(2, 0),
        Cube4x4x4Faces::face_offset(1, 0),
        Cube4x4x4Faces::face_offset(0, 0),
        Cube4x4x4Faces::face_offset(3, 1),
        Cube4x4x4Faces::face_offset(2, 1),
        Cube4x4x4Faces::face_offset(1, 1),
        Cube4x4x4Faces::face_offset(0, 1),
        Cube4x4x4Faces::face_offset(3, 2),
        Cube4x4x4Faces::face_offset(2, 2),
        Cube4x4x4Faces::face_offset(1, 2),
        Cube4x4x4Faces::face_offset(0, 2),
        Cube4x4x4Faces::face_offset(3, 3),
        Cube4x4x4Faces::face_offset(2, 3),
        Cube4x4x4Faces::face_offset(1, 3),
        Cube4x4x4Faces::face_offset(0, 3),
    ],
    // CCW
    [
        Cube4x4x4Faces::face_offset(0, 3),
        Cube4x4x4Faces::face_offset(1, 3),
        Cube4x4x4Faces::face_offset(2, 3),
        Cube4x4x4Faces::face_offset(3, 3),
        Cube4x4x4Faces::face_offset(0, 2),
        Cube4x4x4Faces::face_offset(1, 2),
        Cube4x4x4Faces::face_offset(2, 2),
        Cube4x4x4Faces::face_offset(3, 2),
        Cube4x4x4Faces::face_offset(0, 1),
        Cube4x4x4Faces::face_offset(1, 1),
        Cube4x4x4Faces::face_offset(2, 1),
        Cube4x4x4Faces::face_offset(3, 1),
        Cube4x4x4Faces::face_offset(0, 0),
        Cube4x4x4Faces::face_offset(1, 0),
        Cube4x4x4Faces::face_offset(2, 0),
        Cube4x4x4Faces::face_offset(3, 0),
    ],
];

// Table for rotation of edges in face color format. Each entry is the
// index of the edge where the new color comes from. Edges are numbered
// from top left clockwise.
pub(crate) const CUBE4_EDGE_ROTATION: [[usize; 8]; 2] = [
    // CW
    [2, 3, 4, 5, 6, 7, 0, 1],
    // CCW
    [6, 7, 0, 1, 2, 3, 4, 5],
];

// Table for converting piece format to face color format. First level of
// the array is the edge index in piece format, and the second level of
// the array is for each of the 2 faces on an edge.
pub(crate) const CUBE4_EDGE_INDICIES: [[usize; 2]; 24] = [
    // URB
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Right, 0, 2),
    ],
    // URF
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Right, 0, 1),
    ],
    // UFR
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 0, 2),
    ],
    // UFL
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 0, 1),
    ],
    // ULF
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Left, 0, 2),
    ],
    // ULB
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Left, 0, 1),
    ],
    // UBL
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 0, 2),
    ],
    // UBR
    [
        Cube4x4x4Faces::idx(CubeFace::Top, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 0, 1),
    ],
    // DRF
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Right, 3, 1),
    ],
    // DRB
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Right, 3, 2),
    ],
    // DFL
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 0, 1),
        Cube4x4x4Faces::idx(CubeFace::Front, 3, 1),
    ],
    // DFR
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 0, 2),
        Cube4x4x4Faces::idx(CubeFace::Front, 3, 2),
    ],
    // DLB
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Left, 3, 1),
    ],
    // DLF
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Left, 3, 2),
    ],
    // DBR
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 3, 2),
        Cube4x4x4Faces::idx(CubeFace::Back, 3, 1),
    ],
    // DBL
    [
        Cube4x4x4Faces::idx(CubeFace::Bottom, 3, 1),
        Cube4x4x4Faces::idx(CubeFace::Back, 3, 2),
    ],
    // RFD
    [
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 3),
    ],
    // RFU
    [
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 3),
    ],
    // LFU
    [
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Front, 1, 0),
    ],
    // LFD
    [
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Front, 2, 0),
    ],
    // LBD
    [
        Cube4x4x4Faces::idx(CubeFace::Left, 2, 0),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 3),
    ],
    // LBU
    [
        Cube4x4x4Faces::idx(CubeFace::Left, 1, 0),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 3),
    ],
    // RBU
    [
        Cube4x4x4Faces::idx(CubeFace::Right, 1, 3),
        Cube4x4x4Faces::idx(CubeFace::Back, 1, 0),
    ],
    // RBD
    [
        Cube4x4x4Faces::idx(CubeFace::Right, 2, 3),
        Cube4x4x4Faces::idx(CubeFace::Back, 2, 0),
    ],
];

pub(crate) const CUBE4_EDGE_COLORS: [[Color; 2]; 12] = [
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
    // RF
    [Color::Red, Color::Green],
    // LF
    [Color::Orange, Color::Green],
    // LB
    [Color::Orange, Color::Blue],
    // RB
    [Color::Red, Color::Blue],
];
