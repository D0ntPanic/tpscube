mod cfop;

use crate::{Cube, Cube3x3x3, Solve, TimedMove};

pub use cfop::{
    CFOPAnalysis, CFOPPartialAnalysis, CFOPProgress, CrossAnalysis, F2LPairAnalysis,
    FinalAlignmentAnalysis, OLLAlgorithm, OLLAnalysis, PLLAlgorithm, PLLAnalysis,
};

pub enum Analysis {
    Unsuccessful,
    CFOP(CFOPAnalysis),
}

pub struct CubeWithSolution {
    pub initial_state: Cube3x3x3,
    pub solution: Vec<TimedMove>,
}

pub trait SolveAnalysis {
    fn analyze(&self) -> Analysis;
}

impl Analysis {
    pub fn analyze(solve: &CubeWithSolution) -> Self {
        if let Some(cfop) = CFOPAnalysis::analyze(solve) {
            Analysis::CFOP(cfop)
        } else {
            Analysis::Unsuccessful
        }
    }
}

impl SolveAnalysis for CubeWithSolution {
    fn analyze(&self) -> Analysis {
        Analysis::analyze(self)
    }
}

impl SolveAnalysis for Solve {
    fn analyze(&self) -> Analysis {
        let solve: Option<CubeWithSolution> = self.into();
        if let Some(solve) = solve {
            Analysis::analyze(&solve)
        } else {
            Analysis::Unsuccessful
        }
    }
}

impl From<&Solve> for Option<CubeWithSolution> {
    fn from(solve: &Solve) -> Option<CubeWithSolution> {
        if let Some(moves) = &solve.moves {
            let mut cube = Cube3x3x3::new();
            cube.do_moves(&solve.scramble);
            Some(CubeWithSolution {
                initial_state: cube,
                solution: moves.clone(),
            })
        } else {
            None
        }
    }
}
