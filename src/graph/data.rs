use crate::graph::plot::{Plot, SinglePlot, YAxis};
use crate::theme::Theme;
use tpscube_core::{Analysis, Cube, Cube3x3x3, CubeWithSolution, History, ListAverage};

pub struct GraphData {
    statistic: Statistic,
    phase: Phase,
    average_size: usize,
}

#[derive(Clone, Copy)]
pub enum Statistic {
    Time,
    MoveCount,
}

#[derive(Clone, Copy)]
pub enum Phase {
    EntireSolve,
    CFOP(CFOPPhase),
}

#[derive(Clone, Copy)]
pub enum CFOPPhase {
    Cross,
    F2L,
    OLL,
    PLL,
}

impl Statistic {
    fn y_axis(&self) -> YAxis {
        match self {
            Statistic::Time => YAxis::Time,
            Statistic::MoveCount => YAxis::MoveCount,
        }
    }
}

impl GraphData {
    pub fn new() -> Self {
        Self {
            statistic: Statistic::Time,
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

    pub fn build(self, history: &History) -> Plot {
        let title = format!(
            "{} for {}",
            match self.statistic {
                Statistic::Time => "Time",
                Statistic::MoveCount => "Move Count",
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
            let data_point = match self.phase {
                Phase::EntireSolve => match self.statistic {
                    Statistic::Time => solve.final_time(),
                    Statistic::MoveCount => {
                        if let Some(solution) = &solve.moves {
                            Some(solution.len() as u32 * 1000)
                        } else {
                            continue;
                        }
                    }
                },
                Phase::CFOP(phase) => {
                    if let Some(solution) = &solve.moves {
                        let mut initial_state = Cube3x3x3::new();
                        initial_state.do_moves(&solve.scramble);
                        let analysis = Analysis::analyze(&CubeWithSolution {
                            initial_state,
                            solution: solution.clone(),
                        });
                        if let Analysis::CFOP(cfop) = analysis {
                            match phase {
                                CFOPPhase::Cross => match self.statistic {
                                    Statistic::Time => Some(cfop.cross.time),
                                    Statistic::MoveCount => {
                                        Some(cfop.cross.moves.len() as u32 * 1000)
                                    }
                                },
                                CFOPPhase::F2L => {
                                    Some(cfop.f2l_pairs.iter().fold(0, |sum, pair| {
                                        match self.statistic {
                                            Statistic::Time => {
                                                sum + pair.recognition_time + pair.execution_time
                                            }
                                            Statistic::MoveCount => {
                                                sum + pair.moves.len() as u32 * 1000
                                            }
                                        }
                                    }))
                                }
                                CFOPPhase::OLL => {
                                    if cfop.oll.len() == 0 {
                                        // Don't include OLL skips in OLL timing
                                        continue;
                                    }
                                    Some(cfop.oll.iter().fold(0, |sum, alg| match self.statistic {
                                        Statistic::Time => {
                                            sum + alg.recognition_time + alg.execution_time
                                        }
                                        Statistic::MoveCount => sum + alg.moves.len() as u32 * 1000,
                                    }))
                                }
                                CFOPPhase::PLL => {
                                    if cfop.pll.len() == 0 {
                                        // Don't include PLL skips in PLL timing
                                        continue;
                                    }
                                    Some(match self.statistic {
                                        Statistic::Time => {
                                            cfop.pll.iter().fold(0, |sum, alg| {
                                                sum + alg.recognition_time + alg.execution_time
                                            }) + cfop.alignment.time
                                        }
                                        Statistic::MoveCount => {
                                            cfop.pll.iter().fold(0, |sum, alg| {
                                                sum + alg.moves.len() as u32 * 1000
                                            }) + cfop.alignment.moves.len() as u32 * 1000
                                        }
                                    })
                                }
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
            };

            window.push(data_point);
            if window.len() > self.average_size {
                window.remove(0);
            }
            if window.len() >= self.average_size {
                if let Some(average) = window.as_slice().average() {
                    plot.push(solve.created, average as f32 / 1000.0);
                }
            }
        }

        plot.finalize();
        plot.into()
    }
}
