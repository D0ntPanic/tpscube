use crate::{
    Color, Corner, CornerPiece, Cube, CubeFace, FaceRotation, InitialCubeState, Move, RandomSource,
    RotationDirection,
};
use num_enum::TryFromPrimitive;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[cfg(not(feature = "no_solver"))]
use crate::common::MoveSequence;
#[cfg(not(feature = "no_solver"))]
use crate::Cube3x3x3Faces;
#[cfg(not(feature = "no_solver"))]
use std::convert::TryInto;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Identification of an edge piece. Names come from the faces of the cube this edge
/// belongs to on a solved cube, followed by a differentiator for which of the two
/// edge pieces on the side. The indexes must be chosen such that edges rotating on
/// an inner slice always inverts orientation.
pub enum Edge4x4x4 {
    URB = 0,
    URF = 1,
    UFR = 2,
    UFL = 3,
    ULF = 4,
    ULB = 5,
    UBL = 6,
    UBR = 7,
    DRF = 8,
    DRB = 9,
    DFL = 10,
    DFR = 11,
    DLB = 12,
    DLF = 13,
    DBR = 14,
    DBL = 15,
    RFD = 16,
    RFU = 17,
    LFU = 18,
    LFD = 19,
    LBD = 20,
    LBU = 21,
    RBU = 22,
    RBD = 23,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EdgePiece4x4x4 {
    pub piece: Edge4x4x4,
    pub orientation: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A 4x4x4 cube represented in piece format (optimal for computational algorithms).
pub struct Cube4x4x4 {
    corners: [CornerPiece; 8],
    edges: [EdgePiece4x4x4; 24],
    centers: [Color; 24],
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A 4x4x4 cube represented in face color format (easy to use, matches visuals).
pub struct Cube4x4x4Faces {
    state: [Color; 6 * 16],
}

#[cfg(not(feature = "no_solver"))]
struct Phase1RedCentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase1OrangeCentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase2And3RedOrangeCentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase2GreenBlueCentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase3GreenBlueCentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase4CentersMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase4EdgePairMoveTable;
#[cfg(not(feature = "no_solver"))]
struct Phase1RedCentersPruneTable;
#[cfg(not(feature = "no_solver"))]
struct Phase1OrangeCentersPruneTable;
#[cfg(not(feature = "no_solver"))]
struct Phase2CentersPruneTable;
#[cfg(not(feature = "no_solver"))]
struct Phase3CentersPruneTable;
#[cfg(not(feature = "no_solver"))]
struct Phase4CentersPruneTable;
#[cfg(not(feature = "no_solver"))]
struct Phase4EdgePairPruneTable;

#[cfg(not(feature = "no_solver"))]
impl Phase1RedCentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_1_RED_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase1OrangeCentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_1_ORANGE_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase2And3RedOrangeCentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_2_3_RED_ORANGE_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase2GreenBlueCentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_2_GREEN_BLUE_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase3GreenBlueCentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_3_GREEN_BLUE_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase4CentersMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_4_CENTERS_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase4EdgePairMoveTable {
    fn get(idx: u16, mv: Move) -> u16 {
        let offset = idx as usize * Move::count_4x4x4() * 2 + mv as u8 as usize * 2;
        u16::from_le_bytes(
            crate::tables::solve::CUBE4_PHASE_4_EDGE_PAIR_MOVE_TABLE[offset..offset + 2]
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase1RedCentersPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_1_RED_CENTERS_PRUNE_TABLE[idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase1OrangeCentersPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_1_ORANGE_CENTERS_PRUNE_TABLE[idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase2CentersPruneTable {
    fn get(red_orange_idx: u16, green_blue_idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_2_CENTERS_PRUNE_TABLE[red_orange_idx as usize
            * Cube4x4x4::PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT
            + green_blue_idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase3CentersPruneTable {
    fn get(red_orange_idx: u16, green_blue_idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_3_CENTERS_PRUNE_TABLE[red_orange_idx as usize
            * Cube4x4x4::PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT
            + green_blue_idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase4CentersPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_4_CENTERS_PRUNE_TABLE[idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
impl Phase4EdgePairPruneTable {
    pub fn get(idx: u16) -> usize {
        crate::tables::solve::CUBE4_PHASE_4_EDGE_PAIR_PRUNE_TABLE[idx as usize] as usize
    }
}

#[cfg(not(feature = "no_solver"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Phase1IndexCube {
    red_centers: u16,
    orange_centers: u16,
}

#[cfg(not(feature = "no_solver"))]
impl Phase1IndexCube {
    fn new(pieces: &Cube4x4x4) -> Self {
        Self {
            red_centers: pieces.phase_1_red_centers_index(),
            orange_centers: pieces.phase_1_orange_centers_index(),
        }
    }

    fn do_move(&self, mv: Move) -> Self {
        Self {
            red_centers: Phase1RedCentersMoveTable::get(self.red_centers, mv),
            orange_centers: Phase1OrangeCentersMoveTable::get(self.orange_centers, mv),
        }
    }

    fn is_phase_solved(&self) -> bool {
        // There is more than one solution to this phase, check pruning lookup
        // table to determine if it is solved (the checks are precomputed and
        // baked into this table).
        Phase1RedCentersPruneTable::get(self.red_centers) == 0
            && Phase1OrangeCentersPruneTable::get(self.orange_centers) == 0
    }
}

#[cfg(not(feature = "no_solver"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct EdgePieces {
    edges: [u8; 24],
}

#[cfg(not(feature = "no_solver"))]
impl EdgePieces {
    fn new(pieces: &Cube4x4x4) -> Self {
        let mut edges = [0; 24];
        for i in 0..24 {
            // Only piece itself is required, edge orientation is implicit based
            // on positions and edge piece indexes and is not needed for solving
            edges[i] = pieces.edges[i].piece as u8;
        }
        Self { edges }
    }

    fn do_move(&self, mv: Move) -> Self {
        // Determine face, direction, and width of the move
        let face_idx = mv.face() as u8 as usize;
        let rotation = mv.rotation();
        let width = mv.width();
        let dir = if rotation < 0 {
            RotationDirection::CCW
        } else {
            RotationDirection::CW
        };
        let dir_idx = dir as u8 as usize;
        let mut new_edges = self.edges;

        // Perform rotation of the correct count for the move
        for _ in 0..rotation.abs() {
            // Save existing cube state so that it can be looked up during rotation
            let old_edges = new_edges;

            // Apply outer edge movement using lookup table
            for i in 0..8 {
                let (dest, src) =
                    crate::tables::table4x4x4::CUBE4_EDGE_PIECE_ROTATION[dir_idx][face_idx][i];
                new_edges[dest as u8 as usize] = old_edges[src.piece as u8 as usize];
            }

            if width == 2 {
                // Wide move, apply inner slice edge movement using lookup table. Inner edge pieces
                // always invert orientation no matter which face (this is an invariant that must
                // be true for the edge piece indexes to work properly).
                let edge_offset = match dir {
                    RotationDirection::CW => 3,
                    RotationDirection::CCW => 1,
                };
                for i in 0..4 {
                    let dest =
                        crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i];
                    let src = crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx]
                        [(i + edge_offset) % 4];
                    new_edges[dest as u8 as usize] = old_edges[src as u8 as usize];
                }
            }
        }

        Self { edges: new_edges }
    }

    fn edges_separated(&self) -> bool {
        // Edges are separated if the edge pieces in the even positions do not share any
        // edge pairs.
        let mut seen: [bool; 12] = [false; 12];
        for i in (0..24).step_by(2) {
            if seen[self.edges[i] as usize / 2] {
                return false;
            }
            seen[self.edges[i] as usize / 2] = true;
        }
        true
    }

    fn oll_parity(&self) -> bool {
        // OLL parity exists if there are an odd number of edges flipped (an edge is flipped
        // if the lower bit of the edge piece index does not match the lower bit of the
        // position index).
        let mut flips = 0;
        for i in (0..24).step_by(2) {
            flips += self.edges[i] & 1;
        }
        (flips & 1) != 0
    }

    fn edge_permutation_parity(&self) -> bool {
        // Permutation parity is defined using the number of piece swaps required to arrive
        // at a state with solved edges
        let mut edges = self.edges;
        let mut swaps = 0;
        let mut locations: [u8; 24] = [0; 24];
        for i in 0..24 {
            locations[edges[i] as usize] = i as u8;
        }
        for i in 0..24 {
            if edges[i] != i as u8 {
                locations[edges[i] as usize] = locations[i];
                edges.swap(i, locations[i] as usize);
                swaps += 1;
            }
        }
        (swaps & 1) != 0
    }

    fn equatorial_edges_paired(&self) -> bool {
        self.edges[Edge4x4x4::RFD as u8 as usize] / 2
            == self.edges[Edge4x4x4::RFU as u8 as usize] / 2
            && self.edges[Edge4x4x4::LFU as u8 as usize] / 2
                == self.edges[Edge4x4x4::LFD as u8 as usize] / 2
            && self.edges[Edge4x4x4::LBD as u8 as usize] / 2
                == self.edges[Edge4x4x4::LBU as u8 as usize] / 2
            && self.edges[Edge4x4x4::RBU as u8 as usize] / 2
                == self.edges[Edge4x4x4::RBD as u8 as usize] / 2
    }
}

#[cfg(not(feature = "no_solver"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Phase2IndexCube {
    red_orange_centers: u16,
    green_blue_centers: u16,
    edges: EdgePieces,
}

#[cfg(not(feature = "no_solver"))]
impl Phase2IndexCube {
    fn new(pieces: &Cube4x4x4) -> Self {
        Self {
            red_orange_centers: pieces.phase_2_and_3_red_orange_centers_index(),
            green_blue_centers: pieces.phase_2_green_blue_centers_index(),
            edges: EdgePieces::new(pieces),
        }
    }

    fn do_move(&self, mv: Move) -> Self {
        Self {
            red_orange_centers: Phase2And3RedOrangeCentersMoveTable::get(
                self.red_orange_centers,
                mv,
            ),
            green_blue_centers: Phase2GreenBlueCentersMoveTable::get(self.green_blue_centers, mv),
            edges: self.edges.do_move(mv),
        }
    }

    fn edges_ready(&self) -> bool {
        self.edges.edges_separated()
            && !self.edges.oll_parity()
            && !self.edges.edge_permutation_parity()
    }

    fn is_phase_solved(&self) -> bool {
        // There is more than one solution to this phase, check pruning lookup
        // table to determine if it is solved (the checks are precomputed and
        // baked into this table).
        Phase2CentersPruneTable::get(self.red_orange_centers, self.green_blue_centers) == 0
            && self.edges_ready()
    }
}

#[cfg(not(feature = "no_solver"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Phase3IndexCube {
    red_orange_centers: u16,
    green_blue_centers: u16,
    edges: EdgePieces,
}

#[cfg(not(feature = "no_solver"))]
impl Phase3IndexCube {
    fn new(pieces: &Cube4x4x4) -> Self {
        Self {
            red_orange_centers: pieces.phase_2_and_3_red_orange_centers_index(),
            green_blue_centers: pieces.phase_3_green_blue_centers_index(),
            edges: EdgePieces::new(pieces),
        }
    }

    fn do_move(&self, mv: Move) -> Self {
        Self {
            red_orange_centers: Phase2And3RedOrangeCentersMoveTable::get(
                self.red_orange_centers,
                mv,
            ),
            green_blue_centers: Phase3GreenBlueCentersMoveTable::get(self.green_blue_centers, mv),
            edges: self.edges.do_move(mv),
        }
    }

    fn edges_ready(&self) -> bool {
        self.edges.equatorial_edges_paired()
    }

    fn is_phase_solved(&self) -> bool {
        // There is more than one solution to this phase, check pruning lookup
        // table to determine if it is solved (the checks are precomputed and
        // baked into this table).
        Phase3CentersPruneTable::get(self.red_orange_centers, self.green_blue_centers) == 0
            && self.edges_ready()
    }
}

#[cfg(not(feature = "no_solver"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Phase4IndexCube {
    centers: u16,
    edge_pair: u16,
}

#[cfg(not(feature = "no_solver"))]
impl Phase4IndexCube {
    fn new(pieces: &Cube4x4x4) -> Self {
        Self {
            centers: pieces.phase_4_centers_index(),
            edge_pair: pieces.phase_4_edge_pair_index(),
        }
    }

    fn do_move(&self, mv: Move) -> Self {
        Self {
            centers: Phase4CentersMoveTable::get(self.centers, mv),
            edge_pair: Phase4EdgePairMoveTable::get(self.edge_pair, mv),
        }
    }

    fn is_phase_solved(&self) -> bool {
        self.centers == 0 && self.edge_pair == 0
    }
}

#[cfg(not(feature = "no_solver"))]
struct Solver {
    initial_state: Cube4x4x4,
    moves: Vec<Move>,
    optimal: bool,
    phase_solution: Option<Vec<Move>>,
}

#[cfg(not(feature = "no_solver"))]
impl Solver {
    fn new(cube: &Cube4x4x4, optimal: bool) -> Self {
        Self {
            initial_state: cube.clone(),
            moves: Vec::new(),
            optimal,
            phase_solution: None,
        }
    }

    fn search_phase_1(&mut self, cube: Phase1IndexCube, depth: usize) {
        // Need to go deeper. Iterate through the possible moves.
        let possible_moves = if self.moves.len() == 0 {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_1_MOVES
        } else {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_1_FOLLOWUP_MOVES
                [*self.moves.last().unwrap() as u8 as usize]
        };

        for mv in possible_moves {
            let new_cube = cube.do_move(*mv);

            // Check for solutions. This phase has multiple solved states, so fetch the
            // prune tables early to check for completion.
            let red_prune = Phase1RedCentersPruneTable::get(new_cube.red_centers);
            let orange_prune = Phase1OrangeCentersPruneTable::get(new_cube.orange_centers);
            if red_prune == 0 && orange_prune == 0 {
                let mut moves = self.moves.clone();
                moves.push(*mv);
                self.phase_solution = Some(moves);
                break;
            }

            if depth == 1 {
                continue;
            }

            // Check prune tables to see if a solution to this phase is impossible within the
            // given search depth
            if red_prune >= depth {
                continue;
            }
            if orange_prune >= depth {
                continue;
            }

            // Proceed further into phase
            self.moves.push(*mv);
            self.search_phase_1(new_cube, depth - 1);
            self.moves.pop();

            if self.phase_solution.is_some() {
                break;
            }
        }
    }

    fn search_phase_2(&mut self, cube: Phase2IndexCube, depth: usize) {
        // Need to go deeper. Iterate through the possible moves.
        let possible_moves = if self.moves.len() == 0 {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_2_MOVES
        } else {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_2_FOLLOWUP_MOVES
                [*self.moves.last().unwrap() as u8 as usize]
        };

        for mv in possible_moves {
            let new_cube = cube.do_move(*mv);

            // Check for solutions. This phase has multiple solved states, so fetch the
            // prune tables early to check for completion.
            let center_prune = Phase2CentersPruneTable::get(
                new_cube.red_orange_centers,
                new_cube.green_blue_centers,
            );
            if center_prune == 0 && new_cube.edges_ready() {
                let mut moves = self.moves.clone();
                moves.push(*mv);
                self.phase_solution = Some(moves);
                break;
            }

            if depth == 1 {
                continue;
            }

            // Check prune tables to see if a solution to this phase is impossible within the
            // given search depth
            if center_prune >= depth {
                continue;
            }

            // Proceed further into phase
            self.moves.push(*mv);
            self.search_phase_2(new_cube, depth - 1);
            self.moves.pop();

            if self.phase_solution.is_some() {
                break;
            }
        }
    }

    fn search_phase_3(&mut self, cube: Phase3IndexCube, depth: usize) {
        // Need to go deeper. Iterate through the possible moves.
        let possible_moves = if self.moves.len() == 0 {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_3_MOVES
        } else {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_3_FOLLOWUP_MOVES
                [*self.moves.last().unwrap() as u8 as usize]
        };

        for mv in possible_moves {
            let new_cube = cube.do_move(*mv);

            // Check for solutions. This phase has multiple solved states, so fetch the
            // prune tables early to check for completion.
            let center_prune = Phase3CentersPruneTable::get(
                new_cube.red_orange_centers,
                new_cube.green_blue_centers,
            );
            if center_prune == 0 && new_cube.edges_ready() {
                let mut moves = self.moves.clone();
                moves.push(*mv);
                self.phase_solution = Some(moves);
                break;
            }

            if depth == 1 {
                continue;
            }

            // Check prune tables to see if a solution to this phase is impossible within the
            // given search depth
            if center_prune >= depth {
                continue;
            }

            // Proceed further into phase
            self.moves.push(*mv);
            self.search_phase_3(new_cube, depth - 1);
            self.moves.pop();

            if self.phase_solution.is_some() {
                break;
            }
        }
    }

    fn phase_4_pll_parity(cube: Cube4x4x4) -> bool {
        // PLL parity exists if the permutation parity of the corners does not match
        // the permutation parity of the edges. First compute edge permutation parity.
        let mut edges: [u8; 12] = [0; 12];
        let mut swaps = 0;
        let mut locations: [u8; 12] = [0; 12];
        for i in 0..12 {
            edges[i] = cube.edges[i * 2].piece as u8 / 2;
        }
        for i in 0..12 {
            locations[edges[i] as usize] = i as u8;
        }
        for i in 0..12 {
            if edges[i] != i as u8 {
                locations[edges[i] as usize] = locations[i];
                edges.swap(i, locations[i] as usize);
                swaps += 1;
            }
        }
        let edge_parity = (swaps & 1) != 0;

        // Compute corner permutation parity
        let mut corners: [u8; 8] = [0; 8];
        let mut swaps = 0;
        let mut locations: [u8; 8] = [0; 8];
        for i in 0..8 {
            corners[i] = cube.corners[i].piece as u8;
        }
        for i in 0..8 {
            locations[corners[i] as usize] = i as u8;
        }
        for i in 0..8 {
            if corners[i] != i as u8 {
                locations[corners[i] as usize] = locations[i];
                corners.swap(i, locations[i] as usize);
                swaps += 1;
            }
        }
        let corner_parity = (swaps & 1) != 0;

        // PLL parity exists if parity doesn't match
        edge_parity != corner_parity
    }

    fn search_phase_4(&mut self, cube: Phase4IndexCube, depth: usize) {
        // Need to go deeper. Iterate through the possible moves.
        let possible_moves = if self.moves.len() == 0 {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_4_MOVES
        } else {
            crate::tables::solve::CUBE4_POSSIBLE_PHASE_4_FOLLOWUP_MOVES
                [*self.moves.last().unwrap() as u8 as usize]
        };

        for mv in possible_moves {
            let new_cube = cube.do_move(*mv);

            // Check for solutions. This phase has multiple solved states, so fetch the
            // prune tables early to check for completion.
            if new_cube.is_phase_solved() {
                // Check for PLL parity before accepting solution
                let mut cube = self.initial_state.clone();
                cube.do_moves(&self.moves);
                cube.do_move(*mv);
                if Self::phase_4_pll_parity(cube) {
                    continue;
                }

                let mut moves = self.moves.clone();
                moves.push(*mv);
                self.phase_solution = Some(moves);
                break;
            }

            if depth == 1 {
                continue;
            }

            // Check prune tables to see if a solution to this phase is impossible within the
            // given search depth
            if Phase4EdgePairPruneTable::get(new_cube.edge_pair) >= depth {
                continue;
            }
            if Phase4CentersPruneTable::get(new_cube.centers) >= depth {
                continue;
            }

            // Proceed further into phase
            self.moves.push(*mv);
            self.search_phase_4(new_cube, depth - 1);
            self.moves.pop();

            if self.phase_solution.is_some() {
                break;
            }
        }
    }

    fn solve(mut self) -> Option<Vec<Move>> {
        // If already solved, solution is zero moves
        if self.initial_state.is_solved() {
            return Some(Vec::new());
        }

        let mut solution = Vec::new();

        // Search for phase 1 moves and add to full solution. In this phase we try to get
        // all red and orange center pieces onto the red and orange faces.
        #[cfg(not(target_arch = "wasm32"))]
        let phase_1_start = std::time::Instant::now();

        let index_cube = Phase1IndexCube::new(&self.initial_state);
        if !index_cube.is_phase_solved() {
            let mut depth = 1;
            loop {
                self.search_phase_1(index_cube, depth);
                if self.phase_solution.is_some() {
                    break;
                }
                depth += 1;
            }
            self.initial_state
                .do_moves(&self.phase_solution.as_ref().unwrap());
            solution.append(self.phase_solution.as_mut().unwrap());
            self.phase_solution = None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let phase_1_end = std::time::Instant::now();

        // Search for phase 2 moves and add to full solution. In this phase we get red and
        // orange centers into a state that is solvable using a reduced moveset, get green/blue
        // and white/yellow centers onto the green/blue and white/yellow faces respectively,
        // separate the edges so that each edge's pair is not on the same side, eliminate
        // OLL parity, and ensure matching edge permutation parity such that the last two
        // edges will be solvable.
        #[cfg(not(target_arch = "wasm32"))]
        let phase_2_start = std::time::Instant::now();

        let index_cube = Phase2IndexCube::new(&self.initial_state);
        if !index_cube.is_phase_solved() {
            let mut depth = 1;
            loop {
                self.search_phase_2(index_cube, depth);
                if self.phase_solution.is_some() {
                    break;
                }
                depth += 1;
            }
            self.initial_state
                .do_moves(&self.phase_solution.as_ref().unwrap());
            solution.append(self.phase_solution.as_mut().unwrap());
            self.phase_solution = None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let phase_2_end = std::time::Instant::now();

        // Search for phase 3 moves and add to full solution. In this phase we get red/orange
        // and blue/green centers into vertical columns and pair up the four edge pairs on
        // the equator.
        #[cfg(not(target_arch = "wasm32"))]
        let phase_3_start = std::time::Instant::now();

        let index_cube = Phase3IndexCube::new(&self.initial_state);
        if !index_cube.is_phase_solved() {
            let mut depth = 1;
            loop {
                self.search_phase_3(index_cube, depth);
                if self.phase_solution.is_some() {
                    break;
                }
                depth += 1;
            }
            self.initial_state
                .do_moves(&self.phase_solution.as_ref().unwrap());
            solution.append(self.phase_solution.as_mut().unwrap());
            self.phase_solution = None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let phase_3_end = std::time::Instant::now();

        // Search for phase 4 moves and add to full solution. In this phase we solve the
        // centers, pair up all edges, and eliminate PLL parity.
        #[cfg(not(target_arch = "wasm32"))]
        let phase_4_start = std::time::Instant::now();

        let index_cube = Phase4IndexCube::new(&self.initial_state);
        if !index_cube.is_phase_solved() {
            let mut depth = 1;
            loop {
                self.search_phase_4(index_cube, depth);
                if self.phase_solution.is_some() {
                    break;
                }
                depth += 1;
            }
            self.initial_state
                .do_moves(&self.phase_solution.as_ref().unwrap());
            solution.append(self.phase_solution.as_mut().unwrap());
            self.phase_solution = None;
        }

        #[cfg(not(target_arch = "wasm32"))]
        let phase_4_end = std::time::Instant::now();

        // Convert the 4x4x4 to a 3x3x3 and solve the rest with the 3x3x3 solver
        #[cfg(not(target_arch = "wasm32"))]
        let phase_5_start = std::time::Instant::now();

        let cube = self.initial_state.as_faces();
        let mut colors: [Color; 6 * 9] = [Color::White; 6 * 9];
        for face in &[
            CubeFace::Top,
            CubeFace::Bottom,
            CubeFace::Left,
            CubeFace::Right,
            CubeFace::Front,
            CubeFace::Back,
        ] {
            for row in 0..4 {
                let target_row = match row {
                    0 => 0,
                    1 | 2 => 1,
                    3 => 2,
                    _ => unreachable!(),
                };
                for col in 0..4 {
                    let target_col = match col {
                        0 => 0,
                        1 | 2 => 1,
                        3 => 2,
                        _ => unreachable!(),
                    };
                    colors[Cube3x3x3Faces::idx(*face, target_row, target_col)] =
                        cube.color(*face, row, col);
                }
            }
        }
        let cube = Cube3x3x3Faces::from_colors(colors);
        let mut moves = match if self.optimal {
            cube.solve()
        } else {
            cube.solve_fast()
        } {
            Some(moves) => moves,
            None => return None,
        };
        solution.append(&mut moves);

        #[cfg(not(target_arch = "wasm32"))]
        let phase_5_end = std::time::Instant::now();

        #[cfg(not(target_arch = "wasm32"))]
        println!(
            "P1 {}ms  P2 {}ms  P3 {}ms  P4 {}ms  P5 {}ms",
            (phase_1_end - phase_1_start).as_millis(),
            (phase_2_end - phase_2_start).as_millis(),
            (phase_3_end - phase_3_start).as_millis(),
            (phase_4_end - phase_4_start).as_millis(),
            (phase_5_end - phase_5_start).as_millis()
        );

        Some(solution)
    }
}

impl Cube4x4x4 {
    pub const CORNER_ORIENTATION_INDEX_COUNT: usize =
        crate::tables::CUBE_CORNER_ORIENTATION_INDEX_COUNT;
    pub const CORNER_PERMUTATION_INDEX_COUNT: usize =
        crate::tables::CUBE_CORNER_PERMUTATION_INDEX_COUNT;
    pub const PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT;
    pub const PHASE_2_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_2_RED_ORANGE_CENTERS_INDEX_COUNT;
    pub const PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT;
    pub const PHASE_3_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_3_RED_ORANGE_CENTERS_INDEX_COUNT;
    pub const PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT;
    pub const PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT;
    pub const PHASE_4_GREEN_BLUE_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_4_GREEN_BLUE_CENTERS_INDEX_COUNT;
    pub const PHASE_4_WHITE_YELLOW_CENTERS_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_4_WHITE_YELLOW_CENTERS_INDEX_COUNT;
    pub const PHASE_4_CENTERS_INDEX_COUNT: usize = crate::tables::CUBE4_PHASE_4_CENTERS_INDEX_COUNT;
    pub const PHASE_4_EDGE_PAIR_INDEX_COUNT: usize =
        crate::tables::CUBE4_PHASE_4_EDGE_PAIR_INDEX_COUNT;

    pub fn from_corners_edges_and_centers(
        corners: [CornerPiece; 8],
        edges: [EdgePiece4x4x4; 24],
        centers: [Color; 24],
    ) -> Self {
        Self {
            corners,
            edges,
            centers,
        }
    }

    /// Gets the piece at a given corner
    pub fn corner_piece(&self, corner: Corner) -> CornerPiece {
        self.corners[corner as u8 as usize]
    }

    /// Gets the piece at a given edge
    pub fn edge_piece(&self, edge: Edge4x4x4) -> EdgePiece4x4x4 {
        self.edges[edge as u8 as usize]
    }

    pub fn center_color(&self, face: CubeFace, row: usize, col: usize) -> Color {
        self.centers[Self::center_idx(face, row, col)]
    }

    pub(crate) const fn center_idx(face: CubeFace, row: usize, col: usize) -> usize {
        face as u8 as usize * 4 + row * 2 + col
    }

    pub fn oll_parity(&self) -> bool {
        // OLL parity exists if the number of flipped edges is not a multiple of 4.
        // It is always a multiple of 2, so check bit 1 of the sum of orientations.
        let mut result = 0;
        for edge in &self.edges {
            result += edge.orientation;
        }
        result & 2 != 0
    }

    /// Index for the corner orientations is a simple base 3 integer representation. The
    /// zero index is the solved state. Note that the last corner is not represented in
    /// the index as its value is implicit (all corner orientations must add to a
    /// multiple of 3).
    pub fn corner_orientation_index(&self) -> u16 {
        let mut result = 0;
        for i in 0..7 {
            result = (result * 3) + self.corners[i].orientation as u16;
        }
        result
    }

    /// Index for the corner permutations is the representation of the state in the
    /// factorial number system (each digit in the number decreases in base, with the
    /// digits representing the index of the choice in the remaining possible choices).
    pub fn corner_permutation_index(&self) -> u16 {
        let mut result = 0;
        for i in 0..7 {
            // Get index in set of remaining options by checking how many of the entries
            // are greater than this one (which is the index in the sorted list of
            // remaining options)
            let mut cur = 0;
            for j in i + 1..8 {
                if self.corners[i].piece as u8 > self.corners[j].piece as u8 {
                    cur += 1;
                }
            }
            result = (result + cur) * (7 - i as u16);
        }
        result
    }

    /// Find the positions of the red centers. For this index, it does not
    /// matter what order these pieces are in as they are not identified specifically.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the red centers on the red face have
    /// positions 0-3 (this allows the zero index to represent the fully solved state).
    pub fn phase_1_red_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 4] = [0; 4];
        let mut j = 0;

        for i in 0..24 {
            if self.centers[(i + CubeFace::Right as u8 as usize * 4) % 24] == Color::Red {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(24, 4).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)) as u16
    }

    /// Find the positions of the orange centers. For this index, it does not
    /// matter what order these pieces are in as they are not identified specifically.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the orange centers on the orange face have
    /// positions 0-3 (this allows the zero index to represent the fully solved state).
    pub fn phase_1_orange_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 4] = [0; 4];
        let mut j = 0;

        for i in 0..24 {
            if self.centers[(i + CubeFace::Left as u8 as usize * 4) % 24] == Color::Orange {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(24, 4).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)) as u16
    }

    /// Find the positions of the red and orange centers. It is known at this point that
    /// there are only 8 valid locations for these centers, as they were placed on the
    /// red and orange faces during phase 1. Since we know that pieces that are not red
    /// are orange, we need only represent which pieces are red. For this index, it does
    /// not matter what order these pieces are in as they are not identified specifically.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the red centers on the red face have positions 0-3
    /// (this allows the zero index to represent the fully solved state).
    pub fn phase_2_and_3_red_orange_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 4] = [0; 4];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Right, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Right, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Right, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Right, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Left, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Left, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Left, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Left, 1, 1),
        ];
        for i in 0..8 {
            if self.centers[POSITIONS[i]] == Color::Red {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(8, 4).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)) as u16
    }

    /// Find the positions of the green and blue centers. We know the green and blue
    /// pieces cannot be on the red or orange faces after phase 1 is complete, so the
    /// number of possible locations for these centers is 16. For this index, it does not
    /// matter what order these pieces are in as they are not identified specifically, and
    /// we do not identify the exact color, but rather whether it is green or blue.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the centers on the correct faces have
    /// positions 0-7 (this allows the zero index to represent the fully solved state).
    pub fn phase_2_green_blue_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 8] = [0; 8];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Front, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Front, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Front, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Front, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Back, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Back, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Top, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Top, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Top, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Top, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Bottom, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Bottom, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Bottom, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Bottom, 1, 1),
        ];
        for i in 0..16 {
            if self.centers[POSITIONS[i]] == Color::Green
                || self.centers[POSITIONS[i]] == Color::Blue
            {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(16, 8).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)
            + crate::common::n_choose_k(center_piece_pos[4], 5)
            + crate::common::n_choose_k(center_piece_pos[5], 6)
            + crate::common::n_choose_k(center_piece_pos[6], 7)
            + crate::common::n_choose_k(center_piece_pos[7], 8)) as u16
    }

    /// Find the positions of the green and blue centers. It is known at this point that
    /// there are only 8 valid locations for these centers, as they were placed on the
    /// green and blue faces during phase 2. Since we know that pieces that are not green
    /// are blue, we need only represent which pieces are green. For this index, it does
    /// not matter what order these pieces are in as they are not identified specifically.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the green centers on the green face have positions 0-3
    /// (this allows the zero index to represent the fully solved state).
    pub fn phase_3_green_blue_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 4] = [0; 4];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Front, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Front, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Front, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Front, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Back, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Back, 1, 1),
        ];
        for i in 0..8 {
            if self.centers[POSITIONS[i]] == Color::Green {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(8, 4).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)) as u16
    }

    /// Find the positions of the red and orange centers. It is known at this point that
    /// these centers are layed out in vertical columns after phase 3. We can use only
    /// half of the center pieces since the other half is the same. Since we also know that the
    /// pieces that are not red are orange, we need only represent the pieces that are red,
    /// of which there will only be two. For this index, it does not matter what order these
    /// pieces are in as they are not identified specifically. This allows the index to be
    /// generated using the combinatorial number system. Represent in a way that having the
    /// red centers on the red face have positions 0 and 1 (this allows the zero index to
    /// represent the fully solved state).
    pub fn phase_4_red_orange_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 2] = [0; 2];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Right, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Right, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Left, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Left, 0, 1),
        ];
        for i in 0..4 {
            if self.centers[POSITIONS[i]] == Color::Red {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(4, 2).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)) as u16
    }

    /// Find the positions of the green and blue centers. It is known at this point that
    /// these centers are layed out in vertical columns after phase 3. We can use only
    /// half of the center pieces since the other half is the same. Since we also know that the
    /// pieces that are not green are blue, we need only represent the pieces that are green,
    /// of which there will only be two. For this index, it does not matter what order these
    /// pieces are in as they are not identified specifically. This allows the index to be
    /// generated using the combinatorial number system. Represent in a way that having the
    /// green centers on the green face have positions 0 and 1 (this allows the zero index to
    /// represent the fully solved state).
    pub fn phase_4_green_blue_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 2] = [0; 2];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Front, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Front, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Back, 0, 1),
        ];
        for i in 0..4 {
            if self.centers[POSITIONS[i]] == Color::Green {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(4, 2).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)) as u16
    }

    /// Find the positions of the white and yellow centers. It is known at this point that
    /// there are only 8 valid locations for these centers, as they were placed on the
    /// white and yellow faces during phase 2. Since we know that pieces that are not white
    /// are yellow, we need only represent which pieces are white. For this index, it does
    /// not matter what order these pieces are in as they are not identified specifically.
    /// This allows the index to be generated using the combinatorial number system.
    /// Represent in a way that having the white centers on the white face have positions 0-3
    /// (this allows the zero index to represent the fully solved state).
    pub fn phase_4_white_yellow_centers_index(&self) -> u16 {
        let mut center_piece_pos: [usize; 4] = [0; 4];
        let mut j = 0;

        const POSITIONS: &'static [usize] = &[
            Cube4x4x4::center_idx(CubeFace::Top, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Top, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Top, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Top, 1, 1),
            Cube4x4x4::center_idx(CubeFace::Bottom, 0, 0),
            Cube4x4x4::center_idx(CubeFace::Bottom, 0, 1),
            Cube4x4x4::center_idx(CubeFace::Bottom, 1, 0),
            Cube4x4x4::center_idx(CubeFace::Bottom, 1, 1),
        ];
        for i in 0..8 {
            if self.centers[POSITIONS[i]] == Color::White {
                center_piece_pos[j] = i;
                j += 1;
            }
        }

        // Compute an index using the combinatorial number system. This will be an integer
        // between zero (solved) and n_choose_k(8, 4).
        (crate::common::n_choose_k(center_piece_pos[0], 1)
            + crate::common::n_choose_k(center_piece_pos[1], 2)
            + crate::common::n_choose_k(center_piece_pos[2], 3)
            + crate::common::n_choose_k(center_piece_pos[3], 4)) as u16
    }

    /// Find the positions of all centers during phase 4. The centers are split into three
    /// groups of two faces, with four of the faces already in vertical columns. This
    /// significantly reduces the size of the index space. We will simply combine the
    /// simpler index for each pair of faces into a larger index.
    pub fn phase_4_centers_index(&self) -> u16 {
        self.phase_4_white_yellow_centers_index()
            * Cube4x4x4::PHASE_4_GREEN_BLUE_CENTERS_INDEX_COUNT as u16
            * Cube4x4x4::PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT as u16
            + self.phase_4_green_blue_centers_index()
                * Cube4x4x4::PHASE_4_RED_ORANGE_CENTERS_INDEX_COUNT as u16
            + self.phase_4_red_orange_centers_index()
    }

    /// Index for the edge pairing is the representation of the state in the
    /// factorial number system (each digit in the number decreases in base, with the
    /// digits representing the index of the choice in the remaining possible choices).
    /// This is the phase 4 edge pairing index, which does not include the edges
    /// in the equatorial slice (these were solved in phase 3).
    pub fn phase_4_edge_pair_index(&self) -> u16 {
        // The assigned edges have all non-equatorial edges in the first 16 entries
        let mut edge_pair_index: [u8; 8] = [0; 8];
        let mut odd_edges_from: [u8; 12] = [0; 12];
        for i in 0..8 {
            odd_edges_from[self.edges[i * 2 + 1].piece as u8 as usize / 2] = i as u8;
        }
        for i in 0..8 {
            edge_pair_index[i] = odd_edges_from[self.edges[i * 2].piece as u8 as usize / 2];
        }

        let mut result = 0;
        for i in 0..7 {
            // Get index in set of remaining options by checking how many of the entries
            // are greater than this one (which is the index in the sorted list of
            // remaining options)
            let mut cur = 0;
            for j in i + 1..8 {
                if edge_pair_index[i] as u8 > edge_pair_index[j] as u8 {
                    cur += 1;
                }
            }
            result = (result + cur) * (7 - i as u16);
        }
        result
    }

    /// Determine if all red center pieces are on the red or orange face of the cube
    pub fn red_centers_on_red_or_orange_face(&self) -> bool {
        let mut count = 0;
        for face in &[CubeFace::Left, CubeFace::Right] {
            for row in 0..2 {
                for col in 0..2 {
                    if self.center_color(*face, row, col) == Color::Red {
                        count += 1;
                    }
                }
            }
        }
        count == 4
    }

    /// Determine if all orange center pieces are on the red or orange face of the cube
    pub fn orange_centers_on_red_or_orange_face(&self) -> bool {
        let mut count = 0;
        for face in &[CubeFace::Left, CubeFace::Right] {
            for row in 0..2 {
                for col in 0..2 {
                    if self.center_color(*face, row, col) == Color::Orange {
                        count += 1;
                    }
                }
            }
        }
        count == 4
    }

    /// Determine if the red and orange center pieces are in a configuration that can be
    /// solved with the moveset (R2 L2 F B U D Rw2 Lw2 Fw2 Bw2 Uw2 Dw2). The blue and
    /// green centers must also be on the blue and green face.
    pub fn phase_2_centers_solved(&self) -> bool {
        if self.phase_2_green_blue_centers_index() != 0 {
            return false;
        }
        self.center_color(CubeFace::Right, 0, 0) == self.center_color(CubeFace::Right, 1, 0)
            && self.center_color(CubeFace::Right, 0, 1) == self.center_color(CubeFace::Right, 1, 1)
            && self.center_color(CubeFace::Left, 0, 0) == self.center_color(CubeFace::Left, 1, 0)
            && self.center_color(CubeFace::Left, 0, 1) == self.center_color(CubeFace::Left, 1, 1)
    }

    /// Determine if the red, orange, green, and blue center pieces are in a configuration
    /// that can be solved with the moveset (R2 L2 F2 B2 U D Rw2 Lw2 Fw2 Bw2 Uw2 Dw2).
    pub fn phase_3_centers_solved(&self) -> bool {
        self.center_color(CubeFace::Right, 0, 0) == self.center_color(CubeFace::Right, 1, 0)
            && self.center_color(CubeFace::Right, 0, 1) == self.center_color(CubeFace::Right, 1, 1)
            && self.center_color(CubeFace::Left, 0, 0) == self.center_color(CubeFace::Left, 1, 0)
            && self.center_color(CubeFace::Left, 0, 1) == self.center_color(CubeFace::Left, 1, 1)
            && self.center_color(CubeFace::Front, 0, 0) == self.center_color(CubeFace::Front, 1, 0)
            && self.center_color(CubeFace::Front, 0, 1) == self.center_color(CubeFace::Front, 1, 1)
            && self.center_color(CubeFace::Back, 0, 0) == self.center_color(CubeFace::Back, 1, 0)
            && self.center_color(CubeFace::Back, 0, 1) == self.center_color(CubeFace::Back, 1, 1)
    }

    /// Gets this cube state in face color format
    pub fn as_faces(&self) -> Cube4x4x4Faces {
        let mut faces = Cube4x4x4Faces::new();

        // Translate corner pieces into face colors
        for corner_idx in 0..8 {
            let piece = self.corners[corner_idx];
            for i in 0..3 {
                let dest = crate::tables::corner::CUBE4_CORNER_INDICIES[corner_idx][i];
                let src = crate::tables::corner::CUBE4_CORNER_INDICIES[piece.piece as u8 as usize]
                    [(i + 3 - piece.orientation as usize) % 3];
                let face = Cube4x4x4Faces::face_for_idx(src);
                faces.state[dest] = face.color();
            }
        }

        // Translate edge pieces into face colors
        for edge_idx in 0..24 {
            let piece = self.edges[edge_idx];
            for i in 0..2 {
                let dest = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES[edge_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [piece.piece as u8 as usize][i ^ piece.orientation as usize];
                let face = Cube4x4x4Faces::face_for_idx(src);
                faces.state[dest] = face.color();
            }
        }

        // Set center piece colors
        for center_idx in 0..24 {
            let face = CubeFace::try_from((center_idx / 4) as u8).unwrap();
            let row = (center_idx / 2) % 2;
            let col = center_idx % 2;
            faces.state[Cube4x4x4Faces::idx(face, row + 1, col + 1)] =
                self.center_color(face, row, col);
        }

        faces
    }
}

impl FaceRotation for Cube4x4x4 {
    fn rotate_wide(&mut self, face: CubeFace, dir: RotationDirection, width: usize) {
        let face_idx = face as u8 as usize;
        let dir_idx = dir as u8 as usize;

        // Save existing cube state so that it can be looked up during rotation
        let old_corners = self.corners;
        let old_edges = self.edges;
        let old_centers = self.centers;

        // Apply corner movement using lookup table
        for i in 0..4 {
            let (dest, src) =
                crate::tables::corner::CUBE_CORNER_PIECE_ROTATION[dir_idx][face_idx][i];
            self.corners[dest as u8 as usize] = CornerPiece {
                piece: old_corners[src.piece as u8 as usize].piece,
                orientation: (old_corners[src.piece as u8 as usize].orientation + src.orientation)
                    % 3,
            };
        }

        // Apply outer edge movement using lookup table
        for i in 0..8 {
            let (dest, src) =
                crate::tables::table4x4x4::CUBE4_EDGE_PIECE_ROTATION[dir_idx][face_idx][i];
            self.edges[dest as u8 as usize] = EdgePiece4x4x4 {
                piece: old_edges[src.piece as u8 as usize].piece,
                orientation: (old_edges[src.piece as u8 as usize].orientation ^ src.orientation),
            };
        }

        // Apply center movement using lookup table
        for i in 0..4 {
            let src = crate::tables::table4x4x4::CUBE4_CENTER_PIECE_ROTATION[dir_idx][i];
            self.centers[face_idx * 4 + i] = old_centers[face_idx * 4 + src];
        }

        if width == 2 {
            // Wide move, apply inner slice edge movement using lookup table. Inner edge pieces
            // always invert orientation no matter which face (this is an invariant that must
            // be true for the edge piece indexes to work properly).
            let edge_offset = match dir {
                RotationDirection::CW => 3,
                RotationDirection::CCW => 1,
            };
            for i in 0..4 {
                let dest = crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx]
                    [(i + edge_offset) % 4];
                self.edges[dest as u8 as usize] = EdgePiece4x4x4 {
                    piece: old_edges[src as u8 as usize].piece,
                    orientation: old_edges[src as u8 as usize].orientation ^ 1,
                };
            }

            // Apply inner slice center movement using lookup table
            let center_offset = match dir {
                RotationDirection::CW => 6,
                RotationDirection::CCW => 2,
            };
            for i in 0..8 {
                let dest =
                    crate::tables::table4x4x4::CUBE4_SLICED_CENTER_PIECE_ROTATION[face_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_SLICED_CENTER_PIECE_ROTATION[face_idx]
                    [(i + center_offset) % 8];
                self.centers[dest] = old_centers[src];
            }
        }
    }
}

impl InitialCubeState for Cube4x4x4 {
    fn new() -> Self {
        let mut corners = [CornerPiece {
            piece: Corner::URF,
            orientation: 0,
        }; 8];
        for i in 0..8 {
            corners[i].piece = Corner::try_from(i as u8).unwrap();
        }

        let mut edges = [EdgePiece4x4x4 {
            piece: Edge4x4x4::URB,
            orientation: 0,
        }; 24];
        for i in 0..24 {
            edges[i].piece = Edge4x4x4::try_from(i as u8).unwrap();
        }

        let mut centers = [Color::White; 24];
        for i in 0..24 {
            centers[i] = CubeFace::try_from((i / 4) as u8).unwrap().color();
        }

        Self {
            corners,
            edges,
            centers,
        }
    }

    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self {
        let mut cube = Self::new();

        for _ in 0..400 {
            cube.do_move(Move::try_from(rng.next(Move::count_4x4x4() as u32) as u8).unwrap());
        }

        // FIXME: Implement the below correctly
        // Randomize the corner pieces
        /*        for i in 0..7 {
            let n = rng.next(8) as usize;
            if i != n {
                // Must swap two corners at a time to avoid parity violation
                cube.corners.swap(i, n);
                cube.corners.swap(6, 7);
            }
        }

        // Randomize the edge pieces
        for i in 0..23 {
            let n = rng.next(24) as usize;
            if i != n {
                // Must swap two edges at a time to avoid parity violation
                cube.edges.swap(i, n);
                cube.edges.swap(22, 23);
            }
        }

        // Randomize the center pieces
        for i in 0..23 {
            let n = rng.next(24) as usize;
            if i != n {
                // Must swap two edges at a time to avoid parity violation
                cube.centers.swap(i, n);
                cube.centers.swap(22, 23);
            }
        }

        // Randomize the corner orientations
        let mut corner_orientation_sum = 0;
        for i in 0..7 {
            cube.corners[i].orientation = rng.next(3) as u8;
            corner_orientation_sum += cube.corners[i].orientation;
        }

        // Randomize the edge orientations
        let mut edge_orientation_sum = 0;
        for i in 0..23 {
            cube.edges[i].orientation = rng.next(2) as u8;
            edge_orientation_sum += cube.edges[i].orientation;
        }

        // Make sure all corner orientations add up to a multiple of 3 (otherwise it is not solvable)
        cube.corners[7].orientation = (3 - (corner_orientation_sum % 3)) % 3;

        // Make sure all edge orientations add up to a multiple of 2 (otherwise it is not solvable)
        cube.edges[23].orientation = (2 - (edge_orientation_sum % 2)) % 2;*/

        cube
    }
}

impl Cube for Cube4x4x4 {
    fn is_solved(&self) -> bool {
        // Check corners
        for i in 0..8 {
            let correct_piece = CornerPiece {
                piece: Corner::try_from(i as u8).unwrap(),
                orientation: 0,
            };
            if self.corners[i] != correct_piece {
                return false;
            }
        }

        // Check edges
        for i in 0..24 {
            let correct_piece = EdgePiece4x4x4 {
                piece: Edge4x4x4::try_from(i as u8).unwrap(),
                orientation: 0,
            };
            if self.edges[i] != correct_piece {
                return false;
            }
        }

        // Check centers
        for i in 0..24 {
            if self.centers[i] != CubeFace::try_from((i / 4) as u8).unwrap().color() {
                return false;
            }
        }
        true
    }

    fn do_move(&mut self, mv: Move) {
        self.rotate_counted_wide(mv.face(), mv.rotation(), mv.width());
    }

    fn size(&self) -> usize {
        4
    }

    fn colors(&self) -> BTreeMap<CubeFace, Vec<Vec<Color>>> {
        self.as_faces().colors()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>> {
        Solver::new(self, true).solve()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>> {
        Solver::new(self, false).solve()
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn dyn_clone(&self) -> Box<dyn Cube> {
        Box::new(self.clone())
    }
}

impl Cube4x4x4Faces {
    /// Create a cube state from a color array. The ordering of the array is the faces
    /// in order of the `Face` enumeration, with 9 elements per face. Each face is stored
    /// from top to bottom in row major order, with columns left to right.
    pub fn from_colors(state: [Color; 6 * 16]) -> Self {
        Self { state }
    }

    pub(crate) const fn face_start(face: CubeFace) -> usize {
        face as u8 as usize * 16
    }

    pub(crate) const fn face_offset(row: usize, col: usize) -> usize {
        (row * 4) + col
    }

    pub(crate) const fn idx(face: CubeFace, row: usize, col: usize) -> usize {
        Self::face_start(face) + Self::face_offset(row, col)
    }

    pub(crate) fn face_for_idx(idx: usize) -> CubeFace {
        CubeFace::try_from((idx / 16) as u8).unwrap()
    }

    /// Gets the color for a given place on the cube. For a given `face`, the `row` and
    /// `col` represent the zero-indexed position on the face to be accessed.
    pub fn color(&self, face: CubeFace, row: usize, col: usize) -> Color {
        self.state[Self::idx(face, row, col)]
    }

    /// Gets the color for a given place on the cube. For a given `face`, the `row` and
    /// `col` represent the zero-indexed position on the face to be accessed.
    pub(crate) fn color_by_idx(&self, idx: usize) -> Color {
        self.state[idx]
    }

    /// Gets the color of a specific corner (there are three colors per corner)
    pub fn corner_color(&self, corner: Corner, idx: usize) -> Color {
        self.state[crate::tables::corner::CUBE4_CORNER_INDICIES[corner as u8 as usize][idx]]
    }

    /// Gets the color of a specific edge (there are two colors per edge)
    pub fn edge_color(&self, edge: Edge4x4x4, idx: usize) -> Color {
        self.state[crate::tables::table4x4x4::CUBE4_EDGE_INDICIES[edge as u8 as usize][idx]]
    }

    /// Gets the color of a specific center piece
    pub fn center_color(&self, center: CubeFace, row: usize, col: usize) -> Color {
        self.color_by_idx(Self::idx(center, row + 1, col + 1))
    }

    /// Gets this cube state in piece format
    pub fn as_pieces(&self) -> Cube4x4x4 {
        let mut pieces = Cube4x4x4::new();

        for corner_idx in 0..8 {
            let corner_colors: [Color; 3] = [
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 0),
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 1),
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 2),
            ];
            // Find this corner piece and orientation
            for i in 0..8 {
                if corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 0,
                    };
                    break;
                } else if corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 1,
                    };
                    break;
                } else if corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 2,
                    };
                    break;
                }
            }
        }

        for edge_idx in 0..24 {
            let edge_colors: [Color; 2] = [
                self.edge_color(Edge4x4x4::try_from(edge_idx).unwrap(), 0),
                self.edge_color(Edge4x4x4::try_from(edge_idx).unwrap(), 1),
            ];
            // Find this edge piece and orientation. Which of the two possible edges for the color
            // combination is based on whether we are looking for an odd indexed slot, and whether
            // the detected colors are flipped. This is a simple XOR of the two conditions to
            // determine the lowest bit of the edge index.
            for i in 0..12 {
                if edge_colors[0] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][0]
                    && edge_colors[1] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][1]
                {
                    pieces.edges[edge_idx as usize] = EdgePiece4x4x4 {
                        piece: Edge4x4x4::try_from((i * 2) as u8 ^ (edge_idx as u8 & 1)).unwrap(),
                        orientation: 0,
                    };
                    break;
                } else if edge_colors[1] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][0]
                    && edge_colors[0] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][1]
                {
                    pieces.edges[edge_idx as usize] = EdgePiece4x4x4 {
                        piece: Edge4x4x4::try_from((i * 2) as u8 ^ (edge_idx as u8 & 1) ^ 1)
                            .unwrap(),
                        orientation: 1,
                    };
                    break;
                }
            }
        }

        for center_idx in 0..6 {
            pieces.centers[center_idx * 4] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 0, 0);
            pieces.centers[center_idx * 4 + 1] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 0, 1);
            pieces.centers[center_idx * 4 + 2] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 1, 0);
            pieces.centers[center_idx * 4 + 3] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 1, 1);
        }

        pieces
    }
}

impl FaceRotation for Cube4x4x4Faces {
    fn rotate_wide(&mut self, face: CubeFace, dir: RotationDirection, width: usize) {
        let face_idx = face as u8 as usize;
        let dir_idx = dir as u8 as usize;

        // Rotate colors on face itself
        let mut rotated_colors: [Color; 16] = [Color::White; 16];
        for i in 0..16 {
            rotated_colors[i] = self.state[Self::face_start(face)
                + crate::tables::table4x4x4::CUBE4_FACE_ROTATION[dir_idx][i]];
        }
        for i in 0..16 {
            self.state[Self::face_start(face) + i] = rotated_colors[i];
        }

        // Collect colors on edges and corners
        let mut adjacent_corner_colors: [[Color; 2]; 4] = [[Color::White; 2]; 4];
        for i in 0..4 {
            adjacent_corner_colors[i][0] =
                self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][i][0]];
            adjacent_corner_colors[i][1] =
                self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][i][1]];
        }

        let mut adjacent_edge_colors: [Color; 8] = [Color::White; 8];
        for i in 0..8 {
            adjacent_edge_colors[i] =
                self.state[crate::tables::table4x4x4::CUBE4_EDGE_ADJACENCY[face_idx][i]];
        }

        // Rotate colors on corners
        for i in 0..4 {
            let j = crate::tables::corner::CUBE_CORNER_ROTATION[dir_idx][i];
            self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][j][0]] =
                adjacent_corner_colors[i][0];
            self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][j][1]] =
                adjacent_corner_colors[i][1];
        }

        // Rotate colors on edges
        for i in 0..8 {
            let j = crate::tables::table4x4x4::CUBE4_EDGE_ROTATION[dir_idx][i];
            self.state[crate::tables::table4x4x4::CUBE4_EDGE_ADJACENCY[face_idx][j]] =
                adjacent_edge_colors[i];
        }

        if width == 2 {
            // Wide move, apply inner slice edge movement using lookup table. Inner edge pieces
            // always invert orientation no matter which face (this is an invariant that must
            // be true for the edge piece indexes to work properly).
            let mut slice_edge_colors: [[Color; 2]; 4] = [[Color::White; 2]; 4];
            let edge_offset = match dir {
                RotationDirection::CW => 3,
                RotationDirection::CCW => 1,
            };
            for i in 0..4 {
                let src = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i] as u8
                        as usize];
                for j in 0..2 {
                    slice_edge_colors[i][j] = self.state[src[j]];
                }
            }
            for i in 0..4 {
                let dest = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i] as u8
                        as usize];
                for j in 0..2 {
                    self.state[dest[j]] = slice_edge_colors[(i + edge_offset) % 4][j ^ 1];
                }
            }

            // Apply inner slice center movement using lookup table
            let mut slice_center_colors: [Color; 16] = [Color::White; 16];
            let center_offset = match dir {
                RotationDirection::CW => 6,
                RotationDirection::CCW => 2,
            };
            for i in 0..8 {
                slice_center_colors[i] = self.state
                    [crate::tables::table4x4x4::CUBE4_SLICED_CENTER_FACE_ROTATION[face_idx][i]];
            }
            for i in 0..8 {
                self.state
                    [crate::tables::table4x4x4::CUBE4_SLICED_CENTER_FACE_ROTATION[face_idx][i]] =
                    slice_center_colors[(i + center_offset) % 8];
            }
        }
    }
}

impl InitialCubeState for Cube4x4x4Faces {
    fn new() -> Self {
        let mut state = [Color::White; 6 * 16];
        for i in 0..16 {
            state[Self::face_start(CubeFace::Top) + i] = Color::White;
            state[Self::face_start(CubeFace::Front) + i] = Color::Green;
            state[Self::face_start(CubeFace::Right) + i] = Color::Red;
            state[Self::face_start(CubeFace::Back) + i] = Color::Blue;
            state[Self::face_start(CubeFace::Left) + i] = Color::Orange;
            state[Self::face_start(CubeFace::Bottom) + i] = Color::Yellow;
        }
        Self { state }
    }

    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self {
        Cube4x4x4::sourced_random(rng).as_faces()
    }
}

impl Cube for Cube4x4x4Faces {
    fn is_solved(&self) -> bool {
        for face in 0..6 {
            let face = CubeFace::try_from(face).unwrap();
            for i in 0..16 {
                if self.state[Self::face_start(face) + i] != face.color() {
                    return false;
                }
            }
        }
        true
    }

    fn do_move(&mut self, mv: Move) {
        self.rotate_counted_wide(mv.face(), mv.rotation(), mv.width());
    }

    fn size(&self) -> usize {
        4
    }

    fn colors(&self) -> BTreeMap<CubeFace, Vec<Vec<Color>>> {
        let mut result = BTreeMap::new();
        for face in &[
            CubeFace::Top,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Left,
            CubeFace::Bottom,
        ] {
            let mut rows = Vec::new();
            for row in 0..4 {
                let mut cols = Vec::new();
                for col in 0..4 {
                    cols.push(self.color(*face, row, col));
                }
                rows.push(cols);
            }
            result.insert(*face, rows);
        }
        result
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>> {
        self.as_pieces().solve()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>> {
        self.as_pieces().solve_fast()
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn dyn_clone(&self) -> Box<dyn Cube> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for Cube4x4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_faces().fmt(f)
    }
}

impl std::fmt::Display for Cube4x4x4Faces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_state: [[char; 17]; 12] = [[' '; 17]; 12];
        const FACE_X: [usize; 6] = [4, 4, 8, 12, 0, 4];
        const FACE_Y: [usize; 6] = [0, 4, 4, 4, 4, 8];
        for face_idx in 0..6 {
            for row in 0..4 {
                for col in 0..4 {
                    let ch = match self.state
                        [Self::idx(CubeFace::try_from(face_idx).unwrap(), row, col)]
                    {
                        Color::White => 'W',
                        Color::Green => 'G',
                        Color::Red => 'R',
                        Color::Blue => 'B',
                        Color::Orange => 'O',
                        Color::Yellow => 'Y',
                    };
                    debug_state[FACE_Y[face_idx as usize] + row][FACE_X[face_idx as usize] + col] =
                        ch;
                }
            }
        }
        for row in 0..12 {
            let s: String = debug_state[row].iter().collect();
            write!(f, "{}\n", s)?;
        }
        Ok(())
    }
}

/// Generates a random scramble
#[cfg(not(feature = "no_solver"))]
pub fn scramble_4x4x4() -> Vec<Move> {
    let state = Cube4x4x4::random();
    let solution = state.solve().unwrap();
    solution.inverse()
}

/// Generates a random scramble very fast, but with more moves required than normal
#[cfg(not(feature = "no_solver"))]
pub fn scramble_4x4x4_fast() -> Vec<Move> {
    let state = Cube4x4x4::random();
    let solution = state.solve_fast().unwrap();
    solution.inverse()
}
