mod data;
mod plot;

use crate::font::FontSize;
use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::CustomWidgets;
use anyhow::Result;
use data::{CFOPPhase, GraphData, Phase, Statistic};
use egui::{
    Align, CentralPanel, CtxRef, Direction, Label, Layout, Pos2, Rect, ScrollArea, Sense,
    SidePanel, Stroke, TopBottomPanel, Ui, Vec2,
};
use plot::Plot;
use tpscube_core::{History, SolveType};

const GRAPH_PADDING: f32 = 16.0;

pub struct GraphWidget {
    statistic: Statistic,
    phase: Phase,
    average_size: usize,
    plot: Option<Plot>,
    update_id: Option<u64>,
    solve_type: SolveType,
    settings_restored: bool,
}

impl GraphWidget {
    pub fn new() -> Self {
        Self {
            statistic: Statistic::TotalTime,
            phase: Phase::EntireSolve,
            average_size: 5,
            plot: None,
            update_id: None,
            solve_type: SolveType::Standard3x3x3,
            settings_restored: false,
        }
    }

    fn statistic_options(&mut self, ui: &mut Ui, history: &mut History, is_3x3x3: bool) {
        if ui
            .mode_label("Total time", self.statistic == Statistic::TotalTime)
            .clicked()
        {
            self.statistic = Statistic::TotalTime;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if is_3x3x3 {
            if ui
                .mode_label(
                    "Recognition time",
                    self.statistic == Statistic::RecognitionTime,
                )
                .clicked()
            {
                self.statistic = Statistic::RecognitionTime;
                self.plot = None;
                let _ = self.save_settings(history);
            }

            if ui
                .mode_label("Execution time", self.statistic == Statistic::ExecutionTime)
                .clicked()
            {
                self.statistic = Statistic::ExecutionTime;
                self.plot = None;
                let _ = self.save_settings(history);
            }

            if ui
                .mode_label("Move count", self.statistic == Statistic::MoveCount)
                .clicked()
            {
                self.statistic = Statistic::MoveCount;
                self.plot = None;
                let _ = self.save_settings(history);
            }

            if ui
                .mode_label(
                    "Turns per second",
                    self.statistic == Statistic::TurnsPerSecond,
                )
                .clicked()
            {
                self.statistic = Statistic::TurnsPerSecond;
                self.plot = None;
                let _ = self.save_settings(history);
            }

            if ui
                .mode_label(
                    "Execution TPS",
                    self.statistic == Statistic::ExecutionTurnsPerSecond,
                )
                .clicked()
            {
                self.statistic = Statistic::ExecutionTurnsPerSecond;
                self.plot = None;
                let _ = self.save_settings(history);
            }
        }

        if ui
            .mode_label("Success Rate", self.statistic == Statistic::SuccessRate)
            .clicked()
        {
            self.statistic = Statistic::SuccessRate;
            self.plot = None;
            let _ = self.save_settings(history);
        }
    }

    fn phase_options(&mut self, ui: &mut Ui, history: &mut History) {
        if ui
            .mode_label("Entire solve", self.phase == Phase::EntireSolve)
            .clicked()
        {
            self.phase = Phase::EntireSolve;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label("Cross", self.phase == Phase::CFOP(CFOPPhase::Cross))
            .clicked()
        {
            self.phase = Phase::CFOP(CFOPPhase::Cross);
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label("F2L", self.phase == Phase::CFOP(CFOPPhase::F2L))
            .clicked()
        {
            self.phase = Phase::CFOP(CFOPPhase::F2L);
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label("OLL", self.phase == Phase::CFOP(CFOPPhase::OLL))
            .clicked()
        {
            self.phase = Phase::CFOP(CFOPPhase::OLL);
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label("PLL", self.phase == Phase::CFOP(CFOPPhase::PLL))
            .clicked()
        {
            self.phase = Phase::CFOP(CFOPPhase::PLL);
            self.plot = None;
            let _ = self.save_settings(history);
        }
    }

    fn average_options(&mut self, ui: &mut Ui, history: &mut History, compact: bool) {
        if ui
            .mode_label(
                if compact { "mo3" } else { "Mean of 3" },
                self.average_size == 3,
            )
            .clicked()
        {
            self.average_size = 3;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label(
                if compact { "ao5" } else { "Average of 5" },
                self.average_size == 5,
            )
            .clicked()
        {
            self.average_size = 5;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label(
                if compact { "ao12" } else { "Average of 12" },
                self.average_size == 12,
            )
            .clicked()
        {
            self.average_size = 12;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label(
                if compact { "ao50" } else { "Average of 50" },
                self.average_size == 50,
            )
            .clicked()
        {
            self.average_size = 50;
            self.plot = None;
            let _ = self.save_settings(history);
        }

        if ui
            .mode_label(
                if compact { "ao100" } else { "Average of 100" },
                self.average_size == 100,
            )
            .clicked()
        {
            self.average_size = 100;
            self.plot = None;
            let _ = self.save_settings(history);
        }
    }

    fn landscape_sidebar(&mut self, ctxt: &CtxRef, history: &mut History, solve_type: SolveType) {
        SidePanel::left("left_graph_options")
            .default_width(160.0)
            .resizable(false)
            .show(ctxt, |ui| {
                ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
                ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
                ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
                ScrollArea::auto_sized()
                    .always_show_scroll(false)
                    .id_source("left_graph_options_scroll")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.section("Statistic");
                            self.statistic_options(ui, history, solve_type.is_3x3x3());

                            if solve_type.is_3x3x3() {
                                ui.add_space(8.0);
                                ui.section("Phase");
                                self.phase_options(ui, history);
                            } else {
                                if !matches!(
                                    self.statistic,
                                    Statistic::TotalTime | Statistic::SuccessRate
                                ) {
                                    self.statistic = Statistic::TotalTime;
                                }
                                self.phase = Phase::EntireSolve;
                            }

                            ui.add_space(8.0);
                            ui.section("Average");
                            self.average_options(ui, history, false);
                        });
                    });
            });
    }

    fn portrait_top_bar(&mut self, ctxt: &CtxRef, history: &mut History, solve_type: SolveType) {
        TopBottomPanel::top("top_graph_options").show(ctxt, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::TOP),
                    |ui| {
                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("Statistic");
                                    self.statistic_options(ui, history, solve_type.is_3x3x3());
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

                        if solve_type.is_3x3x3() {
                            ui.allocate_ui(
                                Vec2::new(
                                    (ui.max_rect().width() - 48.0) / 3.0,
                                    ui.max_rect().height(),
                                ),
                                |ui| {
                                    ui.vertical(|ui| {
                                        ui.section("Phase");
                                        self.phase_options(ui, history);
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
                        } else {
                            if !matches!(
                                self.statistic,
                                Statistic::TotalTime | Statistic::SuccessRate
                            ) {
                                self.statistic = Statistic::TotalTime;
                            }
                            self.phase = Phase::EntireSolve;
                        }

                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 6.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("Avg");
                                    self.average_options(ui, history, true);
                                    ui.add_space(4.0);
                                });
                            },
                        );
                    },
                );
            });
        });
    }

    fn restore_settings(&mut self, history: &History) {
        self.average_size = history.setting_as_i64("graph_average").unwrap_or(5) as usize;
        self.statistic = match history
            .setting_as_string("graph_stat")
            .as_ref()
            .map(|s| s.as_str())
        {
            Some("total_time") => Statistic::TotalTime,
            Some("recognition_time") => Statistic::RecognitionTime,
            Some("execution_time") => Statistic::ExecutionTime,
            Some("move_count") => Statistic::MoveCount,
            Some("tps") => Statistic::TurnsPerSecond,
            Some("etps") => Statistic::ExecutionTurnsPerSecond,
            Some("success") => Statistic::SuccessRate,
            Some(_) | None => Statistic::TotalTime,
        };
        self.phase = match history
            .setting_as_string("graph_phase")
            .as_ref()
            .map(|s| s.as_str())
        {
            Some("all") => Phase::EntireSolve,
            Some("cfop/cross") => Phase::CFOP(CFOPPhase::Cross),
            Some("cfop/f2l") => Phase::CFOP(CFOPPhase::F2L),
            Some("cfop/oll") => Phase::CFOP(CFOPPhase::OLL),
            Some("cfop/pll") => Phase::CFOP(CFOPPhase::PLL),
            Some(_) | None => Phase::EntireSolve,
        };
        self.settings_restored = true;
    }

    fn save_settings(&self, history: &mut History) -> Result<()> {
        history.set_i64_setting("graph_average", self.average_size as i64)?;
        history.set_string_setting(
            "graph_stat",
            match self.statistic {
                Statistic::TotalTime => "total_time",
                Statistic::RecognitionTime => "recognition_time",
                Statistic::ExecutionTime => "execution_time",
                Statistic::MoveCount => "move_count",
                Statistic::TurnsPerSecond => "tps",
                Statistic::ExecutionTurnsPerSecond => "etps",
                Statistic::SuccessRate => "success",
            },
        )?;
        history.set_string_setting(
            "graph_phase",
            match self.phase {
                Phase::EntireSolve => "all",
                Phase::CFOP(CFOPPhase::Cross) => "cfop/cross",
                Phase::CFOP(CFOPPhase::F2L) => "cfop/f2l",
                Phase::CFOP(CFOPPhase::OLL) => "cfop/oll",
                Phase::CFOP(CFOPPhase::PLL) => "cfop/pll",
            },
        )?;
        Ok(())
    }

    fn requires_analysis(&self) -> bool {
        match self.phase {
            Phase::CFOP(_) => true,
            _ => match self.statistic {
                Statistic::TotalTime | Statistic::SuccessRate => false,
                _ => true,
            },
        }
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        solve_type: SolveType,
    ) {
        if !self.settings_restored {
            self.restore_settings(history);
        }

        ctxt.set_visuals(side_visuals());
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        if aspect >= 1.0 {
            // Landscape mode. Graph options to the left.
            self.landscape_sidebar(ctxt, history, solve_type);
        } else {
            // Portrait mode. Graph options at the top.
            self.portrait_top_bar(ctxt, history, solve_type);
        }

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            if self.update_id != Some(history.update_id()) || self.solve_type != solve_type {
                // If history has been updated, regenerate plot
                self.plot = None;
                self.solve_type = solve_type;
            }

            // Get plot data
            let plot = if let Some(plot) = &mut self.plot {
                plot
            } else {
                // No plot data cached, regenerate now
                self.plot = Some(
                    GraphData::new()
                        .statistic(self.statistic)
                        .phase(self.phase)
                        .average_size(self.average_size)
                        .build(history, solve_type),
                );
                self.update_id = Some(history.update_id());
                self.plot.as_mut().unwrap()
            };

            if plot.valid() {
                let painter = ui.painter();
                let rect = ui.max_rect();

                // Draw graph title
                let title_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Section.into(), plot.title().to_string());
                let title_width = title_galley.size.x;
                let title_height = title_galley.size.y;
                painter.galley(
                    Pos2::new(rect.center().x - title_width / 2.0, rect.top()),
                    title_galley,
                    Theme::Blue.into(),
                );

                // Draw plot
                let rect = Rect::from_min_size(
                    Pos2::new(
                        rect.left() + GRAPH_PADDING,
                        rect.top() + title_height + GRAPH_PADDING,
                    ),
                    Vec2::new(
                        rect.width() - GRAPH_PADDING * 2.0,
                        rect.height() - title_height - GRAPH_PADDING * 2.0,
                    ),
                );

                let interact = ui.allocate_rect(rect, Sense::click_and_drag());
                plot.update(ctxt, ui, rect, interact);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.add(
                        Label::new(if self.requires_analysis() {
                            "Bluetooth solves are required for this graph"
                        } else {
                            "Not enough data for this graph"
                        })
                        .text_color(Theme::Disabled),
                    );
                });
            }
        });
    }
}
