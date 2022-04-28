pub(crate) mod analysis;
pub(crate) mod corner;
pub(crate) mod table2x2x2;
pub(crate) mod table3x3x3;
pub(crate) mod table4x4x4;

#[cfg(not(feature = "no_solver"))]
pub(crate) mod solve;
