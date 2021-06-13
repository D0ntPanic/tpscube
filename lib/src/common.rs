use crate::rand::{RandomSource, StandardRandomSource};
use chrono::{DateTime, Local};
use num_enum::TryFromPrimitive;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::TryFrom;
use uuid::Uuid;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Colors of the cube
pub enum Color {
    White = 0,
    Green = 1,
    Red = 2,
    Blue = 3,
    Orange = 4,
    Yellow = 5,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Faces of the cube relative to viewing the cube with white on top and green in front
pub enum Face {
    Top = 0,
    Front = 1,
    Right = 2,
    Back = 3,
    Left = 4,
    Bottom = 5,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Direction for face rotation
pub enum RotationDirection {
    CW = 0,
    CCW = 1,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Move to perform on a cube
pub enum Move {
    U = 0,
    Up = 1,
    U2 = 2,
    F = 3,
    Fp = 4,
    F2 = 5,
    R = 6,
    Rp = 7,
    R2 = 8,
    B = 9,
    Bp = 10,
    B2 = 11,
    L = 12,
    Lp = 13,
    L2 = 14,
    D = 15,
    Dp = 16,
    D2 = 17,
}

#[derive(Clone, Debug)]
pub struct TimedMove(Move, u32);

#[derive(Clone, Debug)]
pub struct Solve {
    pub id: String,
    pub solve_type: SolveType,
    pub session: String,
    pub scramble: Vec<Move>,
    pub created: DateTime<Local>,
    pub time: u32,
    pub penalty: Penalty,
    pub device: Option<String>,
    pub moves: Option<Vec<TimedMove>>,
}

impl Solve {
    pub fn new_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn final_time(&self) -> Option<u32> {
        match self.penalty {
            Penalty::None => Some(self.time),
            Penalty::Time(penalty) => Some(self.time + penalty),
            Penalty::DNF => None,
        }
    }
}

impl PartialEq for Solve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Solve {}

impl PartialOrd for Solve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.created.cmp(&other.created) {
            Ordering::Equal => Some(self.id.cmp(&other.id)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for Solve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum Penalty {
    None,
    Time(u32),
    DNF,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
pub enum SolveType {
    Standard3x3x3 = 0,
}

impl Move {
    pub(crate) fn sourced_random_3x3x3<T: RandomSource>(rng: &mut T) -> Move {
        Move::try_from(rng.next(Self::count_3x3x3() as u32) as u8).unwrap()
    }

    /// Gets a randomly chosen move
    pub fn random_3x3x3() -> Move {
        Self::sourced_random_3x3x3(&mut StandardRandomSource)
    }

    pub const fn count_3x3x3() -> usize {
        Move::D2 as u8 as usize + 1
    }

    pub const fn inverse(&self) -> Self {
        match self {
            Move::U => Move::Up,
            Move::Up => Move::U,
            Move::U2 => Move::U2,
            Move::F => Move::Fp,
            Move::Fp => Move::F,
            Move::F2 => Move::F2,
            Move::R => Move::Rp,
            Move::Rp => Move::R,
            Move::R2 => Move::R2,
            Move::B => Move::Bp,
            Move::Bp => Move::B,
            Move::B2 => Move::B2,
            Move::L => Move::Lp,
            Move::Lp => Move::L,
            Move::L2 => Move::L2,
            Move::D => Move::Dp,
            Move::Dp => Move::D,
            Move::D2 => Move::D2,
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        match self {
            Move::U => "U".into(),
            Move::Up => "U'".into(),
            Move::U2 => "U2".into(),
            Move::F => "F".into(),
            Move::Fp => "F'".into(),
            Move::F2 => "F2".into(),
            Move::R => "R".into(),
            Move::Rp => "R'".into(),
            Move::R2 => "R2".into(),
            Move::B => "B".into(),
            Move::Bp => "B'".into(),
            Move::B2 => "B2".into(),
            Move::L => "L".into(),
            Move::Lp => "L'".into(),
            Move::L2 => "L2".into(),
            Move::D => "D".into(),
            Move::Dp => "D'".into(),
            Move::D2 => "D2".into(),
        }
    }
}

impl TimedMove {
    pub fn new(mv: Move, time: u32) -> Self {
        Self(mv, time)
    }

    pub fn move_(&self) -> Move {
        self.0
    }

    pub fn time(&self) -> u32 {
        self.1
    }
}

/// Operations on sequences of cube moves
pub trait MoveSequence {
    /// Returns the inverse of this move sequence (undoing all moves)
    fn inverse(&self) -> Vec<Move>;
}

impl MoveSequence for Vec<Move> {
    fn inverse(&self) -> Vec<Move> {
        self.iter().rev().map(|mv| mv.inverse()).collect()
    }
}

impl MoveSequence for &[Move] {
    fn inverse(&self) -> Vec<Move> {
        self.iter().rev().map(|mv| mv.inverse()).collect()
    }
}

pub trait Cube: Sized {
    /// Creates a new cube in the solved state
    fn new() -> Self;

    /// Generates a random cube state with a given random number source
    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self;

    /// Generates a random cube state
    fn random() -> Self {
        Self::sourced_random(&mut StandardRandomSource)
    }

    /// Determines if this cube is in the solved state
    fn is_solved(&self) -> bool;

    /// Perform a move on the cube
    fn do_move(&mut self, mv: Move);

    /// Perform a sequence of moves on the cube
    fn do_moves(&mut self, seq: &[Move]) {
        for mv in seq {
            self.do_move(*mv);
        }
    }

    /// Finds an efficient solution to this cube state
    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>>;

    /// Finds any solution to this cube state. Likely has many more moves than the
    /// result of `solve`.
    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>>;
}
