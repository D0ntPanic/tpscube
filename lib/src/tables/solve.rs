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

pub(crate) const CUBE4_PHASE_1_RED_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_1_red_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_1_ORANGE_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_1_orange_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_2_3_RED_ORANGE_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_2_red_orange_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_2_GREEN_BLUE_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_2_green_blue_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_3_GREEN_BLUE_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_3_green_blue_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_4_CENTERS_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_4_centers_move_table.bin");
pub(crate) const CUBE4_PHASE_4_EDGE_PAIR_MOVE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_4_edge_pair_move_table.bin");
pub(crate) const CUBE4_PHASE_1_RED_CENTERS_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_1_red_centers_prune_table.bin");
pub(crate) const CUBE4_PHASE_1_ORANGE_CENTERS_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_1_orange_centers_prune_table.bin");
pub(crate) const CUBE4_PHASE_2_CENTERS_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_2_centers_prune_table.bin");
pub(crate) const CUBE4_PHASE_3_CENTERS_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_3_centers_prune_table.bin");
pub(crate) const CUBE4_PHASE_3_LOW_EDGE_PAIR_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_3_low_edge_pair_prune_table.bin");
pub(crate) const CUBE4_PHASE_3_HIGH_EDGE_PAIR_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_3_high_edge_pair_prune_table.bin");
pub(crate) const CUBE4_PHASE_4_CENTERS_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_4_centers_prune_table.bin");
pub(crate) const CUBE4_PHASE_4_EDGE_PAIR_PRUNE_TABLE: &'static [u8] =
    include_bytes!("4x4x4_phase_4_edge_pair_prune_table.bin");

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
    CUBE3_PHASE_1_U_FOLLOWUP,
    // Up
    CUBE3_PHASE_1_U_FOLLOWUP,
    // U2
    CUBE3_PHASE_1_U_FOLLOWUP,
    // F
    CUBE3_PHASE_1_F_FOLLOWUP,
    // Fp
    CUBE3_PHASE_1_F_FOLLOWUP,
    // F2
    CUBE3_PHASE_1_F_FOLLOWUP,
    // R
    CUBE3_PHASE_1_R_FOLLOWUP,
    // Rp
    CUBE3_PHASE_1_R_FOLLOWUP,
    // R2
    CUBE3_PHASE_1_R_FOLLOWUP,
    // B
    CUBE3_PHASE_1_B_FOLLOWUP,
    // Bp
    CUBE3_PHASE_1_B_FOLLOWUP,
    // B2
    CUBE3_PHASE_1_B_FOLLOWUP,
    // L
    CUBE3_PHASE_1_L_FOLLOWUP,
    // Lp
    CUBE3_PHASE_1_L_FOLLOWUP,
    // L2
    CUBE3_PHASE_1_L_FOLLOWUP,
    // D
    CUBE3_PHASE_1_D_FOLLOWUP,
    // Dp
    CUBE3_PHASE_1_D_FOLLOWUP,
    // D2
    CUBE3_PHASE_1_D_FOLLOWUP,
];

const CUBE3_PHASE_1_U_FOLLOWUP: &'static [Move] = &[
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

const CUBE3_PHASE_1_F_FOLLOWUP: &'static [Move] = &[
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
];

const CUBE3_PHASE_1_R_FOLLOWUP: &'static [Move] = &[
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
];

const CUBE3_PHASE_1_B_FOLLOWUP: &'static [Move] = &[
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
];

const CUBE3_PHASE_1_L_FOLLOWUP: &'static [Move] = &[
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
];

const CUBE3_PHASE_1_D_FOLLOWUP: &'static [Move] = &[
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

const CUBE3_PHASE_1_U_LAST_FOLLOWUP: &'static [Move] = &[
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
    CUBE3_PHASE_1_U_LAST_FOLLOWUP,
    // Up
    CUBE3_PHASE_1_U_LAST_FOLLOWUP,
    // U2
    CUBE3_PHASE_1_U_LAST_FOLLOWUP,
    // F
    CUBE3_PHASE_1_F_LAST_FOLLOWUP,
    // Fp
    CUBE3_PHASE_1_F_LAST_FOLLOWUP,
    // F2
    CUBE3_PHASE_1_F_LAST_FOLLOWUP,
    // R
    CUBE3_PHASE_1_R_LAST_FOLLOWUP,
    // Rp
    CUBE3_PHASE_1_R_LAST_FOLLOWUP,
    // R2
    CUBE3_PHASE_1_R_LAST_FOLLOWUP,
    // B
    CUBE3_PHASE_1_B_LAST_FOLLOWUP,
    // Bp
    CUBE3_PHASE_1_B_LAST_FOLLOWUP,
    // B2
    CUBE3_PHASE_1_B_LAST_FOLLOWUP,
    // L
    CUBE3_PHASE_1_L_LAST_FOLLOWUP,
    // Lp
    CUBE3_PHASE_1_L_LAST_FOLLOWUP,
    // L2
    CUBE3_PHASE_1_L_LAST_FOLLOWUP,
    // D
    CUBE3_PHASE_1_D_LAST_FOLLOWUP,
    // Dp
    CUBE3_PHASE_1_D_LAST_FOLLOWUP,
    // D2
    CUBE3_PHASE_1_D_LAST_FOLLOWUP,
];

const CUBE3_PHASE_1_F_LAST_FOLLOWUP: &'static [Move] =
    &[Move::R, Move::Rp, Move::B, Move::Bp, Move::L, Move::Lp];

const CUBE3_PHASE_1_R_LAST_FOLLOWUP: &'static [Move] =
    &[Move::F, Move::Fp, Move::B, Move::Bp, Move::L, Move::Lp];

const CUBE3_PHASE_1_B_LAST_FOLLOWUP: &'static [Move] = &[Move::R, Move::Rp, Move::L, Move::Lp];
const CUBE3_PHASE_1_L_LAST_FOLLOWUP: &'static [Move] = &[Move::F, Move::Fp, Move::B, Move::Bp];

const CUBE3_PHASE_1_D_LAST_FOLLOWUP: &'static [Move] = &[
    Move::F,
    Move::Fp,
    Move::R,
    Move::Rp,
    Move::B,
    Move::Bp,
    Move::L,
    Move::Lp,
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
    CUBE3_PHASE_2_U_FOLLOWUP,
    // Up
    CUBE3_PHASE_2_U_FOLLOWUP,
    // U2
    CUBE3_PHASE_2_U_FOLLOWUP,
    // F
    CUBE3_PHASE_2_F_FOLLOWUP,
    // Fp
    CUBE3_PHASE_2_F_FOLLOWUP,
    // F2
    CUBE3_PHASE_2_F_FOLLOWUP,
    // R
    CUBE3_PHASE_2_R_FOLLOWUP,
    // Rp
    CUBE3_PHASE_2_R_FOLLOWUP,
    // R2
    CUBE3_PHASE_2_R_FOLLOWUP,
    // B
    CUBE3_PHASE_2_B_FOLLOWUP,
    // Bp
    CUBE3_PHASE_2_B_FOLLOWUP,
    // B2
    CUBE3_PHASE_2_B_FOLLOWUP,
    // L
    CUBE3_PHASE_2_L_FOLLOWUP,
    // Lp
    CUBE3_PHASE_2_L_FOLLOWUP,
    // L2
    CUBE3_PHASE_2_L_FOLLOWUP,
    // D
    CUBE3_PHASE_2_D_FOLLOWUP,
    // Dp
    CUBE3_PHASE_2_D_FOLLOWUP,
    // D2
    CUBE3_PHASE_2_D_FOLLOWUP,
];

const CUBE3_PHASE_2_U_FOLLOWUP: &'static [Move] = &[
    Move::F2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

const CUBE3_PHASE_2_F_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

const CUBE3_PHASE_2_R_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

const CUBE3_PHASE_2_B_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
];

const CUBE3_PHASE_2_L_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F2,
    Move::B2,
    Move::D,
    Move::Dp,
    Move::D2,
];

const CUBE3_PHASE_2_D_FOLLOWUP: &'static [Move] = &[Move::F2, Move::R2, Move::B2, Move::L2];

// Set of moves possible as the first move in phase 1 (all moves)
pub(crate) const CUBE4_POSSIBLE_PHASE_1_MOVES: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

// Set of moves that are allowed in phase 1 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
pub(crate) const CUBE4_POSSIBLE_PHASE_1_FOLLOWUP_MOVES: [&'static [Move]; Move::count_4x4x4()] = [
    // U
    CUBE4_PHASE_1_U_FOLLOWUP,
    // Up
    CUBE4_PHASE_1_U_FOLLOWUP,
    // U2
    CUBE4_PHASE_1_U_FOLLOWUP,
    // F
    CUBE4_PHASE_1_F_FOLLOWUP,
    // Fp
    CUBE4_PHASE_1_F_FOLLOWUP,
    // F2
    CUBE4_PHASE_1_F_FOLLOWUP,
    // R
    CUBE4_PHASE_1_R_FOLLOWUP,
    // Rp
    CUBE4_PHASE_1_R_FOLLOWUP,
    // R2
    CUBE4_PHASE_1_R_FOLLOWUP,
    // B
    CUBE4_PHASE_1_B_FOLLOWUP,
    // Bp
    CUBE4_PHASE_1_B_FOLLOWUP,
    // B2
    CUBE4_PHASE_1_B_FOLLOWUP,
    // L
    CUBE4_PHASE_1_L_FOLLOWUP,
    // Lp
    CUBE4_PHASE_1_L_FOLLOWUP,
    // L2
    CUBE4_PHASE_1_L_FOLLOWUP,
    // D
    CUBE4_PHASE_1_D_FOLLOWUP,
    // Dp
    CUBE4_PHASE_1_D_FOLLOWUP,
    // D2
    CUBE4_PHASE_1_D_FOLLOWUP,
    // Uw
    CUBE4_PHASE_1_UW_FOLLOWUP,
    // Uwp
    CUBE4_PHASE_1_UW_FOLLOWUP,
    // Uw2
    CUBE4_PHASE_1_UW_FOLLOWUP,
    // Fw
    CUBE4_PHASE_1_FW_FOLLOWUP,
    // Fwp
    CUBE4_PHASE_1_FW_FOLLOWUP,
    // Fw2
    CUBE4_PHASE_1_FW_FOLLOWUP,
    // Rw
    CUBE4_PHASE_1_RW_FOLLOWUP,
    // Rwp
    CUBE4_PHASE_1_RW_FOLLOWUP,
    // Rw2
    CUBE4_PHASE_1_RW_FOLLOWUP,
    // Bw
    CUBE4_PHASE_1_BW_FOLLOWUP,
    // Bwp
    CUBE4_PHASE_1_BW_FOLLOWUP,
    // Bw2
    CUBE4_PHASE_1_BW_FOLLOWUP,
    // Lw
    CUBE4_PHASE_1_LW_FOLLOWUP,
    // Lwp
    CUBE4_PHASE_1_LW_FOLLOWUP,
    // Lw2
    CUBE4_PHASE_1_LW_FOLLOWUP,
    // Dw
    CUBE4_PHASE_1_DW_FOLLOWUP,
    // Dwp
    CUBE4_PHASE_1_DW_FOLLOWUP,
    // Dw2
    CUBE4_PHASE_1_DW_FOLLOWUP,
];

const CUBE4_PHASE_1_U_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_F_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_R_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_B_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_L_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_D_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_UW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_FW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_RW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_BW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_LW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw,
    Move::Uwp,
    Move::Uw2,
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Dw,
    Move::Dwp,
    Move::Dw2,
];

const CUBE4_PHASE_1_DW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw,
    Move::Fwp,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw,
    Move::Bwp,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
];

// Set of moves possible as the first move in phase 2 (R L F B U D Rw Lw Fw2 Bw2 Uw2 Dw2)
pub(crate) const CUBE4_POSSIBLE_PHASE_2_MOVES: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

// Set of moves that are allowed in phase 2 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
pub(crate) const CUBE4_POSSIBLE_PHASE_2_FOLLOWUP_MOVES: [&'static [Move]; Move::count_4x4x4()] = [
    // U
    CUBE4_PHASE_2_U_FOLLOWUP,
    // Up
    CUBE4_PHASE_2_U_FOLLOWUP,
    // U2
    CUBE4_PHASE_2_U_FOLLOWUP,
    // F
    CUBE4_PHASE_2_F_FOLLOWUP,
    // Fp
    CUBE4_PHASE_2_F_FOLLOWUP,
    // F2
    CUBE4_PHASE_2_F_FOLLOWUP,
    // R
    CUBE4_PHASE_2_R_FOLLOWUP,
    // Rp
    CUBE4_PHASE_2_R_FOLLOWUP,
    // R2
    CUBE4_PHASE_2_R_FOLLOWUP,
    // B
    CUBE4_PHASE_2_B_FOLLOWUP,
    // Bp
    CUBE4_PHASE_2_B_FOLLOWUP,
    // B2
    CUBE4_PHASE_2_B_FOLLOWUP,
    // L
    CUBE4_PHASE_2_L_FOLLOWUP,
    // Lp
    CUBE4_PHASE_2_L_FOLLOWUP,
    // L2
    CUBE4_PHASE_2_L_FOLLOWUP,
    // D
    CUBE4_PHASE_2_D_FOLLOWUP,
    // Dp
    CUBE4_PHASE_2_D_FOLLOWUP,
    // D2
    CUBE4_PHASE_2_D_FOLLOWUP,
    // Uw
    CUBE4_PHASE_2_UW_FOLLOWUP,
    // Uwp
    CUBE4_PHASE_2_UW_FOLLOWUP,
    // Uw2
    CUBE4_PHASE_2_UW_FOLLOWUP,
    // Fw
    CUBE4_PHASE_2_FW_FOLLOWUP,
    // Fwp
    CUBE4_PHASE_2_FW_FOLLOWUP,
    // Fw2
    CUBE4_PHASE_2_FW_FOLLOWUP,
    // Rw
    CUBE4_PHASE_2_RW_FOLLOWUP,
    // Rwp
    CUBE4_PHASE_2_RW_FOLLOWUP,
    // Rw2
    CUBE4_PHASE_2_RW_FOLLOWUP,
    // Bw
    CUBE4_PHASE_2_BW_FOLLOWUP,
    // Bwp
    CUBE4_PHASE_2_BW_FOLLOWUP,
    // Bw2
    CUBE4_PHASE_2_BW_FOLLOWUP,
    // Lw
    CUBE4_PHASE_2_LW_FOLLOWUP,
    // Lwp
    CUBE4_PHASE_2_LW_FOLLOWUP,
    // Lw2
    CUBE4_PHASE_2_LW_FOLLOWUP,
    // Dw
    CUBE4_PHASE_2_DW_FOLLOWUP,
    // Dwp
    CUBE4_PHASE_2_DW_FOLLOWUP,
    // Dw2
    CUBE4_PHASE_2_DW_FOLLOWUP,
];

const CUBE4_PHASE_2_U_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_F_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_R_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_B_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_L_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_D_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_UW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_FW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_RW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_BW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_LW_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Bw2,
    Move::Dw2,
];

const CUBE4_PHASE_2_DW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Rw,
    Move::Rwp,
    Move::Rw2,
    Move::Bw2,
    Move::Lw,
    Move::Lwp,
    Move::Lw2,
];

// Set of moves possible as the first move in phase 3 (R2 L2 F B U D Rw2 Lw2 Fw2 Bw2 Uw2 Dw2)
pub(crate) const CUBE4_POSSIBLE_PHASE_3_MOVES: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

// Set of moves that are allowed in phase 3 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
pub(crate) const CUBE4_POSSIBLE_PHASE_3_FOLLOWUP_MOVES: [&'static [Move]; Move::count_4x4x4()] = [
    // U
    CUBE4_PHASE_3_U_FOLLOWUP,
    // Up
    CUBE4_PHASE_3_U_FOLLOWUP,
    // U2
    CUBE4_PHASE_3_U_FOLLOWUP,
    // F
    CUBE4_PHASE_3_F_FOLLOWUP,
    // Fp
    CUBE4_PHASE_3_F_FOLLOWUP,
    // F2
    CUBE4_PHASE_3_F_FOLLOWUP,
    // R
    CUBE4_PHASE_3_R_FOLLOWUP,
    // Rp
    CUBE4_PHASE_3_R_FOLLOWUP,
    // R2
    CUBE4_PHASE_3_R_FOLLOWUP,
    // B
    CUBE4_PHASE_3_B_FOLLOWUP,
    // Bp
    CUBE4_PHASE_3_B_FOLLOWUP,
    // B2
    CUBE4_PHASE_3_B_FOLLOWUP,
    // L
    CUBE4_PHASE_3_L_FOLLOWUP,
    // Lp
    CUBE4_PHASE_3_L_FOLLOWUP,
    // L2
    CUBE4_PHASE_3_L_FOLLOWUP,
    // D
    CUBE4_PHASE_3_D_FOLLOWUP,
    // Dp
    CUBE4_PHASE_3_D_FOLLOWUP,
    // D2
    CUBE4_PHASE_3_D_FOLLOWUP,
    // Uw
    CUBE4_PHASE_3_UW_FOLLOWUP,
    // Uwp
    CUBE4_PHASE_3_UW_FOLLOWUP,
    // Uw2
    CUBE4_PHASE_3_UW_FOLLOWUP,
    // Fw
    CUBE4_PHASE_3_FW_FOLLOWUP,
    // Fwp
    CUBE4_PHASE_3_FW_FOLLOWUP,
    // Fw2
    CUBE4_PHASE_3_FW_FOLLOWUP,
    // Rw
    CUBE4_PHASE_3_RW_FOLLOWUP,
    // Rwp
    CUBE4_PHASE_3_RW_FOLLOWUP,
    // Rw2
    CUBE4_PHASE_3_RW_FOLLOWUP,
    // Bw
    CUBE4_PHASE_3_BW_FOLLOWUP,
    // Bwp
    CUBE4_PHASE_3_BW_FOLLOWUP,
    // Bw2
    CUBE4_PHASE_3_BW_FOLLOWUP,
    // Lw
    CUBE4_PHASE_3_LW_FOLLOWUP,
    // Lwp
    CUBE4_PHASE_3_LW_FOLLOWUP,
    // Lw2
    CUBE4_PHASE_3_LW_FOLLOWUP,
    // Dw
    CUBE4_PHASE_3_DW_FOLLOWUP,
    // Dwp
    CUBE4_PHASE_3_DW_FOLLOWUP,
    // Dw2
    CUBE4_PHASE_3_DW_FOLLOWUP,
];

const CUBE4_PHASE_3_U_FOLLOWUP: &'static [Move] = &[
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_F_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_R_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_B_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_L_FOLLOWUP: &'static [Move] = &[
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
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_D_FOLLOWUP: &'static [Move] = &[
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::Uw2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_UW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_FW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_RW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Bw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_BW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Rw2,
    Move::Lw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_LW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Uw2,
    Move::Fw2,
    Move::Bw2,
    Move::Dw2,
];

const CUBE4_PHASE_3_DW_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F,
    Move::Fp,
    Move::F2,
    Move::R2,
    Move::B,
    Move::Bp,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

// Set of moves possible as the first move in phase 4 (R2 L2 F2 B2 U D Rw2 Lw2 Fw2 Bw2)
pub(crate) const CUBE4_POSSIBLE_PHASE_4_MOVES: &'static [Move] = &[
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
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

// Set of moves that are allowed in phase 4 following each given moves. For example, L should never follow L'.
// Also, avoid move sequences like L R L by forcing opposite faces to be turned only in a single order.
pub(crate) const CUBE4_POSSIBLE_PHASE_4_FOLLOWUP_MOVES: [&'static [Move]; Move::count_4x4x4()] = [
    // U
    CUBE4_PHASE_4_U_FOLLOWUP,
    // Up
    CUBE4_PHASE_4_U_FOLLOWUP,
    // U2
    CUBE4_PHASE_4_U_FOLLOWUP,
    // F
    CUBE4_PHASE_4_F_FOLLOWUP,
    // Fp
    CUBE4_PHASE_4_F_FOLLOWUP,
    // F2
    CUBE4_PHASE_4_F_FOLLOWUP,
    // R
    CUBE4_PHASE_4_R_FOLLOWUP,
    // Rp
    CUBE4_PHASE_4_R_FOLLOWUP,
    // R2
    CUBE4_PHASE_4_R_FOLLOWUP,
    // B
    CUBE4_PHASE_4_B_FOLLOWUP,
    // Bp
    CUBE4_PHASE_4_B_FOLLOWUP,
    // B2
    CUBE4_PHASE_4_B_FOLLOWUP,
    // L
    CUBE4_PHASE_4_L_FOLLOWUP,
    // Lp
    CUBE4_PHASE_4_L_FOLLOWUP,
    // L2
    CUBE4_PHASE_4_L_FOLLOWUP,
    // D
    CUBE4_PHASE_4_D_FOLLOWUP,
    // Dp
    CUBE4_PHASE_4_D_FOLLOWUP,
    // D2
    CUBE4_PHASE_4_D_FOLLOWUP,
    // Uw
    CUBE4_PHASE_4_UW_FOLLOWUP,
    // Uwp
    CUBE4_PHASE_4_UW_FOLLOWUP,
    // Uw2
    CUBE4_PHASE_4_UW_FOLLOWUP,
    // Fw
    CUBE4_PHASE_4_FW_FOLLOWUP,
    // Fwp
    CUBE4_PHASE_4_FW_FOLLOWUP,
    // Fw2
    CUBE4_PHASE_4_FW_FOLLOWUP,
    // Rw
    CUBE4_PHASE_4_RW_FOLLOWUP,
    // Rwp
    CUBE4_PHASE_4_RW_FOLLOWUP,
    // Rw2
    CUBE4_PHASE_4_RW_FOLLOWUP,
    // Bw
    CUBE4_PHASE_4_BW_FOLLOWUP,
    // Bwp
    CUBE4_PHASE_4_BW_FOLLOWUP,
    // Bw2
    CUBE4_PHASE_4_BW_FOLLOWUP,
    // Lw
    CUBE4_PHASE_4_LW_FOLLOWUP,
    // Lwp
    CUBE4_PHASE_4_LW_FOLLOWUP,
    // Lw2
    CUBE4_PHASE_4_LW_FOLLOWUP,
    // Dw
    CUBE4_PHASE_4_DW_FOLLOWUP,
    // Dwp
    CUBE4_PHASE_4_DW_FOLLOWUP,
    // Dw2
    CUBE4_PHASE_4_DW_FOLLOWUP,
];

const CUBE4_PHASE_4_U_FOLLOWUP: &'static [Move] = &[
    Move::F2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_F_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_R_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F2,
    Move::B2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_B_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::R2,
    Move::L2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_L_FOLLOWUP: &'static [Move] = &[
    Move::U,
    Move::Up,
    Move::U2,
    Move::F2,
    Move::B2,
    Move::D,
    Move::Dp,
    Move::D2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_D_FOLLOWUP: &'static [Move] = &[
    Move::F2,
    Move::R2,
    Move::B2,
    Move::L2,
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_UW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_FW_FOLLOWUP: &'static [Move] = &[
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
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_RW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Bw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_BW_FOLLOWUP: &'static [Move] = &[
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
    Move::Rw2,
    Move::Lw2,
];

const CUBE4_PHASE_4_LW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Bw2,
];

const CUBE4_PHASE_4_DW_FOLLOWUP: &'static [Move] = &[
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
    Move::Fw2,
    Move::Rw2,
    Move::Bw2,
    Move::Lw2,
];
