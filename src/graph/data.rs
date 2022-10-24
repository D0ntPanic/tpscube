use crate::graph::plot::{Plot, SinglePlot, YAxis};
use crate::theme::Theme;
use tpscube_core::{
    Analysis, Cube, Cube3x3x3, CubeWithSolution, History, InitialCubeState, ListAverage, Penalty,
    Solve, SolveType,
};

pub struct GraphData {
    statistic: Statistic,
    phase: Phase,
    average_size: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Statistic {
    TotalTime,
    RecognitionTime,
    ExecutionTime,
    MoveCount,
    TurnsPerSecond,
    ExecutionTurnsPerSecond,
    SuccessRate,
    RecognitionAccuracy,
    ExecutionAccuracy,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    EntireSolve,
    CFOP(CFOPPhase),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CFOPPhase {
    Cross,
    F2L,
    OLL,
    PLL,
}

impl Statistic {
    fn y_axis(&self) -> YAxis {
        match self {
            Statistic::TotalTime | Statistic::RecognitionTime | Statistic::ExecutionTime => {
                YAxis::Time
            }
            Statistic::MoveCount => YAxis::MoveCount,
            Statistic::TurnsPerSecond | Statistic::ExecutionTurnsPerSecond => YAxis::TurnsPerSecond,
            Statistic::SuccessRate
            | Statistic::RecognitionAccuracy
            | Statistic::ExecutionAccuracy => YAxis::SuccessRate,
        }
    }
}

impl GraphData {
    pub fn new() -> Self {
        Self {
            statistic: Statistic::TotalTime,
            phase: Phase::EntireSolve,
            average_size: 5,
        }
    }

    pub fn statistic(mut self, statistic: Statistic) -> Self {
        self.statistic = statistic;
        self
    }

    pub fn phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    pub fn average_size(mut self, size: usize) -> Self {
        self.average_size = size;
        self
    }

    fn analyze(solve: &Solve) -> Option<Analysis> {
        if let Some(solution) = &solve.moves {
            let mut initial_state = Cube3x3x3::new();
            initial_state.do_moves(&solve.scramble);
            Some(Analysis::analyze(&CubeWithSolution {
                initial_state,
                solution: solution.clone(),
            }))
        } else {
            None
        }
    }

    fn data_point(solve: &Solve, statistic: Statistic, phase: Phase) -> Option<u32> {
        match statistic {
            Statistic::TurnsPerSecond => {
                // Calculate TPS generically by fetching time and move stats
                let time = Self::data_point(solve, Statistic::TotalTime, phase);
                let moves = Self::data_point(solve, Statistic::MoveCount, phase);
                if let Some(time) = time {
                    if let Some(moves) = moves {
                        if moves > 0 && time > 0 {
                            Some((moves * 1000) / time)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Statistic::ExecutionTurnsPerSecond => {
                // Calculate TPS generically by fetching time and move stats
                let time = Self::data_point(solve, Statistic::ExecutionTime, phase);
                let moves = Self::data_point(solve, Statistic::MoveCount, phase);
                if let Some(time) = time {
                    if let Some(moves) = moves {
                        if moves > 0 && time > 0 {
                            Some((moves * 1000) / time)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Statistic::SuccessRate => match solve.penalty {
                Penalty::DNF | Penalty::RecognitionDNF | Penalty::ExecutionDNF => Some(0),
                _ => Some(1),
            },
            Statistic::RecognitionAccuracy => match solve.penalty {
                Penalty::DNF => None,
                Penalty::RecognitionDNF => Some(0),
                _ => Some(1),
            },
            Statistic::ExecutionAccuracy => match solve.penalty {
                Penalty::DNF => None,
                Penalty::ExecutionDNF => Some(0),
                Penalty::RecognitionDNF => None,
                _ => Some(1),
            },
            _ => match phase {
                Phase::EntireSolve => match statistic {
                    Statistic::TotalTime => solve.final_time(),
                    Statistic::RecognitionTime => {
                        if let Some(analysis) = Self::analyze(solve) {
                            if let Analysis::CFOP(cfop) = analysis {
                                // Total up recognition times from all phases
                                Some(
                                    cfop.f2l_pairs
                                        .iter()
                                        .fold(0, |sum, pair| sum + pair.recognition_time)
                                        + cfop
                                            .oll
                                            .iter()
                                            .fold(0, |sum, alg| sum + alg.recognition_time)
                                        + cfop
                                            .pll
                                            .iter()
                                            .fold(0, |sum, alg| sum + alg.recognition_time),
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Statistic::ExecutionTime => {
                        if let Some(analysis) = Self::analyze(solve) {
                            if let Analysis::CFOP(cfop) = analysis {
                                // Total up execution times from all phases
                                Some(
                                    cfop.cross.time
                                        + cfop
                                            .f2l_pairs
                                            .iter()
                                            .fold(0, |sum, pair| sum + pair.execution_time)
                                        + cfop
                                            .oll
                                            .iter()
                                            .fold(0, |sum, alg| sum + alg.execution_time)
                                        + cfop
                                            .pll
                                            .iter()
                                            .fold(0, |sum, alg| sum + alg.execution_time)
                                        + cfop.alignment.time,
                                )
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Statistic::MoveCount => {
                        if let Some(solution) = &solve.moves {
                            Some(solution.len() as u32 * 1000)
                        } else {
                            None
                        }
                    }
                    Statistic::TurnsPerSecond
                    | Statistic::ExecutionTurnsPerSecond
                    | Statistic::SuccessRate
                    | Statistic::RecognitionAccuracy
                    | Statistic::ExecutionAccuracy => {
                        unreachable!()
                    }
                },
                Phase::CFOP(phase) => {
                    if let Some(analysis) = Self::analyze(solve) {
                        if let Analysis::CFOP(cfop) = analysis {
                            match phase {
                                CFOPPhase::Cross => match statistic {
                                    Statistic::TotalTime => Some(cfop.cross.time),
                                    Statistic::RecognitionTime => Some(0),
                                    Statistic::ExecutionTime => Some(cfop.cross.time),
                                    Statistic::MoveCount => {
                                        Some(cfop.cross.moves.len() as u32 * 1000)
                                    }
                                    Statistic::TurnsPerSecond
                                    | Statistic::ExecutionTurnsPerSecond
                                    | Statistic::SuccessRate
                                    | Statistic::RecognitionAccuracy
                                    | Statistic::ExecutionAccuracy => {
                                        unreachable!()
                                    }
                                },
                                CFOPPhase::F2L => Some(cfop.f2l_pairs.iter().fold(
                                    0,
                                    |sum, pair| match statistic {
                                        Statistic::TotalTime => {
                                            sum + pair.recognition_time + pair.execution_time
                                        }
                                        Statistic::RecognitionTime => sum + pair.recognition_time,
                                        Statistic::ExecutionTime => sum + pair.execution_time,
                                        Statistic::MoveCount => {
                                            sum + pair.moves.len() as u32 * 1000
                                        }
                                        Statistic::TurnsPerSecond
                                        | Statistic::ExecutionTurnsPerSecond
                                        | Statistic::SuccessRate
                                        | Statistic::RecognitionAccuracy
                                        | Statistic::ExecutionAccuracy => unreachable!(),
                                    },
                                )),
                                CFOPPhase::OLL => {
                                    if cfop.oll.len() == 0 {
                                        // Don't include OLL skips in OLL timing
                                        None
                                    } else {
                                        Some(cfop.oll.iter().fold(0, |sum, alg| match statistic {
                                            Statistic::TotalTime => {
                                                sum + alg.recognition_time + alg.execution_time
                                            }
                                            Statistic::RecognitionTime => {
                                                sum + alg.recognition_time
                                            }
                                            Statistic::ExecutionTime => sum + alg.execution_time,
                                            Statistic::MoveCount => {
                                                sum + alg.moves.len() as u32 * 1000
                                            }
                                            Statistic::TurnsPerSecond
                                            | Statistic::ExecutionTurnsPerSecond
                                            | Statistic::SuccessRate
                                            | Statistic::RecognitionAccuracy
                                            | Statistic::ExecutionAccuracy => unreachable!(),
                                        }))
                                    }
                                }
                                CFOPPhase::PLL => {
                                    if cfop.pll.len() == 0 {
                                        // Don't include PLL skips in PLL timing
                                        None
                                    } else {
                                        Some(match statistic {
                                            Statistic::TotalTime => {
                                                cfop.pll.iter().fold(0, |sum, alg| {
                                                    sum + alg.recognition_time + alg.execution_time
                                                }) + cfop.alignment.time
                                            }
                                            Statistic::RecognitionTime => cfop
                                                .pll
                                                .iter()
                                                .fold(0, |sum, alg| sum + alg.recognition_time),
                                            Statistic::ExecutionTime => {
                                                cfop.pll
                                                    .iter()
                                                    .fold(0, |sum, alg| sum + alg.execution_time)
                                                    + cfop.alignment.time
                                            }
                                            Statistic::MoveCount => {
                                                cfop.pll.iter().fold(0, |sum, alg| {
                                                    sum + alg.moves.len() as u32 * 1000
                                                }) + cfop.alignment.moves.len() as u32 * 1000
                                            }
                                            Statistic::TurnsPerSecond
                                            | Statistic::ExecutionTurnsPerSecond
                                            | Statistic::SuccessRate
                                            | Statistic::RecognitionAccuracy
                                            | Statistic::ExecutionAccuracy => unreachable!(),
                                        })
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            },
        }
    }

    pub fn build(self, history: &History, solve_type: SolveType) -> Plot {
        let title = format!(
            "{} for {}",
            match self.statistic {
                Statistic::TotalTime => "Time",
                Statistic::RecognitionTime => "Recognition Time",
                Statistic::ExecutionTime => "Execution Time",
                Statistic::MoveCount => "Move Count",
                Statistic::TurnsPerSecond => "Turns per Second",
                Statistic::ExecutionTurnsPerSecond => "Execution TPS",
                Statistic::SuccessRate => "Success Rate",
                Statistic::RecognitionAccuracy => "Recognition Accuracy",
                Statistic::ExecutionAccuracy => "Execution Accuracy",
            },
            match self.phase {
                Phase::EntireSolve => "Entire Solve",
                Phase::CFOP(CFOPPhase::Cross) => "Cross Phase",
                Phase::CFOP(CFOPPhase::F2L) => "F2L Phase",
                Phase::CFOP(CFOPPhase::OLL) => "OLL Phase",
                Phase::CFOP(CFOPPhase::PLL) => "PLL Phase",
            }
        );

        let mut plot = SinglePlot::new(
            title,
            self.statistic.y_axis(),
            match self.phase {
                Phase::CFOP(CFOPPhase::Cross) => Theme::Red.into(),
                Phase::CFOP(CFOPPhase::F2L) => Theme::Blue.into(),
                Phase::CFOP(CFOPPhase::OLL) => Theme::Yellow.into(),
                Phase::CFOP(CFOPPhase::PLL) => Theme::Green.into(),
                _ => Theme::Blue.into(),
            },
        );

        let mut window = Vec::new();
        for solve in history.iter() {
            if solve.solve_type != solve_type {
                // Only include solves with the current solve type
                continue;
            }

            let data_point = match Self::data_point(solve, self.statistic, self.phase) {
                Some(value) => Some(value),
                None => continue,
            };

            window.push(data_point);
            if window.len() > self.average_size {
                window.remove(0);
            }
            if window.len() >= self.average_size {
                if matches!(
                    self.statistic,
                    Statistic::SuccessRate
                        | Statistic::RecognitionAccuracy
                        | Statistic::ExecutionAccuracy
                ) {
                    plot.push(
                        solve.created,
                        window.iter().fold(0, |sum, pt| sum + pt.unwrap_or(0)) as f32 * 100.0
                            / window.len() as f32,
                    );
                } else if let Some(average) = window.as_slice().average() {
                    plot.push(solve.created, average as f32 / 1000.0);
                }
            }
        }

        plot.into()
    }
}
