use crate::algorithms::AlgorithmRender;
use crate::app::SolveDetails;
use crate::font::{FontSize, LabelFontSize};
use crate::theme::Theme;
use crate::timer::scramble::TimerCube;
use crate::timer::state::TimerState;
use crate::widgets::{solve_time_string, CustomWidgets};
use chrono::{DateTime, Local};
use egui::{
    popup_below_widget, Align, Align2, CtxRef, CursorIcon, Label, Layout, ScrollArea,
    SelectableLabel, Sense, SidePanel, Stroke, TopBottomPanel, Ui, Vec2,
};
use tpscube_core::{
    Algorithm, Average, BestSolve, Cube, Cube3x3x3, CubeFace, History, InitialCubeState,
    KnownAlgorithms, ListAverage, OLLAlgorithm, PLLAlgorithm, Penalty, Solve, SolveList, SolveType,
};

use super::scramble::LastLayerAlgorithmSelection;

pub struct TimerSession {
    update_id: Option<u64>,
    solves: Vec<Solve>,
    last_ao5: Option<Average>,
    last_ao12: Option<Average>,
    session_avg: Option<u32>,
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
}

enum SessionTime {
    BestSolve(BestSolve),
    AverageOfN(Average),
    SessionAverage(u32),
}

impl TimerSession {
    pub fn new() -> Self {
        Self {
            update_id: None,
            solves: Vec::new(),
            last_ao5: None,
            last_ao12: None,
            session_avg: None,
            best_solve: None,
            best_ao5: None,
            best_ao12: None,
        }
    }

    pub fn check_solve_type(&mut self, history: &mut History, solve_type: SolveType) {
        // If solve type no longer matches session's, create a new session to hold the
        // new type of solves
        if let Some(session) = history.sessions().get(history.current_session()) {
            if session.solve_type() != solve_type {
                self.new_session(history);
            }
        }
    }

    fn from_solves(update_id: Option<u64>, solves: Vec<Solve>) -> Self {
        let last_ao5 = solves.as_slice().last_average(5);
        let last_ao12 = solves.as_slice().last_average(12);
        let session_avg = solves.as_slice().average();
        let best_solve = solves.as_slice().best();
        let best_ao5 = solves.as_slice().best_average(5);
        let best_ao12 = solves.as_slice().best_average(12);

        Self {
            update_id,
            solves,
            last_ao5,
            last_ao12,
            session_avg,
            best_solve,
            best_ao5,
            best_ao12,
        }
    }

    fn update(&mut self, history: &History) {
        if let Some(session) = history.sessions().get(history.current_session()) {
            // Check for updates
            if let Some(update_id) = self.update_id {
                if update_id == session.update_id() {
                    // Already cached and up to date
                    return;
                }
            }

            // Cache solve information
            *self = Self::from_solves(Some(session.update_id()), session.to_vec(history));
        } else {
            // New session, invalidate cache
            *self = Self::from_solves(None, Vec::new());
        }
    }

    fn session_time(
        ui: &mut Ui,
        name: &str,
        small: bool,
        time: Option<SessionTime>,
        details: &mut Option<SolveDetails>,
    ) {
        ui.horizontal(|ui| {
            if small {
                ui.add(Label::new(format!("{}:", name)).small());
            } else {
                ui.label(format!("{}:", name));
            }
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.visuals_mut().widgets.noninteractive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                };
                ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                };
                ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Blue.into(),
                };
                ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Blue.into(),
                };

                if let Some(time) = time {
                    match time {
                        SessionTime::AverageOfN(average) => {
                            if ui
                                .add(
                                    Label::new(solve_time_string(average.time))
                                        .sense(Sense::click()),
                                )
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                *details = Some(SolveDetails::AverageOfSolves(average.solves));
                            }
                        }
                        SessionTime::SessionAverage(time) => {
                            ui.label(solve_time_string(time));
                        }
                        SessionTime::BestSolve(solve) => {
                            if ui
                                .add(
                                    Label::new(solve_time_string(solve.time)).sense(Sense::click()),
                                )
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                *details = Some(SolveDetails::IndividualSolve(solve.solve));
                            }
                        }
                    }
                } else {
                    ui.label("-");
                }
            })
        });
    }

    fn add_standard_solve(
        ui: &mut Ui,
        idx: usize,
        solve: &Solve,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        ui.horizontal(|ui| {
            ui.add(Label::new(format!("{}.", idx + 1)).text_color(Theme::Disabled));
            ui.with_layout(Layout::right_to_left(), |ui| {
                let popup_id = ui.make_persistent_id(format!("timer-{}", solve.id));
                let response = ui.add(Label::new("  ☰").small().sense(Sense::click()));
                if response.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                popup_below_widget(ui, popup_id, &response, |ui| {
                    ui.set_min_width(180.0);
                    if ui
                        .add(
                            SelectableLabel::new(
                                match solve.penalty {
                                    Penalty::None => true,
                                    _ => false,
                                },
                                "No penalty",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::None);
                        let _ = history.local_commit();
                    }

                    if ui
                        .add(
                            SelectableLabel::new(
                                match solve.penalty {
                                    Penalty::Time(2000) => true,
                                    _ => false,
                                },
                                "2 second penalty",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::Time(2000));
                        let _ = history.local_commit();
                    }

                    if ui
                        .add(
                            SelectableLabel::new(
                                match solve.penalty {
                                    Penalty::DNF => true,
                                    _ => false,
                                },
                                "DNF",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::DNF);
                        let _ = history.local_commit();
                    }

                    ui.separator();

                    if ui
                        .add(
                            SelectableLabel::new(false, "Delete solve")
                                .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.delete_solve(solve.id.clone());
                        let _ = history.local_commit();
                    }
                });

                // Draw penalty if there is one, but always reserve space for it
                let penalty_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Small.into(), " (+2) ".into());
                let (response, painter) = ui.allocate_painter(penalty_galley.size, Sense::hover());
                if let Penalty::Time(penalty) = solve.penalty {
                    painter.text(
                        response.rect.left_bottom(),
                        Align2::LEFT_BOTTOM,
                        format!(" (+{})", penalty / 1000),
                        FontSize::Small.into(),
                        Theme::Red.into(),
                    );
                }

                ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                };
                ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Blue.into(),
                };
                ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Blue.into(),
                };
                let response = if let Some(time) = solve.final_time() {
                    ui.add(Label::new(solve_time_string(time)).sense(Sense::click()))
                } else {
                    ui.add(
                        Label::new("DNF")
                            .text_color(Theme::Red)
                            .sense(Sense::click()),
                    )
                };

                // Check for click on solve time
                if response.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::IndividualSolve(solve.clone()));
                }
            });
        });
    }

    fn training_penalty_menu(ui: &mut Ui, solve: &Solve, history: &mut History) {
        let popup_id = ui.make_persistent_id(format!("timer-{}", solve.id));
        let response = ui.add(Label::new("  ☰").small().sense(Sense::click()));
        if response.clicked() {
            ui.memory().toggle_popup(popup_id);
        }
        popup_below_widget(ui, popup_id, &response, |ui| {
            ui.set_min_width(180.0);
            if ui
                .add(
                    SelectableLabel::new(
                        match solve.penalty {
                            Penalty::None => true,
                            _ => false,
                        },
                        "No penalty",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.penalty(solve.id.clone(), Penalty::None);
                let _ = history.local_commit();
            }

            if ui
                .add(
                    SelectableLabel::new(
                        match solve.penalty {
                            Penalty::Time(2000) => true,
                            _ => false,
                        },
                        "2 second penalty",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.penalty(solve.id.clone(), Penalty::Time(2000));
                let _ = history.local_commit();
            }

            if ui
                .add(
                    SelectableLabel::new(
                        match solve.penalty {
                            Penalty::RecognitionDNF => true,
                            _ => false,
                        },
                        "Misrecognized",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.penalty(solve.id.clone(), Penalty::RecognitionDNF);
                let _ = history.local_commit();
            }

            if ui
                .add(
                    SelectableLabel::new(
                        match solve.penalty {
                            Penalty::ExecutionDNF => true,
                            _ => false,
                        },
                        "Misexecuted",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.penalty(solve.id.clone(), Penalty::ExecutionDNF);
                let _ = history.local_commit();
            }

            ui.separator();

            if ui
                .add(
                    SelectableLabel::new(false, "Delete solve").text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.delete_solve(solve.id.clone());
                let _ = history.local_commit();
            }
        });
    }

    fn add_training_solve(
        ui: &mut Ui,
        idx: usize,
        solve: &Solve,
        history: &mut History,
        details: &mut Option<SolveDetails>,
        algorithm: Algorithm,
    ) {
        let moves = match algorithm {
            Algorithm::OLL(oll) => KnownAlgorithms::oll(oll)[0].clone(),
            Algorithm::PLL(pll) => KnownAlgorithms::pll(pll)[0].clone(),
        };

        ui.horizontal(|ui| {
            algorithm.draw(
                ui,
                &moves,
                ui.fonts().row_height(FontSize::Normal.into()) * 2.0,
                false,
                None,
            );

            ui.add_space(8.0);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(format!("{}.", idx + 1)).text_color(Theme::Disabled));
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let _ = ui.allocate_space(
                            ui.fonts()
                                .layout_single_line(FontSize::Small.into(), "  ☰".into())
                                .size,
                        );
                        ui.add(
                            Label::new(match algorithm {
                                Algorithm::OLL(oll) => format!("#{}", oll.as_number()),
                                _ => algorithm.to_string(),
                            })
                            .text_color(Theme::Disabled),
                        );
                    });
                });

                ui.with_layout(Layout::right_to_left(), |ui| {
                    Self::training_penalty_menu(ui, solve, history);

                    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Content.into(),
                    };
                    ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Blue.into(),
                    };
                    ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Blue.into(),
                    };
                    let response = if let Some(time) = solve.final_time() {
                        ui.add(Label::new(solve_time_string(time)).sense(Sense::click()))
                    } else {
                        match solve.penalty {
                            Penalty::RecognitionDNF => ui.add(
                                Label::new("Misrecognize")
                                    .text_color(Theme::Red)
                                    .small()
                                    .sense(Sense::click()),
                            ),
                            Penalty::ExecutionDNF => ui.add(
                                Label::new("Misexecute")
                                    .text_color(Theme::Red)
                                    .small()
                                    .sense(Sense::click()),
                            ),
                            _ => ui.add(
                                Label::new("DNF")
                                    .text_color(Theme::Red)
                                    .sense(Sense::click()),
                            ),
                        }
                    };

                    // Check for click on solve time
                    if response.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                        *details = Some(SolveDetails::IndividualSolve(solve.clone()));
                    }
                });
            });
        });
    }

    fn add_oll_training_solve(
        ui: &mut Ui,
        idx: usize,
        solve: &Solve,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        let mut cube = Cube3x3x3::new();
        cube.do_moves(&solve.scramble);
        let algorithm = OLLAlgorithm::from_cube(&cube.as_faces(), CubeFace::Top);
        if let Some(algorithm) = algorithm {
            Self::add_training_solve(ui, idx, solve, history, details, Algorithm::OLL(algorithm))
        } else {
            Self::add_standard_solve(ui, idx, solve, history, details)
        }
    }

    fn add_pll_training_solve(
        ui: &mut Ui,
        idx: usize,
        solve: &Solve,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        let mut cube = Cube3x3x3::new();
        cube.do_moves(&solve.scramble);
        let algorithm = PLLAlgorithm::from_cube(&cube.as_faces(), CubeFace::Top);
        if let Some(algorithm) = algorithm {
            Self::add_training_solve(ui, idx, solve, history, details, Algorithm::PLL(algorithm))
        } else {
            Self::add_standard_solve(ui, idx, solve, history, details)
        }
    }

    fn add_solve(
        ui: &mut Ui,
        idx: usize,
        solve: &Solve,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        // Change window theme so that popup menu stands out
        let old_visuals = ui.ctx().style().visuals.clone();
        ui.ctx().set_visuals(crate::style::popup_visuals());

        ui.style_mut().spacing.item_spacing.x = 0.0;

        match solve.solve_type {
            SolveType::OLLTraining => {
                Self::add_oll_training_solve(ui, idx, solve, history, details)
            }
            SolveType::PLLTraining => {
                Self::add_pll_training_solve(ui, idx, solve, history, details)
            }
            _ => Self::add_standard_solve(ui, idx, solve, history, details),
        };

        // Restore old theme
        ui.ctx().set_visuals(old_visuals);
    }

    fn last_layer_training_algorithm_menu(
        ui: &mut Ui,
        history: &mut History,
        cube: &mut TimerCube,
    ) {
        let popup_id = ui.make_persistent_id("last-layer-training-algorithms");
        let response = ui.add(
            Label::new(format!(
                "{} ⏷",
                cube.last_layer_training_algorithms().to_string()
            ))
            .sense(Sense::click()),
        );
        if response.clicked() {
            ui.memory().toggle_popup(popup_id);
        }
        popup_below_widget(ui, popup_id, &response, |ui| {
            ui.set_min_width(180.0);
            if ui
                .add(
                    SelectableLabel::new(
                        matches!(
                            cube.last_layer_training_algorithms(),
                            LastLayerAlgorithmSelection::Known
                        ),
                        "Known",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_algorithms(
                    LastLayerAlgorithmSelection::Known,
                    history,
                );
            }

            if ui
                .add(
                    SelectableLabel::new(
                        matches!(
                            cube.last_layer_training_algorithms(),
                            LastLayerAlgorithmSelection::Learning
                        ),
                        "Learning",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_algorithms(
                    LastLayerAlgorithmSelection::Learning,
                    history,
                );
            }

            if ui
                .add(
                    SelectableLabel::new(
                        matches!(
                            cube.last_layer_training_algorithms(),
                            LastLayerAlgorithmSelection::KnownAndLearning
                        ),
                        "Known + Learning",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_algorithms(
                    LastLayerAlgorithmSelection::KnownAndLearning,
                    history,
                );
            }

            if ui
                .add(
                    SelectableLabel::new(
                        matches!(
                            cube.last_layer_training_algorithms(),
                            LastLayerAlgorithmSelection::All
                        ),
                        "All",
                    )
                    .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_algorithms(LastLayerAlgorithmSelection::All, history);
            }
        });
    }

    fn last_layer_training_weight_menu(ui: &mut Ui, history: &mut History, cube: &mut TimerCube) {
        let popup_id = ui.make_persistent_id("last-layer-training-weight");
        let response = ui.add(
            Label::new(match cube.last_layer_training_realistic_weights() {
                false => "Equal ⏷",
                true => "Realistic ⏷",
            })
            .sense(Sense::click()),
        );
        if response.clicked() {
            ui.memory().toggle_popup(popup_id);
        }
        popup_below_widget(ui, popup_id, &response, |ui| {
            ui.set_min_width(100.0);
            if ui
                .add(
                    SelectableLabel::new(!cube.last_layer_training_realistic_weights(), "Equal")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_realistic_weights(false, history);
            }

            if ui
                .add(
                    SelectableLabel::new(cube.last_layer_training_realistic_weights(), "Realistic")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_realistic_weights(true, history);
            }
        });
    }

    fn last_layer_training_learning_multiplier_menu(
        ui: &mut Ui,
        history: &mut History,
        cube: &mut TimerCube,
    ) {
        let popup_id = ui.make_persistent_id("last-layer-training-learning-mutliplier");
        let response = ui.add(
            Label::new(format!(
                "{}x ⏷",
                cube.last_layer_training_learning_multiplier()
            ))
            .sense(Sense::click()),
        );
        if response.clicked() {
            ui.memory().toggle_popup(popup_id);
        }
        popup_below_widget(ui, popup_id, &response, |ui| {
            ui.set_min_width(50.0);
            if ui
                .add(
                    SelectableLabel::new(cube.last_layer_training_learning_multiplier() == 1, "1x")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_learning_multiplier(1, history);
            }

            if ui
                .add(
                    SelectableLabel::new(cube.last_layer_training_learning_multiplier() == 2, "2x")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_learning_multiplier(2, history);
            }

            if ui
                .add(
                    SelectableLabel::new(cube.last_layer_training_learning_multiplier() == 4, "4x")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_learning_multiplier(4, history);
            }

            if ui
                .add(
                    SelectableLabel::new(cube.last_layer_training_learning_multiplier() == 8, "8x")
                        .text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                cube.set_last_layer_training_learning_multiplier(8, history);
            }
        });
    }

    fn new_session_button(ui: &mut Ui, history: &mut History) {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Red.into(),
            };
            ui.style_mut().visuals.widgets.active.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Red.into(),
            };
            ui.with_layout(Layout::right_to_left(), |ui| {
                if ui
                    .add(Label::new("↺  New session").sense(Sense::click()))
                    .clicked()
                {
                    let _ = history.new_session();
                }
            })
        });
    }

    pub fn landscape_sidebar(
        &mut self,
        ctxt: &CtxRef,
        history: &mut History,
        details: &mut Option<SolveDetails>,
        cube: &mut TimerCube,
    ) {
        SidePanel::left("left_timer")
            .default_width(175.0)
            .resizable(false)
            .show(ctxt, |ui| {
                self.update(history);

                if cube.solve_type().is_last_layer_training() {
                    ui.section("Settings");

                    ui.vertical(|ui| {
                        ui.label("Algorithms: ");
                        Self::last_layer_training_algorithm_menu(ui, history, cube);

                        ui.horizontal(|ui| {
                            ui.label("Weighting:");
                            Self::last_layer_training_weight_menu(ui, history, cube);
                        });

                        if matches!(
                            cube.last_layer_training_algorithms(),
                            LastLayerAlgorithmSelection::KnownAndLearning
                                | LastLayerAlgorithmSelection::All
                        ) {
                            ui.horizontal(|ui| {
                                ui.label("Learning Weight:");
                                Self::last_layer_training_learning_multiplier_menu(
                                    ui, history, cube,
                                );
                            });
                        }

                        ui.add_space(8.0);
                        Self::new_session_button(ui, history);
                    });
                } else {
                    ui.section("Session");

                    ui.vertical(|ui| {
                        Self::session_time(
                            ui,
                            "Last ao5",
                            false,
                            self.last_ao5
                                .clone()
                                .map(|avg| SessionTime::AverageOfN(avg)),
                            details,
                        );
                        Self::session_time(
                            ui,
                            "Last ao12",
                            false,
                            self.last_ao12
                                .clone()
                                .map(|avg| SessionTime::AverageOfN(avg)),
                            details,
                        );
                        Self::session_time(
                            ui,
                            "Session avg",
                            false,
                            self.session_avg.map(|avg| SessionTime::SessionAverage(avg)),
                            details,
                        );
                        Self::session_time(
                            ui,
                            "Best solve",
                            false,
                            self.best_solve
                                .clone()
                                .map(|best| SessionTime::BestSolve(best)),
                            details,
                        );
                        Self::session_time(
                            ui,
                            "Best ao5",
                            false,
                            self.best_ao5
                                .clone()
                                .map(|avg| SessionTime::AverageOfN(avg)),
                            details,
                        );
                        Self::session_time(
                            ui,
                            "Best ao12",
                            false,
                            self.best_ao12
                                .clone()
                                .map(|avg| SessionTime::AverageOfN(avg)),
                            details,
                        );

                        ui.add_space(8.0);
                        Self::new_session_button(ui, history);
                    });
                }

                ui.add_space(8.0);
                ui.section("Solves");

                ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
                ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
                ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
                ScrollArea::auto_sized()
                    .id_source("timer_solve_list")
                    .show(ui, |ui| {
                        let mut has_solves = false;
                        for (idx, solve) in self.solves.iter().enumerate().rev() {
                            Self::add_solve(ui, idx, solve, history, details);
                            has_solves = true;
                        }
                        if !has_solves {
                            ui.add(
                                Label::new("No solves in this session").text_color(Theme::Disabled),
                            );
                        }
                    });
            });
    }

    pub fn portrait_top_bar(
        &mut self,
        ctxt: &CtxRef,
        history: &mut History,
        details: &mut Option<SolveDetails>,
        cube: &mut TimerCube,
        state: &TimerState,
    ) {
        TopBottomPanel::top("top_timer").show(ctxt, |ui| {
            self.update(history);

            if cube.solve_type().is_last_layer_training() {
                // Settings header with embedded new session button.
                ui.horizontal(|ui| {
                    ui.add(
                        Label::new("Settings")
                            .font_size(FontSize::Section)
                            .text_color(Theme::Blue),
                    );
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        if ui
                            .add(Label::new("↺  New session").sense(Sense::click()))
                            .clicked()
                        {
                            let _ = history.new_session();
                        }
                    });
                });
                ui.section_separator();

                let width = ui.max_rect().width();

                ui.horizontal(|ui| {
                    ui.allocate_ui(
                        Vec2::new((width - 24.0) * 2.0 / 3.0, ui.max_rect().height()),
                        |ui| {
                            ui.vertical(|ui| {
                                ui.allocate_at_least(
                                    Vec2::new((width - 24.0) * 2.0 / 3.0, 0.0),
                                    Sense::hover(),
                                );

                                ui.horizontal(|ui| {
                                    ui.label("Algorithms: ");
                                    Self::last_layer_training_algorithm_menu(ui, history, cube);
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Weighting:");
                                    Self::last_layer_training_weight_menu(ui, history, cube);
                                });

                                if matches!(
                                    cube.last_layer_training_algorithms(),
                                    LastLayerAlgorithmSelection::KnownAndLearning
                                        | LastLayerAlgorithmSelection::All
                                ) {
                                    ui.horizontal(|ui| {
                                        ui.label("Learning Weight:");
                                        Self::last_layer_training_learning_multiplier_menu(
                                            ui, history, cube,
                                        );
                                    });
                                }
                            });
                        },
                    );

                    // Show separator between settings and last case
                    ui.scope(|ui| {
                        ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                            width: 1.0,
                            color: Theme::Disabled.into(),
                        };
                        ui.separator();
                    });

                    ui.allocate_ui(
                        Vec2::new((width - 24.0) / 3.0, ui.max_rect().height()),
                        |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                if let TimerState::Inactive(_, Some(last_solve)) = state {
                                    if last_solve.solve_type == cube.solve_type() {
                                        let algorithm = match last_solve.solve_type {
                                            SolveType::OLLTraining => {
                                                let mut cube = Cube3x3x3::new();
                                                cube.do_moves(&last_solve.scramble);
                                                let algorithm = OLLAlgorithm::from_cube(
                                                    &cube.as_faces(),
                                                    CubeFace::Top,
                                                );
                                                algorithm.map(|alg| Algorithm::OLL(alg))
                                            }
                                            SolveType::PLLTraining => {
                                                let mut cube = Cube3x3x3::new();
                                                cube.do_moves(&last_solve.scramble);
                                                let algorithm = PLLAlgorithm::from_cube(
                                                    &cube.as_faces(),
                                                    CubeFace::Top,
                                                );
                                                algorithm.map(|alg| Algorithm::PLL(alg))
                                            }
                                            _ => None,
                                        };

                                        if let Some(algorithm) = algorithm {
                                            ui.label(format!(
                                                "Last: {}",
                                                match algorithm {
                                                    Algorithm::OLL(oll) => match oll {
                                                        OLLAlgorithm::OLL(_) => oll.to_string(),
                                                        _ => format!(
                                                            "{} (#{})",
                                                            oll.to_string(),
                                                            oll.as_number()
                                                        ),
                                                    },
                                                    _ => algorithm.to_string(),
                                                },
                                            ));
                                            ui.add_space(4.0);

                                            let moves = match algorithm {
                                                Algorithm::OLL(oll) => {
                                                    KnownAlgorithms::oll(oll)[0].clone()
                                                }
                                                Algorithm::PLL(pll) => {
                                                    KnownAlgorithms::pll(pll)[0].clone()
                                                }
                                            };

                                            algorithm.draw(
                                                ui,
                                                &moves,
                                                ui.fonts().row_height(FontSize::Normal.into())
                                                    * 2.0,
                                                false,
                                                None,
                                            );
                                        }
                                    }
                                }
                            });
                        },
                    );
                });
            } else {
                // Session header with embedded new session button.
                ui.horizontal(|ui| {
                    ui.add(
                        Label::new("Session")
                            .font_size(FontSize::Section)
                            .text_color(Theme::Blue),
                    );
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        if ui
                            .add(Label::new("↺  New session").sense(Sense::click()))
                            .clicked()
                        {
                            let _ = history.new_session();
                        }
                    })
                });
                ui.section_separator();

                // If the screen is too small, can only show last averages
                let best_cutoff = if crate::is_mobile() == Some(true) {
                    320.0
                } else {
                    290.0
                };
                let show_best = ui.max_rect().width() > best_cutoff;

                ui.horizontal(|ui| {
                    // Show last averages
                    ui.allocate_ui(
                        Vec2::new(
                            if show_best {
                                (ui.max_rect().width() - 24.0) / 2.0
                            } else {
                                ui.max_rect().width()
                            },
                            ui.max_rect().height(),
                        ),
                        |ui| {
                            ui.vertical(|ui| {
                                Self::session_time(
                                    ui,
                                    "Last ao5",
                                    true,
                                    self.last_ao5
                                        .clone()
                                        .map(|avg| SessionTime::AverageOfN(avg)),
                                    details,
                                );
                                Self::session_time(
                                    ui,
                                    "Last ao12",
                                    true,
                                    self.last_ao12
                                        .clone()
                                        .map(|avg| SessionTime::AverageOfN(avg)),
                                    details,
                                );
                                Self::session_time(
                                    ui,
                                    "Session avg",
                                    true,
                                    self.session_avg.map(|avg| SessionTime::SessionAverage(avg)),
                                    details,
                                );
                            });
                        },
                    );

                    if show_best {
                        // Show separator between last averages and best averages
                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Disabled.into(),
                            };
                            ui.separator();
                        });

                        // Show best averages
                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 24.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    Self::session_time(
                                        ui,
                                        "Best solve",
                                        true,
                                        self.best_solve
                                            .clone()
                                            .map(|best| SessionTime::BestSolve(best)),
                                        details,
                                    );
                                    Self::session_time(
                                        ui,
                                        "Best ao5",
                                        true,
                                        self.best_ao5
                                            .clone()
                                            .map(|avg| SessionTime::AverageOfN(avg)),
                                        details,
                                    );
                                    Self::session_time(
                                        ui,
                                        "Best ao12",
                                        true,
                                        self.best_ao12
                                            .clone()
                                            .map(|avg| SessionTime::AverageOfN(avg)),
                                        details,
                                    );
                                });
                            },
                        );
                    }
                });
            }

            ui.add_space(4.0);
        });
    }

    pub fn last_solve_time(&self) -> Option<DateTime<Local>> {
        if let Some(solve) = self.solves.last() {
            Some(solve.created)
        } else {
            None
        }
    }

    pub fn new_session(&mut self, history: &mut History) {
        history.new_session();
        self.update(history);
    }
}
