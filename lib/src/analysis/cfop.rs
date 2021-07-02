use crate::tables::{
    CUBE3_EDGE_ADJACENCY, CUBE3_F2L_PAIRS, CUBE3_LAST_LAYER_EDGE, CUBE3_OLL_CASES, CUBE3_PLL_CASES,
};
use crate::{
    cube3x3x3::FaceRowOrColumn, Color, Cube, Cube3x3x3Faces, CubeWithSolution, Face, Move,
    MoveSequence, TimedMove,
};

/// Analysis of a full solve using CFOP method. Both one-look and two-look
/// are fully supported automatically.
#[derive(Clone)]
pub struct CFOPAnalysis {
    pub cross: CrossAnalysis,
    pub f2l_pairs: Vec<F2LPairAnalysis>,
    pub oll: Vec<OLLAnalysis>,
    pub pll: Vec<PLLAnalysis>,
    pub alignment: FinalAlignmentAnalysis,
}

/// Partial analysis of a cube solution. This analysis can be performed on
/// an incomplete solve to get the current progress.
#[derive(Clone)]
pub struct CFOPPartialAnalysis {
    pub progress: CFOPProgress,
    pub cross: Option<CrossAnalysis>,
    pub f2l_pairs: Vec<F2LPairAnalysis>,
    pub oll: Vec<OLLAnalysis>,
    pub pll: Vec<PLLAnalysis>,
    pub alignment: Option<FinalAlignmentAnalysis>,
}

/// Analysis of the cross phase of a CFOP solution.
#[derive(Clone)]
pub struct CrossAnalysis {
    /// Color of the first layer cross
    pub color: Color,
    /// Time spent solving the first layer cross
    pub time: u32,
    /// Moves performed
    pub moves: Vec<Move>,
}

/// Analysis of a single F2L pair insertion in a CFOP solution.
#[derive(Clone)]
pub struct F2LPairAnalysis {
    /// Time spent recognizing the state
    pub recognition_time: u32,
    /// Time spent inserting the pair
    pub execution_time: u32,
    /// Move index of the start of this pair insertion
    pub start_move_index: usize,
    /// Moves performed
    pub moves: Vec<Move>,
}

/// Analysis of an OLL algorithm performance. There may be more than
/// one of these in a solve in the case of a two-look solution, a
/// choice of algorithm that goes through multiple states, or a
/// mistake during solution.
#[derive(Clone)]
pub struct OLLAnalysis {
    /// The algorithm that needs to be performed to solve OLL in a single step
    pub one_look_algorithm: OLLAlgorithm,
    /// The actual algorithm that was performed
    pub performed_algorithm: OLLAlgorithm,
    /// The state after the algorithm. If `None`, OLL is solved. Otherwise, contains
    /// the same value as `one_look_algorithm` for the next phase of the OLL solution.
    pub new_state: Option<OLLAlgorithm>,
    /// Time spent recognizing the state
    pub recognition_time: u32,
    /// Time spent executing the algorithm
    pub execution_time: u32,
    /// Move index of the start of the algorithm
    pub start_move_index: usize,
    /// Moves performed
    pub moves: Vec<Move>,
}

/// OLL algorithm used during solve. Two-look algorithms are named and
/// others use their standard number.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OLLAlgorithm {
    H,
    Pi,
    U,
    T,
    L,
    Antisune,
    Sune,
    OLL(u8),
}

/// Analysis of an PLL algorithm performance. There may be more than
/// one of these in a solve in the case of a two-look solution, a
/// choice of algorithm that goes through multiple states, or a
/// mistake during solution.
#[derive(Clone)]
pub struct PLLAnalysis {
    /// The algorithm that needs to be performed to solve the cube in a single step
    pub one_look_algorithm: PLLAlgorithm,
    /// The actual algorithm that was performed
    pub performed_algorithm: PLLAlgorithm,
    /// The state after the algorithm. If `None`, the cube is solved. Otherwise, contains
    /// the same value as `one_look_algorithm` for the next phase of the PLL solution.
    pub new_state: Option<PLLAlgorithm>,
    /// Time spent recognizing the state
    pub recognition_time: u32,
    /// Time spent executing the algorithm
    pub execution_time: u32,
    /// Move index of the start of the algorithm
    pub start_move_index: usize,
    /// Moves performed
    pub moves: Vec<Move>,
}

/// PLL algorithm used during solve.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PLLAlgorithm {
    Aa,
    Ab,
    F,
    Ga,
    Gb,
    Gc,
    Gd,
    Ja,
    Jb,
    Ra,
    Rb,
    T,
    E,
    Na,
    Nb,
    V,
    Y,
    H,
    Ua,
    Ub,
    Z,
}

/// Analysis of the final alignment of the last layer. This is after the PLL algorithm
/// is completed and one or more face rotations are needed to finish the solve. These
/// fields may be zero if the cube was solved directly after the PLL algorithm.
#[derive(Clone)]
pub struct FinalAlignmentAnalysis {
    /// Time spent aligning the last layer
    pub time: u32,
    /// Move index of the start of the algorithm
    pub start_move_index: usize,
    /// Moves performed
    pub moves: Vec<Move>,
}

/// State of the cube as it's being solved with CFOP method
#[derive(Clone, PartialEq, Eq)]
pub enum CFOPProgress {
    /// No progress on solve
    Initial,
    /// Cross on first layer complete, solving first two layers. Current number of
    /// F2L pairs inserted is given.
    F2LPair(usize),
    /// First two layers are solved. Current OLL algorithm required to orient the
    /// last layer is given.
    OLL(OLLAlgorithm),
    /// Last layer is oriented. Current PLL algorithm required to solve the cube
    /// is given.
    PLL(PLLAlgorithm),
    /// All layers are solved but last layer is not yet aligned
    FinalAlignment,
    /// Cube is solved
    Solved,
}

struct AnalysisData {
    progress: CFOPProgress,
    state_start_time: u32,
    state_start_index: usize,
    state_recognition_time: Option<u32>,
    state_moves: Vec<Move>,
    total_moves: usize,
    cube: Cube3x3x3Faces,
    cross_color: Color,
    cross_face: Face,
    cross_analysis: Option<CrossAnalysis>,
    f2l_pairs: Vec<F2LPairAnalysis>,
    oll_analysis: Vec<OLLAnalysis>,
    pll_analysis: Vec<PLLAnalysis>,
    alignment: Option<FinalAlignmentAnalysis>,
    time: u32,
}

impl CFOPAnalysis {
    pub fn analyze(solve: &CubeWithSolution) -> Option<Self> {
        CFOPPartialAnalysis::analyze(solve).into()
    }
}

impl OLLAlgorithm {
    fn from_index(idx: usize) -> Self {
        match idx + 1 {
            21 => OLLAlgorithm::H,
            22 => OLLAlgorithm::Pi,
            23 => OLLAlgorithm::U,
            24 => OLLAlgorithm::T,
            25 => OLLAlgorithm::L,
            26 => OLLAlgorithm::Antisune,
            27 => OLLAlgorithm::Sune,
            idx => OLLAlgorithm::OLL(idx as u8),
        }
    }

    pub fn is_cross(&self) -> bool {
        match self {
            OLLAlgorithm::H
            | OLLAlgorithm::Pi
            | OLLAlgorithm::U
            | OLLAlgorithm::T
            | OLLAlgorithm::L
            | OLLAlgorithm::Antisune
            | OLLAlgorithm::Sune => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> u8 {
        match self {
            OLLAlgorithm::H => 21,
            OLLAlgorithm::Pi => 22,
            OLLAlgorithm::U => 23,
            OLLAlgorithm::T => 24,
            OLLAlgorithm::L => 25,
            OLLAlgorithm::Antisune => 26,
            OLLAlgorithm::Sune => 27,
            OLLAlgorithm::OLL(num) => *num,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OLLAlgorithm::H => "H".into(),
            OLLAlgorithm::Pi => "Pi".into(),
            OLLAlgorithm::U => "U".into(),
            OLLAlgorithm::T => "T".into(),
            OLLAlgorithm::L => "L".into(),
            OLLAlgorithm::Antisune => "Antisune".into(),
            OLLAlgorithm::Sune => "Sune".into(),
            OLLAlgorithm::OLL(num) => format!("#{}", num),
        }
    }

    fn bit_for_idx(cube: &Cube3x3x3Faces, idx: usize, color: Color, bit: usize) -> u32 {
        if cube.color_by_idx(idx) == color {
            1 << bit
        } else {
            0
        }
    }

    fn bitmask_from_cube(cube: &Cube3x3x3Faces, last_layer: Face) -> u32 {
        let color = last_layer.color();
        let edges = &CUBE3_LAST_LAYER_EDGE[last_layer as u8 as usize];
        let mut mask = Self::bit_for_idx(cube, edges[0].idx(0), color, 20);
        mask |= Self::bit_for_idx(cube, edges[0].idx(1), color, 19);
        mask |= Self::bit_for_idx(cube, edges[0].idx(2), color, 18);
        mask |= Self::bit_for_idx(cube, edges[3].idx(2), color, 17);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 0, 0), color, 16);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 0, 1), color, 15);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 0, 2), color, 14);
        mask |= Self::bit_for_idx(cube, edges[1].idx(0), color, 13);
        mask |= Self::bit_for_idx(cube, edges[3].idx(1), color, 12);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 1, 0), color, 11);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 1, 1), color, 10);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 1, 2), color, 9);
        mask |= Self::bit_for_idx(cube, edges[1].idx(1), color, 8);
        mask |= Self::bit_for_idx(cube, edges[3].idx(0), color, 7);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 2, 0), color, 6);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 2, 1), color, 5);
        mask |= Self::bit_for_idx(cube, Cube3x3x3Faces::idx(last_layer, 2, 2), color, 4);
        mask |= Self::bit_for_idx(cube, edges[1].idx(2), color, 3);
        mask |= Self::bit_for_idx(cube, edges[2].idx(2), color, 2);
        mask |= Self::bit_for_idx(cube, edges[2].idx(1), color, 1);
        mask |= Self::bit_for_idx(cube, edges[2].idx(0), color, 0);
        mask
    }

    fn move_bit(mask: u32, old: usize, new: usize) -> u32 {
        if mask & (1 << old) != 0 {
            1 << new
        } else {
            0
        }
    }

    fn rotate_bitmask(mask: u32) -> u32 {
        let mut result = Self::move_bit(mask, 13, 0);
        result |= Self::move_bit(mask, 8, 1);
        result |= Self::move_bit(mask, 3, 2);
        result |= Self::move_bit(mask, 18, 3);
        result |= Self::move_bit(mask, 14, 4);
        result |= Self::move_bit(mask, 9, 5);
        result |= Self::move_bit(mask, 4, 6);
        result |= Self::move_bit(mask, 0, 7);
        result |= Self::move_bit(mask, 19, 8);
        result |= Self::move_bit(mask, 15, 9);
        result |= Self::move_bit(mask, 10, 10);
        result |= Self::move_bit(mask, 5, 11);
        result |= Self::move_bit(mask, 1, 12);
        result |= Self::move_bit(mask, 20, 13);
        result |= Self::move_bit(mask, 16, 14);
        result |= Self::move_bit(mask, 11, 15);
        result |= Self::move_bit(mask, 6, 16);
        result |= Self::move_bit(mask, 2, 17);
        result |= Self::move_bit(mask, 17, 18);
        result |= Self::move_bit(mask, 12, 19);
        result |= Self::move_bit(mask, 7, 20);
        result
    }

    pub fn from_cube(cube: &Cube3x3x3Faces, last_layer: Face) -> Option<Self> {
        let mask1 = Self::bitmask_from_cube(cube, last_layer);
        let mask2 = Self::rotate_bitmask(mask1);
        let mask3 = Self::rotate_bitmask(mask2);
        let mask4 = Self::rotate_bitmask(mask3);
        for mask in &[mask1, mask2, mask3, mask4] {
            for (idx, case) in CUBE3_OLL_CASES.iter().enumerate() {
                if mask == case {
                    return Some(Self::from_index(idx));
                }
            }
        }
        None
    }
}

impl PLLAlgorithm {
    fn from_index(idx: usize) -> Self {
        match idx {
            0 => PLLAlgorithm::Aa,
            1 => PLLAlgorithm::Ab,
            2 => PLLAlgorithm::F,
            3 => PLLAlgorithm::Ga,
            4 => PLLAlgorithm::Gb,
            5 => PLLAlgorithm::Gc,
            6 => PLLAlgorithm::Gd,
            7 => PLLAlgorithm::Ja,
            8 => PLLAlgorithm::Jb,
            9 => PLLAlgorithm::Ra,
            10 => PLLAlgorithm::Rb,
            11 => PLLAlgorithm::T,
            12 => PLLAlgorithm::E,
            13 => PLLAlgorithm::Na,
            14 => PLLAlgorithm::Nb,
            15 => PLLAlgorithm::V,
            16 => PLLAlgorithm::Y,
            17 => PLLAlgorithm::H,
            18 => PLLAlgorithm::Ua,
            19 => PLLAlgorithm::Ub,
            20 => PLLAlgorithm::Z,
            _ => unreachable!(),
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            PLLAlgorithm::Aa => "Aa",
            PLLAlgorithm::Ab => "Ab",
            PLLAlgorithm::F => "F",
            PLLAlgorithm::Ga => "Ga",
            PLLAlgorithm::Gb => "Gb",
            PLLAlgorithm::Gc => "Gc",
            PLLAlgorithm::Gd => "Gd",
            PLLAlgorithm::Ja => "Ja",
            PLLAlgorithm::Jb => "Jb",
            PLLAlgorithm::Ra => "Ra",
            PLLAlgorithm::Rb => "Rb",
            PLLAlgorithm::T => "T",
            PLLAlgorithm::E => "E",
            PLLAlgorithm::Na => "Na",
            PLLAlgorithm::Nb => "Nb",
            PLLAlgorithm::V => "V",
            PLLAlgorithm::Y => "Y",
            PLLAlgorithm::H => "H",
            PLLAlgorithm::Ua => "Ua",
            PLLAlgorithm::Ub => "Ub",
            PLLAlgorithm::Z => "Z",
        }
    }

    pub fn from_cube(cube: &Cube3x3x3Faces, last_layer: Face) -> Option<Self> {
        // Iterate for each possible rotation (cases are stored as a single one of the
        // possible rotations).
        for rotation in 0..4 {
            let face = &CUBE3_LAST_LAYER_EDGE[last_layer as u8 as usize];

            // Set up color array where we will store the index where we see each color.
            // There will be exactly 3 occurences of each of the 4 side colors. The array
            // is sized for all 6 colors to simplify the logic. We will sort the array
            // later and the top and bottom color will be removed easily, since it is
            // always zero.
            let mut colors = [0u16; 6];
            let mut idx = 0u16;

            // Loop through each side and add color information
            for edge_idx in 0..4 {
                let edge = &face[(edge_idx + rotation) % 4];
                match edge {
                    FaceRowOrColumn::RowLeftToRight(face, row) => {
                        for col in 0..3 {
                            let color = &mut colors[cube.color(*face, *row, col) as u8 as usize];
                            *color <<= 4;
                            *color |= idx;
                            idx += 1;
                        }
                    }
                    FaceRowOrColumn::RowRightToLeft(face, row) => {
                        for col in 0..3 {
                            let color =
                                &mut colors[cube.color(*face, *row, 2 - col) as u8 as usize];
                            *color <<= 4;
                            *color |= idx;
                            idx += 1;
                        }
                    }
                    FaceRowOrColumn::ColumnTopDown(face, col) => {
                        for row in 0..3 {
                            let color = &mut colors[cube.color(*face, row, *col) as u8 as usize];
                            *color <<= 4;
                            *color |= idx;
                            idx += 1;
                        }
                    }
                    FaceRowOrColumn::ColumnBottomUp(face, col) => {
                        for row in 0..3 {
                            let color =
                                &mut colors[cube.color(*face, 2 - row, *col) as u8 as usize];
                            *color <<= 4;
                            *color |= idx;
                            idx += 1;
                        }
                    }
                }
            }

            // Sort colors and take last four. For PLL there are only 4 possible colors on
            // the edges so the first two entries will always be zero.
            colors.sort();
            let colors = &colors[2..];

            // Detect PLL case. This may not succeed if it is the wrong rotation. We try
            // all four rotations.
            for (idx, case) in CUBE3_PLL_CASES.iter().enumerate() {
                if colors == case {
                    return Some(Self::from_index(idx));
                }
            }
        }
        None
    }
}

impl AnalysisData {
    fn new(solve: &CubeWithSolution, cross_color: Color) -> Self {
        let mut result = Self {
            progress: CFOPProgress::Initial,
            state_start_time: 0,
            state_start_index: 0,
            state_recognition_time: None,
            state_moves: Vec::new(),
            total_moves: 0,
            cube: solve.initial_state.as_faces(),
            cross_color,
            cross_face: cross_color.face(),
            cross_analysis: None,
            f2l_pairs: Vec::new(),
            oll_analysis: Vec::new(),
            pll_analysis: Vec::new(),
            alignment: None,
            time: 0,
        };
        result.check_for_state_transitions();
        result
    }

    fn new_state(&mut self, state: CFOPProgress) {
        self.progress = state;
        self.state_start_time = self.time;
        self.state_start_index = self.total_moves;
        self.state_recognition_time = None;
        self.state_moves.clear();
    }

    fn cross_solved(&self) -> bool {
        let cross_edges = &CUBE3_EDGE_ADJACENCY[self.cross_face as u8 as usize];
        self.cube.color(self.cross_face, 0, 1) == self.cross_face.color()
            && self.cube.color(self.cross_face, 1, 0) == self.cross_face.color()
            && self.cube.color(self.cross_face, 1, 2) == self.cross_face.color()
            && self.cube.color(self.cross_face, 2, 1) == self.cross_face.color()
            && self.cube.color_by_idx(cross_edges[0])
                == Cube3x3x3Faces::face_for_idx(cross_edges[0]).color()
            && self.cube.color_by_idx(cross_edges[1])
                == Cube3x3x3Faces::face_for_idx(cross_edges[1]).color()
            && self.cube.color_by_idx(cross_edges[2])
                == Cube3x3x3Faces::face_for_idx(cross_edges[2]).color()
            && self.cube.color_by_idx(cross_edges[3])
                == Cube3x3x3Faces::face_for_idx(cross_edges[3]).color()
    }

    fn f2l_pair_count(&self) -> usize {
        let mut pair_count = 0;
        for pairs in &CUBE3_F2L_PAIRS[self.cross_face as u8 as usize] {
            let mut ok = true;
            for piece in pairs {
                if self.cube.color_by_idx(*piece) != Cube3x3x3Faces::face_for_idx(*piece).color() {
                    ok = false;
                }
            }
            if ok {
                pair_count += 1;
            }
        }
        pair_count
    }

    fn f2l_solved(&self) -> bool {
        self.cross_solved() && self.f2l_pair_count() == 4
    }

    fn last_layer_oriented(&self) -> bool {
        let face = self.cross_face.opposite();
        for row in 0..3 {
            for col in 0..3 {
                if self.cube.color(face, row, col) != face.color() {
                    return false;
                }
            }
        }
        true
    }

    fn last_layer_solved(&self) -> bool {
        if !self.last_layer_oriented() {
            return false;
        }
        let face = self.cross_face.opposite();
        for edge in &CUBE3_LAST_LAYER_EDGE[face as u8 as usize] {
            match edge {
                FaceRowOrColumn::RowLeftToRight(face, row)
                | FaceRowOrColumn::RowRightToLeft(face, row) => {
                    for col in 1..3 {
                        if self.cube.color(*face, *row, col) != self.cube.color(*face, *row, 0) {
                            return false;
                        }
                    }
                }
                FaceRowOrColumn::ColumnTopDown(face, col)
                | FaceRowOrColumn::ColumnBottomUp(face, col) => {
                    for row in 1..3 {
                        if self.cube.color(*face, row, *col) != self.cube.color(*face, 0, *col) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn check_for_single_state_transition(&mut self) {
        if self.total_moves > self.state_start_index && self.state_recognition_time.is_none() {
            self.state_recognition_time = Some(self.time - self.state_start_time);
        }
        match self.progress.clone() {
            CFOPProgress::Initial => {
                if self.cross_solved() {
                    self.cross_analysis = Some(CrossAnalysis {
                        color: self.cross_color,
                        time: self.time - self.state_start_time,
                        moves: self.state_moves.clone(),
                    });
                    self.new_state(CFOPProgress::F2LPair(0));
                }
            }
            CFOPProgress::F2LPair(count) => {
                let new_pair_count = self.f2l_pair_count();
                if self.cross_solved() && new_pair_count > count {
                    if self.state_moves.len() != 0 {
                        let recognition_time = self.state_recognition_time.unwrap_or(0);
                        self.f2l_pairs.push(F2LPairAnalysis {
                            recognition_time,
                            execution_time: self.time - self.state_start_time - recognition_time,
                            start_move_index: self.state_start_index,
                            moves: self.state_moves.clone(),
                        });
                    }
                    if new_pair_count == 4 {
                        if self.last_layer_solved() {
                            self.new_state(CFOPProgress::FinalAlignment);
                        } else if self.last_layer_oriented() {
                            self.new_state(CFOPProgress::PLL(
                                PLLAlgorithm::from_cube(&self.cube, self.cross_face.opposite())
                                    .unwrap(),
                            ));
                        } else {
                            self.new_state(CFOPProgress::OLL(
                                OLLAlgorithm::from_cube(&self.cube, self.cross_face.opposite())
                                    .unwrap(),
                            ));
                        }
                    } else {
                        self.new_state(CFOPProgress::F2LPair(new_pair_count));
                    }
                }
            }
            CFOPProgress::OLL(one_look_algorithm) => {
                if self.f2l_solved() {
                    if self.last_layer_oriented() {
                        // OLL complete, record OLL algorithm performance
                        if self.state_moves.len() != 0 {
                            // To arrive at OLL solved state, we must have performed the OLL
                            // algorithm found at the start of the state.
                            let recognition_time = self.state_recognition_time.unwrap_or(0);
                            self.oll_analysis.push(OLLAnalysis {
                                one_look_algorithm,
                                performed_algorithm: one_look_algorithm,
                                new_state: None,
                                recognition_time,
                                execution_time: self.time
                                    - self.state_start_time
                                    - recognition_time,
                                start_move_index: self.state_start_index,
                                moves: self.state_moves.clone(),
                            });
                        }

                        // Check PLL and transition state
                        if self.last_layer_solved() {
                            self.new_state(CFOPProgress::FinalAlignment);
                        } else {
                            self.new_state(CFOPProgress::PLL(
                                PLLAlgorithm::from_cube(&self.cube, self.cross_face.opposite())
                                    .unwrap(),
                            ));
                        }
                        return;
                    }

                    let new_one_look_algorithm =
                        OLLAlgorithm::from_cube(&self.cube, self.cross_face.opposite()).unwrap();
                    if new_one_look_algorithm != one_look_algorithm {
                        // We have arrived at a different OLL case. Record the state transition,
                        // the execution stats, and which algorithm was performed.
                        if self.state_moves.len() != 0 {
                            // Detect performed algorithm by applying the inverse of the moves
                            // performed to a solved cube and detect which OLL case it is. We
                            // know we were already in a state where the first two layers were
                            // solved, and we are now also in a state where the first two layers
                            // are solved, so this transformation is always going to yield an
                            // OLL case.
                            let mut xform_cube = Cube3x3x3Faces::new();
                            xform_cube.do_moves(&self.state_moves.inverse());
                            let performed_algorithm =
                                OLLAlgorithm::from_cube(&xform_cube, self.cross_face.opposite())
                                    .unwrap();

                            let recognition_time = self.state_recognition_time.unwrap_or(0);
                            self.oll_analysis.push(OLLAnalysis {
                                one_look_algorithm,
                                performed_algorithm,
                                new_state: Some(new_one_look_algorithm),
                                recognition_time,
                                execution_time: self.time
                                    - self.state_start_time
                                    - recognition_time,
                                start_move_index: self.state_start_index,
                                moves: self.state_moves.clone(),
                            });
                        }
                        self.new_state(CFOPProgress::OLL(new_one_look_algorithm));
                    }
                }
            }
            CFOPProgress::PLL(one_look_algorithm) => {
                if self.f2l_solved() && self.last_layer_oriented() {
                    if self.last_layer_solved() {
                        // PLL complete, record PLL algorithm performance
                        if self.state_moves.len() != 0 {
                            // To arrive at PLL solved state, we must have performed the PLL
                            // algorithm found at the start of the state.
                            let recognition_time = self.state_recognition_time.unwrap_or(0);
                            self.pll_analysis.push(PLLAnalysis {
                                one_look_algorithm,
                                performed_algorithm: one_look_algorithm,
                                new_state: None,
                                recognition_time,
                                execution_time: self.time
                                    - self.state_start_time
                                    - recognition_time,
                                start_move_index: self.state_start_index,
                                moves: self.state_moves.clone(),
                            });
                        }
                        self.new_state(CFOPProgress::FinalAlignment);
                        return;
                    }

                    let new_one_look_algorithm =
                        PLLAlgorithm::from_cube(&self.cube, self.cross_face.opposite()).unwrap();
                    if new_one_look_algorithm != one_look_algorithm {
                        // We have arrived at a different PLL case. Record the state transition,
                        // the execution stats, and which algorithm was performed.
                        if self.state_moves.len() != 0 {
                            // Detect performed algorithm by applying the inverse of the moves
                            // performed to a solved cube and detect which PLL case it is. We
                            // know we were already in a state where OLL was solved, and we
                            // are now also in a state where OLL is solved, so this
                            // transformation is always going to yield a PLL case.
                            let mut xform_cube = Cube3x3x3Faces::new();
                            xform_cube.do_moves(&self.state_moves.inverse());
                            let performed_algorithm =
                                PLLAlgorithm::from_cube(&xform_cube, self.cross_face.opposite())
                                    .unwrap();

                            let recognition_time = self.state_recognition_time.unwrap_or(0);
                            self.pll_analysis.push(PLLAnalysis {
                                one_look_algorithm,
                                performed_algorithm,
                                new_state: Some(new_one_look_algorithm),
                                recognition_time,
                                execution_time: self.time
                                    - self.state_start_time
                                    - recognition_time,
                                start_move_index: self.state_start_index,
                                moves: self.state_moves.clone(),
                            });
                        }
                        self.new_state(CFOPProgress::PLL(new_one_look_algorithm));
                    }
                }
            }
            CFOPProgress::FinalAlignment => {
                if self.cube.is_solved() {
                    self.alignment = Some(FinalAlignmentAnalysis {
                        time: self.time - self.state_start_time,
                        start_move_index: self.state_start_index,
                        moves: self.state_moves.clone(),
                    });
                }
            }
            _ => (),
        }
    }

    fn check_for_state_transitions(&mut self) -> bool {
        // Perform state transitions until no change
        let mut changed = false;
        loop {
            let before = self.progress.clone();
            self.check_for_single_state_transition();
            if self.progress == before {
                return changed;
            }
            changed = true;
        }
    }

    fn do_move(&mut self, timed_move: &TimedMove) {
        self.cube.do_move(timed_move.move_());
        self.time = timed_move.time();
        self.total_moves += 1;
        self.state_moves.push(timed_move.move_());
        self.check_for_state_transitions();
    }
}

impl CFOPPartialAnalysis {
    pub fn analyze(solve: &CubeWithSolution) -> Self {
        let cases = [
            Self::analyze_for_cross_color(solve, Color::White),
            Self::analyze_for_cross_color(solve, Color::Green),
            Self::analyze_for_cross_color(solve, Color::Red),
            Self::analyze_for_cross_color(solve, Color::Blue),
            Self::analyze_for_cross_color(solve, Color::Orange),
            Self::analyze_for_cross_color(solve, Color::Yellow),
        ];
        let mut best: Option<Self> = None;
        for case in cases {
            if let Some(prev_best) = &best {
                if case.transition_count() > prev_best.transition_count()
                    || (case.transition_count() == prev_best.transition_count()
                        && case.sum_of_transition_times() < prev_best.sum_of_transition_times())
                {
                    best = Some(case);
                }
            } else {
                best = Some(case);
            }
        }
        best.unwrap()
    }

    fn transition_count(&self) -> usize {
        let mut count = 0;
        if self.cross.is_some() {
            count += 1;
        }
        count += self.oll.len();
        count += self.pll.len();
        if self.alignment.is_some() {
            count += 1;
        }
        count
    }

    fn sum_of_transition_times(&self) -> u32 {
        let mut sum = 0;
        let mut time = 0;
        if let Some(cross) = &self.cross {
            time += cross.time;
            sum += time;
        }
        for oll in &self.oll {
            time += oll.recognition_time + oll.execution_time;
            sum += time;
        }
        for pll in &self.pll {
            time += pll.recognition_time + pll.execution_time;
            sum += time;
        }
        if let Some(align) = &self.alignment {
            time += align.time;
            sum += time;
        }
        sum
    }

    fn analyze_for_cross_color(solve: &CubeWithSolution, cross_color: Color) -> Self {
        let mut data = AnalysisData::new(solve, cross_color);
        for mv in &solve.solution {
            data.do_move(mv);
        }

        Self {
            progress: data.progress,
            cross: data.cross_analysis,
            f2l_pairs: data.f2l_pairs,
            oll: data.oll_analysis,
            pll: data.pll_analysis,
            alignment: data.alignment,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress == CFOPProgress::Solved
    }
}

impl From<CFOPPartialAnalysis> for Option<CFOPAnalysis> {
    fn from(analysis: CFOPPartialAnalysis) -> Option<CFOPAnalysis> {
        if let Some(cross) = analysis.cross {
            if let Some(alignment) = analysis.alignment {
                return Some(CFOPAnalysis {
                    cross,
                    f2l_pairs: analysis.f2l_pairs,
                    oll: analysis.oll,
                    pll: analysis.pll,
                    alignment,
                });
            }
        }
        None
    }
}

impl std::fmt::Display for CFOPPartialAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(cross) = &self.cross {
            write!(
                f,
                "{} cross: {} moves in {}ms\n",
                cross.color.to_str(),
                cross.moves.len(),
                cross.time
            )?;
        }
        for pair in &self.f2l_pairs {
            write!(
                f,
                "F2L pair: Recognition {}ms, {} moves in {}ms\n",
                pair.recognition_time,
                pair.moves.len(),
                pair.execution_time
            )?;
        }
        for oll in &self.oll {
            if oll.new_state.is_some() {
                write!(
                    f,
                    "Intermediate OLL ({}): Recognition {}ms, {} moves in {}ms, one-look was {}\n",
                    oll.performed_algorithm.to_string(),
                    oll.recognition_time,
                    oll.moves.len(),
                    oll.execution_time,
                    oll.one_look_algorithm.to_string(),
                )?;
            } else {
                write!(
                    f,
                    "OLL ({}): Recognition {}ms, {} moves in {}ms\n",
                    oll.performed_algorithm.to_string(),
                    oll.recognition_time,
                    oll.moves.len(),
                    oll.execution_time
                )?;
            }
        }
        for pll in &self.pll {
            if pll.new_state.is_some() {
                write!(
                    f,
                    "Intermediate PLL ({}): Recognition {}ms, {} moves in {}ms, one-look was {}\n",
                    pll.performed_algorithm.to_str(),
                    pll.recognition_time,
                    pll.moves.len(),
                    pll.execution_time,
                    pll.one_look_algorithm.to_str(),
                )?;
            } else {
                write!(
                    f,
                    "PLL ({}): Recognition {}ms, {} moves in {}ms\n",
                    pll.performed_algorithm.to_str(),
                    pll.recognition_time,
                    pll.moves.len(),
                    pll.execution_time
                )?;
            }
        }
        if let Some(alignment) = &self.alignment {
            if alignment.moves.len() != 0 {
                write!(
                    f,
                    "Alignment: {} moves in {}ms\n",
                    alignment.moves.len(),
                    alignment.time
                )?;
            }
        }
        Ok(())
    }
}
