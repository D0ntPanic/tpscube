mod list;
mod report;

use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::CustomWidgets;
use egui::{
    Align, CentralPanel, CtxRef, Direction, Layout, SidePanel, Stroke, TopBottomPanel, Ui, Vec2,
};
use report::TPSReport;
use std::collections::HashMap;
use tpscube_core::{
    Analysis, Cube, Cube3x3x3, CubeWithSolution, History, OLLAlgorithm, PLLAlgorithm,
};

pub struct AlgorithmsWidget {
    cached_update_id: Option<u64>,
    algorithm_stats: AlgorithmStats,
    mode: AlgorithmMode,
    sort: Sort,
}

struct AlgorithmStats {
    oll: HashMap<OLLAlgorithm, AlgorithmCounts>,
    pll: HashMap<PLLAlgorithm, AlgorithmCounts>,
}

#[derive(Default)]
struct AlgorithmCounts {
    perform_count: usize,
    total_moves: usize,
    total_recognition_time: u64,
    total_execution_time: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AlgorithmMode {
    Algorithms(AlgorithmType),
    TPSReport(AlgorithmType),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AlgorithmType {
    OLL,
    PLL,
}

struct Sort {
    column: SortColumn,
    order: SortOrder,
}

enum SortColumn {
    Count,
    RecognitionTime,
    ExecutionTime,
    TotalTime,
    MoveCount,
    TPS,
    ExecutionTPS,
}

enum SortOrder {
    Ascending,
    Descending,
}

impl AlgorithmsWidget {
    pub fn new() -> Self {
        Self {
            cached_update_id: None,
            algorithm_stats: AlgorithmStats {
                oll: HashMap::new(),
                pll: HashMap::new(),
            },
            mode: AlgorithmMode::TPSReport(AlgorithmType::PLL),
            sort: Sort {
                column: SortColumn::TPS,
                order: SortOrder::Descending,
            },
        }
    }

    fn analyze(&mut self, history: &History) {
        self.algorithm_stats.oll.clear();
        self.algorithm_stats.pll.clear();

        for solve in history.iter() {
            if let Some(moves) = &solve.moves {
                let mut unsolved_state = Cube3x3x3::new();
                unsolved_state.do_moves(&solve.scramble);
                let analysis = Analysis::analyze(&CubeWithSolution {
                    initial_state: unsolved_state.clone(),
                    solution: moves.clone(),
                });

                if let Analysis::CFOP(cfop) = analysis {
                    for oll in cfop.oll {
                        let oll_entry = self
                            .algorithm_stats
                            .oll
                            .entry(oll.performed_algorithm)
                            .or_insert(AlgorithmCounts::default());
                        oll_entry.perform_count += 1;
                        oll_entry.total_moves += oll.moves.len();
                        oll_entry.total_recognition_time += oll.recognition_time as u64;
                        oll_entry.total_execution_time += oll.execution_time as u64;
                    }

                    for pll in cfop.pll {
                        let pll_entry = self
                            .algorithm_stats
                            .pll
                            .entry(pll.performed_algorithm)
                            .or_insert(AlgorithmCounts::default());
                        pll_entry.perform_count += 1;
                        pll_entry.total_moves += pll.moves.len();
                        pll_entry.total_recognition_time += pll.recognition_time as u64;
                        pll_entry.total_execution_time += pll.execution_time as u64;
                    }
                }
            }
        }
    }

    fn algorithm_options(&mut self, ui: &mut Ui) {
        if ui
            .mode_label(
                "OLL",
                self.mode == AlgorithmMode::Algorithms(AlgorithmType::OLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::Algorithms(AlgorithmType::OLL);
        }

        if ui
            .mode_label(
                "PLL",
                self.mode == AlgorithmMode::Algorithms(AlgorithmType::PLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::Algorithms(AlgorithmType::PLL);
        }
    }

    fn report_options(&mut self, ui: &mut Ui) {
        if ui
            .mode_label(
                "OLL",
                self.mode == AlgorithmMode::TPSReport(AlgorithmType::OLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::TPSReport(AlgorithmType::OLL);
        }

        if ui
            .mode_label(
                "PLL",
                self.mode == AlgorithmMode::TPSReport(AlgorithmType::PLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::TPSReport(AlgorithmType::PLL);
        }
    }

    fn landscape_sidebar(&mut self, ctxt: &CtxRef) {
        SidePanel::left("left_algorithm_options")
            .default_width(160.0)
            .resizable(false)
            .show(ctxt, |ui| {
                ui.vertical(|ui| {
                    ui.section("Algorithms");
                    self.algorithm_options(ui);

                    ui.add_space(8.0);
                    ui.section("TPS Reports");
                    self.report_options(ui);
                });
            });
    }

    fn portrait_top_bar(&mut self, ctxt: &CtxRef) {
        TopBottomPanel::top("top_algorithm_options").show(ctxt, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::TOP),
                    |ui| {
                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("Algorithms");
                                    self.algorithm_options(ui);
                                    ui.add_space(4.0);
                                });
                            },
                        );

                        // Show separator between sections
                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Disabled.into(),
                            };
                            ui.separator();
                        });

                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("TPS Reports");
                                    self.report_options(ui);
                                    ui.add_space(4.0);
                                });
                            },
                        );
                    },
                );
            });
        });
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        if self.cached_update_id != Some(history.update_id()) {
            self.cached_update_id = Some(history.update_id());
            self.analyze(history);
        }

        ctxt.set_visuals(side_visuals());
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        if aspect >= 1.0 {
            // Landscape mode. Report options to the left.
            self.landscape_sidebar(ctxt);
        } else {
            // Portrait mode. Report options at the top.
            self.portrait_top_bar(ctxt);
        }

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| match self.mode {
            AlgorithmMode::TPSReport(alg_type) => {
                let report = TPSReport::new(&self.algorithm_stats, alg_type, &mut self.sort);
                report.update(ui);
            }
            _ => (),
        });
    }
}
