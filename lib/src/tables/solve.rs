use crate::common::Move;

pub(crate) const CUBE_CORNER_ORIENTATION_MOVE_TABLE: &'static [u8] =
    include_bytes!("corner_orientation_move_table.bin");
pub(crate) const CUBE_CORNER_PERMUTATION_MOVE_TABLE: &'static [u8] =
    include_bytes!("corner_permutation_move_table.bin");
pub(crate) const CUBE3_EDGE_ORIENTATION_MOVE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_edge_orientation_move_table.bin");
pub(crate) const CUBE3_EQUATORIAL_EDGE_SLICE_MOVE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_equatorial_edge_slice_move_table.bin");
pub(crate) const CUBE3_PHASE_2_EDGE_PERMUTATION_MOVE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_phase_2_edge_permutation_move_table.bin");
pub(crate) const CUBE3_PHASE_2_EQUATORIAL_EDGE_PERMUTATION_MOVE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_phase_2_equatorial_edge_permutation_move_table.bin");
pub(crate) const CUBE_CORNER_ORIENTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("corner_orientation_prune_table.bin");
pub(crate) const CUBE_CORNER_PERMUTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("corner_permutation_prune_table.bin");
pub(crate) const CUBE3_CORNER_ORIENTATION_EDGE_SLICE_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_corner_orientation_edge_slice_prune_table.bin");
pub(crate) const CUBE3_EDGE_ORIENTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_edge_orientation_prune_table.bin");
pub(crate) const CUBE3_COMBINED_ORIENTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_combined_orientation_prune_table.bin");
pub(crate) const CUBE3_CORNER_EDGE_PERMUTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_corner_edge_permutation_prune_table.bin");
pub(crate) const CUBE3_PHASE_1_CORNER_PERMUTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_phase_1_corner_permutation_prune_table.bin");
pub(crate) const CUBE3_PHASE_2_EDGE_PERMUTATION_PRUNE_TABLE: &'static [u8] =
    include_bytes!("3x3x3_phase_2_edge_permutation_prune_table.bin");

pub(crate) const CUBE2_POSSIBLE_MOVES: &'static [Move] = CUBE3_POSSIBLE_PHASE_1_MOVES;
pub(crate) const CUBE2_POSSIBLE_FOLLOWUP_MOVES: [&'static [Move]; Move::count_2x2x2()] =
    CUBE3_POSSIBLE_PHASE_1_FOLLOWUP_MOVES;

// Set of moves possible as the first move in phase 1 (all moves)
pub(crate) const CUBE3_POSSIBLE_PHASE_1_MOVES: &'static [Move] = &[
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
pub(crate) const CUBE3_POSSIBLE_PHASE_1_FOLLOWUP_MOVES: [&'static [Move]; Move::count_3x3x3()] = [
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
pub(crate) const CUBE3_POSSIBLE_PHASE_1_LAST_MOVES: &'static [Move] = &[
    Move::F,
    Move::Fp,
    Move::R,
    Move::Rp,
    Move::B,
    Move::Bp,
    Move::L,
    Move::Lp,
];

pub(crate) const CUBE3_POSSIBLE_PHASE_1_LAST_FOLLOWUP_MOVES: [&'static [Move];
    Move::count_3x3x3()] = [
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
pub(crate) const CUBE3_POSSIBLE_PHASE_2_MOVES: &'static [Move] = &[
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
pub(crate) const CUBE3_POSSIBLE_PHASE_2_FOLLOWUP_MOVES: [&'static [Move]; Move::count_3x3x3()] = [
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
