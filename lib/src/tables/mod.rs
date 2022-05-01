pub(crate) mod analysis;
pub(crate) mod corner;
pub(crate) mod table2x2x2;
pub(crate) mod table3x3x3;
pub(crate) mod table4x4x4;

pub(crate) const CUBE_CORNER_ORIENTATION_INDEX_COUNT: usize = 3usize.pow(7);
pub(crate) const CUBE_CORNER_PERMUTATION_INDEX_COUNT: usize = crate::common::factorial(8);

pub(crate) const CUBE3_EDGE_ORIENTATION_INDEX_COUNT: usize = 2usize.pow(11);
pub(crate) const CUBE3_PHASE_2_EDGE_PERMUTATION_INDEX_COUNT: usize = crate::common::factorial(8);
pub(crate) const CUBE3_EDGE_SLICE_INDEX_COUNT: usize = crate::common::n_choose_k(12, 4);
pub(crate) const CUBE3_PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT: usize =
    crate::common::factorial(4);

pub(crate) const CUBE4_PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(24, 4);
pub(crate) const CUBE4_PHASE_2_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(8, 4);
pub(crate) const CUBE4_PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(16, 8);
pub(crate) const CUBE4_PHASE_3_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(8, 4);
pub(crate) const CUBE4_PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(8, 4);
pub(crate) const CUBE4_PHASE_3_EDGE_PAIR_INDEX_COUNT: usize =
    crate::common::factorial(12) / crate::common::factorial(6);
pub(crate) const CUBE4_PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(4, 2);
pub(crate) const CUBE4_PHASE_4_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(4, 2);
pub(crate) const CUBE4_PHASE_4_WHITE_YELLOW_CENTERS_INDEX_COUNT: usize =
    crate::common::n_choose_k(8, 4);
pub(crate) const CUBE4_PHASE_4_CENTERS_INDEX_COUNT: usize =
    CUBE4_PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT
        * CUBE4_PHASE_4_GREEN_BLUE_CENTERS_INDEX_COUNT
        * CUBE4_PHASE_4_WHITE_YELLOW_CENTERS_INDEX_COUNT;
pub(crate) const CUBE4_PHASE_4_EDGE_PAIR_INDEX_COUNT: usize = crate::common::factorial(8);

#[cfg(not(feature = "no_solver"))]
pub(crate) mod solve;
