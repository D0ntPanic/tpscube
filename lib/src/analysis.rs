mod cfop;

use crate::{Cube, Cube3x3x3, Solve, TimedMove};

pub use cfop::{
    CFOPAnalysis, CFOPPartialAnalysis, CFOPProgress, CrossAnalysis, F2LPairAnalysis,
    FinalAlignmentAnalysis, OLLAlgorithm, OLLAnalysis, PLLAlgorithm, PLLAnalysis,
};

#[derive(Clone)]
pub enum Analysis {
    Unsuccessful,
    CFOP(CFOPAnalysis),
}

#[derive(Clone)]
pub enum PartialAnalysis {
    Unsuccessful,
    CFOP(CFOPPartialAnalysis),
}

#[derive(Clone)]
pub struct AnalysisStepSummary {
    pub name: String,
    pub short_name: String,
    pub major_step_index: usize,
    pub algorithm: Option<String>,
    pub recognition_time: u32,
    pub execution_time: u32,
    pub substeps: Vec<AnalysisSubstepTime>,
    pub move_count: usize,
}

#[derive(Clone, Copy)]
pub enum AnalysisSubstepTime {
    Recognition(u32),
    Execution(u32),
}

pub trait AnalysisSummary {
    fn step_summary(&self) -> Vec<AnalysisStepSummary>;
    fn detailed_step_summary(&self) -> Vec<AnalysisStepSummary>;
}

pub trait PartialAnalysisMethod: AnalysisSummary {
    fn transition_count(&self) -> usize;
    fn sum_of_transition_times(&self) -> u32;
    fn is_complete(&self) -> bool;
    fn to_partial_analysis(&self) -> PartialAnalysis;
}

#[derive(Clone)]
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

impl PartialAnalysis {
    pub fn analyze(solve: &CubeWithSolution) -> Self {
        if solve.solution.len() == 0 {
            // No moves, nothing can be analyzed
            return PartialAnalysis::Unsuccessful;
        }

        // Analyze with all available solving methods
        let methods = &[&CFOPPartialAnalysis::analyze(solve)];

        // Find the most likely solving method based on transition counts and timing
        let mut best: Option<&dyn PartialAnalysisMethod> = None;
        for method in methods {
            if let Some(prev_best) = &best {
                if method.transition_count() > prev_best.transition_count()
                    || (method.transition_count() == prev_best.transition_count()
                        && method.sum_of_transition_times() < prev_best.sum_of_transition_times())
                {
                    best = Some(*method);
                }
            } else {
                best = Some(*method);
            }
        }
        best.unwrap().to_partial_analysis()
    }
}

impl AnalysisSummary for Analysis {
    fn step_summary(&self) -> Vec<AnalysisStepSummary> {
        match self {
            Analysis::Unsuccessful => Vec::new(),
            Analysis::CFOP(analysis) => analysis.step_summary(),
        }
    }

    fn detailed_step_summary(&self) -> Vec<AnalysisStepSummary> {
        match self {
            Analysis::Unsuccessful => Vec::new(),
            Analysis::CFOP(analysis) => analysis.detailed_step_summary(),
        }
    }
}

impl AnalysisSummary for PartialAnalysis {
    fn step_summary(&self) -> Vec<AnalysisStepSummary> {
        match self {
            PartialAnalysis::Unsuccessful => Vec::new(),
            PartialAnalysis::CFOP(analysis) => analysis.step_summary(),
        }
    }

    fn detailed_step_summary(&self) -> Vec<AnalysisStepSummary> {
        match self {
            PartialAnalysis::Unsuccessful => Vec::new(),
            PartialAnalysis::CFOP(analysis) => analysis.detailed_step_summary(),
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
