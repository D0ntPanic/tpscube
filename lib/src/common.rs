use crate::rand::{RandomSource, StandardRandomSource};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use num_enum::TryFromPrimitive;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

#[cfg(not(feature = "no_solver"))]
use std::convert::TryInto;

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
/// Identification of a corner piece. Names come from the faces of the cube this corner
/// belongs to on a solved cube.
pub enum Corner {
    URF = 0,
    UFL = 1,
    ULB = 2,
    UBR = 3,
    DFR = 4,
    DLF = 5,
    DBL = 6,
    DRB = 7,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CornerPiece {
    pub piece: Corner,
    pub orientation: u8,
}

#[cfg(not(feature = "no_solver"))]
pub(crate) struct CornerOrientationMoveTable;
#[cfg(not(feature = "no_solver"))]
pub(crate) struct CornerPermutationMoveTable;
#[cfg(not(feature = "no_solver"))]
pub(crate) struct CornerOrientationPruneTable;
#[cfg(not(feature = "no_solver"))]
pub(crate) struct CornerPermutationPruneTable;

impl Color {
    pub fn face(&self) -> CubeFace {
        match self {
            Color::White => CubeFace::Top,
            Color::Green => CubeFace::Front,
            Color::Red => CubeFace::Right,
            Color::Blue => CubeFace::Back,
            Color::Orange => CubeFace::Left,
            Color::Yellow => CubeFace::Bottom,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Color::White => "White",
            Color::Green => "Green",
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Orange => "Orange",
            Color::Yellow => "Yellow",
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TryFromPrimitive)]
/// Faces of the cube relative to viewing the cube with white on top and green in front
pub enum CubeFace {
    Top = 0,
    Front = 1,
    Right = 2,
    Back = 3,
    Left = 4,
    Bottom = 5,
}

impl CubeFace {
    pub fn color(&self) -> Color {
        match self {
            CubeFace::Top => Color::White,
            CubeFace::Front => Color::Green,
            CubeFace::Right => Color::Red,
            CubeFace::Back => Color::Blue,
            CubeFace::Left => Color::Orange,
            CubeFace::Bottom => Color::Yellow,
        }
    }

    pub fn opposite(&self) -> CubeFace {
        match self {
            CubeFace::Top => CubeFace::Bottom,
            CubeFace::Front => CubeFace::Back,
            CubeFace::Right => CubeFace::Left,
            CubeFace::Back => CubeFace::Front,
            CubeFace::Left => CubeFace::Right,
            CubeFace::Bottom => CubeFace::Top,
        }
    }
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

#[cfg(not(feature = "no_solver"))]
impl CornerOrientationMoveTable {
    pub fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_3x3x3() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::CUBE_CORNER_ORIENTATION_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl CornerPermutationMoveTable {
    pub fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_3x3x3() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::CUBE_CORNER_PERMUTATION_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl CornerOrientationPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::CUBE_CORNER_ORIENTATION_PRUNE_TABLE[idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl CornerPermutationPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::CUBE_CORNER_PERMUTATION_PRUNE_TABLE[idx as usize] as usize
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

#[derive(Clone)]
pub struct BestSolve {
    pub solve: Solve,
    pub time: u32,
}

#[derive(Clone)]
pub struct Average {
    pub solves: Vec<Solve>,
    pub time: u32,
}

pub trait ListAverage {
    fn average(&self) -> Option<u32>;
    fn average_ignore_dnf(&self) -> Option<u32>;
}

pub trait SolveList: ListAverage {
    fn last_average(&self, count: usize) -> Option<Average>;
    fn last_average_ignore_dnf(&self, count: usize) -> Option<Average>;
    fn best(&self) -> Option<BestSolve>;
    fn best_average(&self, count: usize) -> Option<Average>;
    fn best_average_ignore_dnf(&self, count: usize) -> Option<Average>;
}

impl ListAverage for &[Option<u32>] {
    fn average(&self) -> Option<u32> {
        if self.len() == 0 {
            return None;
        }

        // Sort solves by time, ensuring that DNF is considered the
        // maximum time.
        let mut sorted: Vec<Option<u32>> = self.to_vec();
        sorted.sort_unstable_by(|a, b| {
            if a.is_none() && b.is_none() {
                Ordering::Equal
            } else if a.is_none() {
                Ordering::Greater
            } else if b.is_none() {
                Ordering::Less
            } else {
                let a = a.unwrap();
                let b = b.unwrap();
                a.cmp(&b)
            }
        });

        // Remove the best and worst time(s) as appropriate for the size of the set.
        // If there are less than 5 values, use an arithmetic mean and do not
        // eliminate any values.
        let to_remove = if sorted.len() >= 5 {
            (sorted.len() + 39) / 40
        } else {
            0
        };
        let solves = &sorted[to_remove..sorted.len() - to_remove];

        // Sum the solves that are not removed. If there is a DNF in this set, the
        // entire average is invalid.
        let sum = solves.iter().fold(Some(0), |sum, time| {
            if let Some(sum) = sum {
                if let Some(time) = time {
                    Some(sum + *time as u64)
                } else {
                    None
                }
            } else {
                None
            }
        });

        // Compute final average
        if let Some(sum) = sum {
            Some(((sum + (solves.len() as u64 / 2)) / (solves.len() as u64)) as u32)
        } else {
            None
        }
    }

    fn average_ignore_dnf(&self) -> Option<u32> {
        if self.len() == 0 {
            return None;
        }

        // Sort solves by time, ensuring that DNF is considered the
        // maximum time.
        let mut sorted: Vec<Option<u32>> = self.to_vec();
        sorted.retain(|a| a.is_some());
        sorted.sort_unstable_by(|a, b| {
            if a.is_none() && b.is_none() {
                Ordering::Equal
            } else if a.is_none() {
                Ordering::Greater
            } else if b.is_none() {
                Ordering::Less
            } else {
                let a = a.unwrap();
                let b = b.unwrap();
                a.cmp(&b)
            }
        });

        // Remove the best and worst time(s) as appropriate for the size of the set.
        // If there are less than 5 values, use an arithmetic mean and do not
        // eliminate any values.
        let to_remove = if sorted.len() >= 5 {
            (sorted.len() + 39) / 40
        } else {
            0
        };
        let solves = &sorted[to_remove..sorted.len() - to_remove];

        // Sum the solves that are not removed. If there is a DNF in this set, the
        // entire average is invalid.
        let sum = solves.iter().fold(Some(0), |sum, time| {
            if let Some(sum) = sum {
                if let Some(time) = time {
                    Some(sum + *time as u64)
                } else {
                    None
                }
            } else {
                None
            }
        });

        // Compute final average
        if let Some(sum) = sum {
            if solves.len() > 0 {
                Some(((sum + (solves.len() as u64 / 2)) / (solves.len() as u64)) as u32)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ListAverage for &[Solve] {
    fn average(&self) -> Option<u32> {
        let times: Vec<Option<u32>> = self.iter().map(|solve| solve.final_time()).collect();
        times.as_slice().average()
    }
    fn average_ignore_dnf(&self) -> Option<u32> {
        let times: Vec<Option<u32>> = self.iter().map(|solve| solve.final_time()).collect();
        times.as_slice().average_ignore_dnf()
    }
}

impl SolveList for &[Solve] {
    fn last_average(&self, count: usize) -> Option<Average> {
        if self.len() >= count {
            let solves = self[self.len() - count..].to_vec();
            if let Some(time) = solves.as_slice().average() {
                Some(Average { solves, time })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn last_average_ignore_dnf(&self, count: usize) -> Option<Average> {
        if self.len() >= count {
            // let mut solves = self[self.len() - count..].to_vec();
            let mut solves = self.to_vec();
            solves.retain(|solve| solve.final_time().is_some());
            solves = solves[solves.len() - count..].to_vec();
            if let Some(time) = solves.as_slice().average() {
                Some(Average { solves, time })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn best(&self) -> Option<BestSolve> {
        self.iter()
            .fold(None, |best, solve| {
                if let Some(time) = solve.final_time() {
                    if let Some((best_solve, best_time)) = best {
                        if time < best_time {
                            Some((solve, time))
                        } else {
                            Some((best_solve, best_time))
                        }
                    } else {
                        Some((solve, time))
                    }
                } else {
                    best
                }
            })
            .map(|best| {
                let (solve, time) = best;
                BestSolve {
                    solve: solve.clone(),
                    time,
                }
            })
    }

    fn best_average(&self, count: usize) -> Option<Average> {
        self.windows(count)
            .fold(None, |best, solves| {
                if let Some(time) = solves.average() {
                    if let Some((best_solves, best_time)) = best {
                        if time < best_time {
                            Some((solves, time))
                        } else {
                            Some((best_solves, best_time))
                        }
                    } else {
                        Some((solves, time))
                    }
                } else {
                    best
                }
            })
            .map(|best| {
                let (solves, time) = best;
                Average {
                    solves: solves.to_vec(),
                    time,
                }
            })
    }
    fn best_average_ignore_dnf(&self, count: usize) -> Option<Average> {
        self.windows(count)
            .fold(None, |best, solves| {
                if let Some(time) = solves.average_ignore_dnf() {
                    if let Some((best_solves, best_time)) = best {
                        if time < best_time {
                            Some((solves, time))
                        } else {
                            Some((best_solves, best_time))
                        }
                    } else {
                        Some((solves, time))
                    }
                } else {
                    best
                }
            })
            .map(|best| {
                let (solves, time) = best;
                Average {
                    solves: solves.to_vec(),
                    time,
                }
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Penalty {
    None,
    Time(u32),
    DNF,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum SolveType {
    Standard3x3x3 = 0,
    OneHanded3x3x3 = 1,
    Blind3x3x3 = 2,
    Standard2x2x2 = 3,
    /*Standard4x4x4 = 4,
    Blind4x4x4 = 5,
    Standard5x5x5 = 6,
    Blind5x5x5 = 7,
    Standard6x6x6 = 8,
    Standard7x7x7 = 9,
    Pyraminx = 10,
    Megaminx = 11,
    Skewb = 12,
    Square1 = 13,
    Clock = 14,*/
}

impl SolveType {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "3x3x3" => Some(SolveType::Standard3x3x3),
            "3x3x3 OH" => Some(SolveType::OneHanded3x3x3),
            "3x3x3 Blind" => Some(SolveType::Blind3x3x3),
            "2x2x2" => Some(SolveType::Standard2x2x2),
            /*"4x4x4" => Some(SolveType::Standard4x4x4),
            "4x4x4 Blind" => Some(SolveType::Blind4x4x4),
            "5x5x5" => Some(SolveType::Standard5x5x5),
            "5x5x5 Blind" => Some(SolveType::Blind5x5x5),
            "6x6x6" => Some(SolveType::Standard6x6x6),
            "7x7x7" => Some(SolveType::Standard7x7x7),
            "Pyraminx" => Some(SolveType::Pyraminx),
            "Megaminx" => Some(SolveType::Megaminx),
            "Skewb" => Some(SolveType::Skewb),
            "Square-1" => Some(SolveType::Square1),
            "Clock" => Some(SolveType::Clock),*/
            _ => None,
        }
    }

    pub fn is_3x3x3(&self) -> bool {
        match self {
            SolveType::Standard3x3x3 => true,
            SolveType::OneHanded3x3x3 => true,
            SolveType::Blind3x3x3 => true,
            _ => false,
        }
    }
}

impl ToString for SolveType {
    fn to_string(&self) -> String {
        match self {
            SolveType::Standard3x3x3 => "3x3x3".into(),
            SolveType::OneHanded3x3x3 => "3x3x3 OH".into(),
            SolveType::Blind3x3x3 => "3x3x3 Blind".into(),
            SolveType::Standard2x2x2 => "2x2x2".into(),
            /*SolveType::Standard4x4x4 => "4x4x4".into(),
            SolveType::Blind4x4x4 => "4x4x4 Blind".into(),
            SolveType::Standard5x5x5 => "5x5x5".into(),
            SolveType::Blind5x5x5 => "5x5x5 Blind".into(),
            SolveType::Standard6x6x6 => "6x6x6".into(),
            SolveType::Standard7x7x7 => "7x7x7".into(),
            SolveType::Pyraminx => "Pyraminx".into(),
            SolveType::Megaminx => "Megaminx".into(),
            SolveType::Skewb => "Skewb".into(),
            SolveType::Square1 => "Square-1".into(),
            SolveType::Clock => "Clock".into(),*/
        }
    }
}

impl Move {
    pub fn from_face_and_rotation(face: CubeFace, rotation: i32) -> Option<Self> {
        let rotation = rotation % 4;
        match face {
            CubeFace::Top => match rotation {
                -3 => Some(Move::U),
                -2 => Some(Move::U2),
                -1 => Some(Move::Up),
                1 => Some(Move::U),
                2 => Some(Move::U2),
                3 => Some(Move::Up),
                _ => None,
            },
            CubeFace::Front => match rotation {
                -3 => Some(Move::F),
                -2 => Some(Move::F2),
                -1 => Some(Move::Fp),
                1 => Some(Move::F),
                2 => Some(Move::F2),
                3 => Some(Move::Fp),
                _ => None,
            },
            CubeFace::Right => match rotation {
                -3 => Some(Move::R),
                -2 => Some(Move::R2),
                -1 => Some(Move::Rp),
                1 => Some(Move::R),
                2 => Some(Move::R2),
                3 => Some(Move::Rp),
                _ => None,
            },
            CubeFace::Back => match rotation {
                -3 => Some(Move::B),
                -2 => Some(Move::B2),
                -1 => Some(Move::Bp),
                1 => Some(Move::B),
                2 => Some(Move::B2),
                3 => Some(Move::Bp),
                _ => None,
            },
            CubeFace::Left => match rotation {
                -3 => Some(Move::L),
                -2 => Some(Move::L2),
                -1 => Some(Move::Lp),
                1 => Some(Move::L),
                2 => Some(Move::L2),
                3 => Some(Move::Lp),
                _ => None,
            },
            CubeFace::Bottom => match rotation {
                -3 => Some(Move::D),
                -2 => Some(Move::D2),
                -1 => Some(Move::Dp),
                1 => Some(Move::D),
                2 => Some(Move::D2),
                3 => Some(Move::Dp),
                _ => None,
            },
        }
    }

    pub(crate) fn sourced_random_2x2x2<T: RandomSource>(rng: &mut T) -> Move {
        Move::try_from(rng.next(Self::count_2x2x2() as u32) as u8).unwrap()
    }

    pub(crate) fn sourced_random_3x3x3<T: RandomSource>(rng: &mut T) -> Move {
        Move::try_from(rng.next(Self::count_3x3x3() as u32) as u8).unwrap()
    }

    /// Gets a randomly chosen move
    pub fn random_2x2x2() -> Move {
        Self::sourced_random_2x2x2(&mut StandardRandomSource)
    }

    /// Gets a randomly chosen move
    pub fn random_3x3x3() -> Move {
        Self::sourced_random_3x3x3(&mut StandardRandomSource)
    }

    pub const fn count_2x2x2() -> usize {
        Move::D2 as u8 as usize + 1
    }

    pub const fn count_3x3x3() -> usize {
        Move::D2 as u8 as usize + 1
    }

    pub const fn face(&self) -> CubeFace {
        match self {
            Move::U => CubeFace::Top,
            Move::Up => CubeFace::Top,
            Move::U2 => CubeFace::Top,
            Move::F => CubeFace::Front,
            Move::Fp => CubeFace::Front,
            Move::F2 => CubeFace::Front,
            Move::R => CubeFace::Right,
            Move::Rp => CubeFace::Right,
            Move::R2 => CubeFace::Right,
            Move::B => CubeFace::Back,
            Move::Bp => CubeFace::Back,
            Move::B2 => CubeFace::Back,
            Move::L => CubeFace::Left,
            Move::Lp => CubeFace::Left,
            Move::L2 => CubeFace::Left,
            Move::D => CubeFace::Bottom,
            Move::Dp => CubeFace::Bottom,
            Move::D2 => CubeFace::Bottom,
        }
    }

    /// Gets the face rotation amount in number of 90 degree clockwise rotations
    pub const fn rotation(&self) -> i32 {
        match self {
            Move::U => 1,
            Move::Up => -1,
            Move::U2 => 2,
            Move::F => 1,
            Move::Fp => -1,
            Move::F2 => 2,
            Move::R => 1,
            Move::Rp => -1,
            Move::R2 => 2,
            Move::B => 1,
            Move::Bp => -1,
            Move::B2 => 2,
            Move::L => 1,
            Move::Lp => -1,
            Move::L2 => 2,
            Move::D => 1,
            Move::Dp => -1,
            Move::D2 => 2,
        }
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

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "U" => Some(Move::U),
            "U'" => Some(Move::Up),
            "U2" => Some(Move::U2),
            "F" => Some(Move::F),
            "F'" => Some(Move::Fp),
            "F2" => Some(Move::F2),
            "R" => Some(Move::R),
            "R'" => Some(Move::Rp),
            "R2" => Some(Move::R2),
            "B" => Some(Move::B),
            "B'" => Some(Move::Bp),
            "B2" => Some(Move::B2),
            "L" => Some(Move::L),
            "L'" => Some(Move::Lp),
            "L2" => Some(Move::L2),
            "D" => Some(Move::D),
            "D'" => Some(Move::Dp),
            "D2" => Some(Move::D2),
            _ => None,
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
pub trait MoveSequence: Sized {
    /// Returns the inverse of this move sequence (undoing all moves)
    fn inverse(&self) -> Vec<Move>;

    /// Returns the human-readable string for this move sequence
    fn to_string(&self) -> String;
}

impl MoveSequence for Vec<Move> {
    fn inverse(&self) -> Vec<Move> {
        self.as_slice().inverse()
    }

    fn to_string(&self) -> String {
        self.as_slice().to_string()
    }
}

impl MoveSequence for &[Move] {
    fn inverse(&self) -> Vec<Move> {
        self.iter().rev().map(|mv| mv.inverse()).collect()
    }

    fn to_string(&self) -> String {
        let moves: Vec<String> = self.iter().map(|mv| mv.to_string()).collect();
        moves.join(" ")
    }
}

/// Operations on sequences of cube moves with timing information
pub trait TimedMoveSequence {
    /// Returns the human-readable string for this move sequence
    fn to_string(&self) -> String;
}

impl TimedMoveSequence for Vec<TimedMove> {
    fn to_string(&self) -> String {
        self.as_slice().to_string()
    }
}

impl TimedMoveSequence for &[TimedMove] {
    fn to_string(&self) -> String {
        let moves: Vec<String> = self
            .iter()
            .map(|mv| format!("{}@{}", mv.0.to_string(), mv.1))
            .collect();
        moves.join(" ")
    }
}

pub trait InitialCubeState: Sized {
    /// Creates a new cube in the solved state
    fn new() -> Self;

    /// Generates a random cube state with a given random number source
    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self;

    /// Generates a random cube state
    fn random() -> Self {
        Self::sourced_random(&mut StandardRandomSource)
    }
}

pub trait Cube {
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

    fn size(&self) -> usize;
    fn colors(&self) -> BTreeMap<CubeFace, Vec<Vec<Color>>>;

    /// Finds an efficient solution to this cube state
    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>>;

    /// Finds any solution to this cube state. Likely has many more moves than the
    /// result of `solve`.
    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>>;

    fn reset(&mut self);
    fn dyn_clone(&self) -> Box<dyn Cube>;
}

pub fn parse_move_string(string: &str) -> Result<Vec<Move>> {
    let mut moves = Vec::new();
    for move_str in string.split(' ') {
        if move_str.len() == 0 {
            continue;
        }
        let mv = Move::from_str(move_str).ok_or_else(|| anyhow!("Invalid move '{}'", move_str))?;
        moves.push(mv);
    }
    Ok(moves)
}

pub fn parse_timed_move_string(string: &str) -> Result<Vec<TimedMove>> {
    let mut moves = Vec::new();
    for move_str in string.split(' ') {
        if move_str.len() == 0 {
            continue;
        }
        let mut move_iter = move_str.split('@');
        let mv_str = move_iter
            .next()
            .ok_or_else(|| anyhow!("Invalid move '{}'", move_str))?;
        let time_str = move_iter
            .next()
            .ok_or_else(|| anyhow!("Invalid move '{}'", move_str))?;
        let mv = Move::from_str(mv_str).ok_or_else(|| anyhow!("Invalid move '{}'", move_str))?;
        let time = u32::from_str(time_str)?;
        moves.push(TimedMove(mv, time));
    }
    Ok(moves)
}
