use crate::app::SolveDetails;
use crate::font::FontSize;
use crate::style::content_visuals;
use crate::theme::Theme;
use crate::widgets::{date_string, solve_time_string};
use egui::{
    containers::ScrollArea, popup_below_widget, Align2, CentralPanel, CtxRef, CursorIcon, Pos2,
    Rect, SelectableLabel, Sense, Stroke, Ui, Vec2,
};
use tpscube_core::{Average, BestSolve, History, ListAverage, Penalty, Solve, SolveList};

const REGION_PADDING: f32 = 16.0;
const SESSION_REGION_BORDER: f32 = 8.0;
const SESSION_SEPARATOR_SIZE: f32 = 16.0;
const SESSION_BEST_PADDING: f32 = 32.0;
const BEST_TIME_COL_PADDING: f32 = 32.0;
const BEST_TIME_ROW_PADDING: f32 = 8.0;

trait HistoryRegion {
    fn height(&self, ui: &Ui, layout_metrics: &SolveLayoutMetrics) -> f32;
    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        layout_metrics: &SolveLayoutMetrics,
        history: &mut History,
        all_time_best: &Option<AllTimeBestRegion>,
        details: &mut Option<SolveDetails>,
    );
}

struct NoSolvesRegion;

struct AllTimeBestRegion {
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
}

struct SessionRegion {
    session_id: String,
    name: String,
    solves: Vec<Solve>,
    last_solve: Solve,
    rows: usize,
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
    average: Option<u32>,
}

struct HistoryRegionLayout {
    region: Box<dyn HistoryRegion>,
    y: f32,
    height: f32,
}

pub struct HistoryWidget {
    regions: Vec<HistoryRegionLayout>,
    all_time_best_region: Option<AllTimeBestRegion>,
    total_height: f32,
    cached_update_id: Option<u64>,
    cached_best_columns: usize,
    cached_solve_columns: usize,
}

struct SolveLayoutMetrics {
    solve_number_width: f32,
    solve_time_width: f32,
    solve_penalty_width: f32,
    solve_menu_width: f32,
    total_solve_width: f32,
    best_solve_width: f32,
    best_columns: usize,
    solve_columns: usize,
    solve_content_width: f32,
}

impl HistoryRegion for NoSolvesRegion {
    fn height(&self, ui: &Ui, _layout_metrics: &SolveLayoutMetrics) -> f32 {
        ui.max_rect().height() - 32.0
    }

    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        _layout_metrics: &SolveLayoutMetrics,
        _history: &mut History,
        _all_time_best: &Option<AllTimeBestRegion>,
        _details: &mut Option<SolveDetails>,
    ) {
        let galley = ui.fonts().layout_multiline(
            FontSize::Section.into(),
            "There are no solves available. If you have other devices to sync with, \
            set your sync key in Settings."
                .into(),
            rect.width(),
        );
        ui.painter().galley(
            Pos2::new(
                rect.center().x - galley.size.x / 2.0,
                rect.center().y - galley.size.y / 2.0,
            ),
            galley,
            Theme::Disabled.into(),
        );
    }
}

impl AllTimeBestRegion {
    fn columns(&self) -> usize {
        let mut best_count = 0;
        if self.best_solve.is_some() {
            best_count += 1;
        }
        if self.best_ao5.is_some() {
            best_count += 1;
        }
        if self.best_ao12.is_some() {
            best_count += 1;
        }
        best_count
    }
}

impl HistoryRegion for AllTimeBestRegion {
    fn height(&self, ui: &Ui, layout_metrics: &SolveLayoutMetrics) -> f32 {
        let rows = (self.columns() + layout_metrics.best_columns - 1) / layout_metrics.best_columns;
        rows as f32
            * (ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING)
            - BEST_TIME_ROW_PADDING
    }

    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        layout_metrics: &SolveLayoutMetrics,
        _history: &mut History,
        _all_time_best: &Option<AllTimeBestRegion>,
        details: &mut Option<SolveDetails>,
    ) {
        let mut best_count = self.columns();
        let mut row_columns_left = if best_count <= layout_metrics.best_columns {
            best_count
        } else if layout_metrics.best_columns == 2 {
            layout_metrics
                .best_columns
                .min(best_count - layout_metrics.best_columns)
        } else {
            layout_metrics.best_columns
        };

        let mut x = rect.center().x
            - (row_columns_left as f32 * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                - BEST_TIME_COL_PADDING)
                / 2.0;
        let mut y = rect.top();

        // Draw best solve
        if let Some(solve) = &self.best_solve {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best solve".into());
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y,
                ),
                galley,
                Theme::Content.into(),
            );

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::BestTime.into(), solve_time_string(solve.time));
            let rect = Rect::from_min_size(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y + ui.fonts().row_height(FontSize::Normal.into()),
                ),
                galley.size,
            );
            let interact = ui.allocate_rect(rect, Sense::click());
            ui.painter().galley(
                rect.left_top(),
                galley,
                if interact.hovered() {
                    Theme::Yellow.into()
                } else {
                    Theme::Orange.into()
                },
            );

            // Check for click on solve time
            if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                *details = Some(SolveDetails::IndividualSolve(solve.solve.clone()));
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = best_count.min(layout_metrics.best_columns);
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw best average of 5
        if let Some(average) = &self.best_ao5 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 5".into());
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y,
                ),
                galley,
                Theme::Content.into(),
            );

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::BestTime.into(), solve_time_string(average.time));
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y + ui.fonts().row_height(FontSize::Normal.into()),
                ),
                galley,
                Theme::Orange.into(),
            );
            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = best_count.min(layout_metrics.best_columns);
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw best average of 12
        if let Some(average) = &self.best_ao12 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 12".into());
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y,
                ),
                galley,
                Theme::Content.into(),
            );

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::BestTime.into(), solve_time_string(average.time));
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y + ui.fonts().row_height(FontSize::Normal.into()),
                ),
                galley,
                Theme::Orange.into(),
            );
        }
    }
}

impl HistoryRegion for SessionRegion {
    fn height(&self, ui: &Ui, layout_metrics: &SolveLayoutMetrics) -> f32 {
        // Layout best solve and average region to determine line wrapping
        let mut x = 0.0;
        let mut lines = 1;
        if let Some(best_solve) = &self.best_solve {
            x += ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best solve: ".into())
                .size
                .x;
            x += ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_solve.time))
                .size
                .x
                + SESSION_BEST_PADDING;
        }

        if let Some(best_ao5) = &self.best_ao5 {
            let width = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 5: ".into())
                .size
                .x
                + ui.fonts()
                    .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao5.time))
                    .size
                    .x
                + SESSION_BEST_PADDING;
            if (x + width) > layout_metrics.solve_content_width {
                x = 0.0;
                lines += 1;
            }
            x += width;
        }

        if let Some(best_ao12) = &self.best_ao12 {
            let width = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 12: ".into())
                .size
                .x
                + ui.fonts()
                    .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao12.time))
                    .size
                    .x
                + SESSION_BEST_PADDING;
            if (x + width) > layout_metrics.solve_content_width {
                x = 0.0;
                lines += 1;
            }
            x += width;
        }

        if let Some(average) = &self.average {
            let width = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Session avg: ".into())
                .size
                .x
                + ui.fonts()
                    .layout_single_line(FontSize::Normal.into(), solve_time_string(*average))
                    .size
                    .x;
            if (x + width) > layout_metrics.solve_content_width {
                lines += 1;
            }
        }

        ui.fonts().row_height(FontSize::Normal.into()) * ((self.rows + lines) as f32)
            + ui.fonts().row_height(FontSize::Section.into())
            + SESSION_REGION_BORDER
            + SESSION_SEPARATOR_SIZE * 2.0
    }

    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        layout_metrics: &SolveLayoutMetrics,
        history: &mut History,
        all_time_best: &Option<AllTimeBestRegion>,
        details: &mut Option<SolveDetails>,
    ) {
        let (all_time_best_solve, all_time_best_ao5, all_time_best_ao12) = match all_time_best {
            Some(region) => (
                region.best_solve.as_ref().map(|best| best.time),
                region.best_ao5.as_ref().map(|best| best.time),
                region.best_ao12.as_ref().map(|best| best.time),
            ),
            None => (None, None, None),
        };

        // Draw session background
        let shaded_area = match layout_metrics.solve_columns {
            1 => rect.shrink2(Vec2::new(REGION_PADDING, 0.0)),
            _ => Rect::from_center_size(
                rect.center(),
                Vec2::new(
                    layout_metrics.solve_content_width + REGION_PADDING * 2.0,
                    rect.height(),
                ),
            ),
        };
        ui.painter()
            .rect_filled(shaded_area, 0.0, Theme::Background);

        // Draw session name
        let content_area = shaded_area.shrink2(Vec2::new(SESSION_REGION_BORDER, 0.0));

        let mut name = self.name.clone();
        let mut truncated = false;
        while name.len() > 0 {
            let string = if truncated {
                format!("{}…", name)
            } else {
                name.clone()
            };
            let name_galley = ui
                .fonts()
                .layout_single_line(FontSize::Section.into(), string);

            if name_galley.size.x > content_area.width() {
                name.pop();
                truncated = true;
                continue;
            }

            ui.painter()
                .galley(content_area.left_top(), name_galley, Theme::Blue.into());
            break;
        }

        // Draw separator between name and solve list
        let mut y = content_area.top() + ui.fonts().row_height(FontSize::Section.into());
        ui.painter().line_segment(
            [
                Pos2::new(content_area.left(), y + SESSION_SEPARATOR_SIZE / 2.0),
                Pos2::new(content_area.right(), y + SESSION_SEPARATOR_SIZE / 2.0),
            ],
            Stroke {
                width: 1.0,
                color: Theme::DarkBlue.into(),
            },
        );
        y += SESSION_SEPARATOR_SIZE;

        // Draw solves
        let col_width = layout_metrics.total_solve_width + SESSION_SEPARATOR_SIZE;
        let row_height = ui.fonts().row_height(FontSize::Normal.into());
        let mut i = 0;
        for col in 0..layout_metrics.solve_columns {
            if i >= self.solves.len() {
                break;
            }

            for row in 0..self.rows {
                // Draw solve number
                ui.painter().text(
                    Pos2::new(
                        content_area.left() + col as f32 * col_width,
                        y + row as f32 * row_height,
                    ),
                    Align2::LEFT_TOP,
                    format!("{}.", i + 1),
                    FontSize::Normal.into(),
                    Theme::Disabled.into(),
                );

                // Layout solve time for right alignment
                let time = self.solves[i].final_time();
                let galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    match time {
                        Some(time) => solve_time_string(time),
                        None => "DNF".into(),
                    },
                );

                let solve_time_rect = Rect::from_min_size(
                    Pos2::new(
                        content_area.left()
                            + col as f32 * col_width
                            + layout_metrics.solve_number_width
                            + layout_metrics.solve_time_width
                            - galley.size.x,
                        y + row as f32 * row_height,
                    ),
                    galley.size,
                );

                // Draw solve time
                let interact = ui.allocate_rect(solve_time_rect, Sense::click());
                ui.painter().galley(
                    solve_time_rect.left_top(),
                    galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else {
                        match time {
                            Some(_) => {
                                if time == all_time_best_solve {
                                    Theme::Orange.into()
                                } else if time == self.best_solve.as_ref().map(|best| best.time) {
                                    Theme::Green.into()
                                } else {
                                    Theme::Content.into()
                                }
                            }
                            None => Theme::Red.into(),
                        }
                    },
                );

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::IndividualSolve(self.solves[i].clone()));
                }

                if let Penalty::Time(penalty) = self.solves[i].penalty {
                    // Draw penalty
                    ui.painter().text(
                        Pos2::new(
                            content_area.left()
                                + col as f32 * col_width
                                + layout_metrics.solve_number_width
                                + layout_metrics.solve_time_width,
                            y + row as f32 * row_height
                                + ui.fonts().row_height(FontSize::Normal.into()),
                        ),
                        Align2::LEFT_BOTTOM,
                        format!(" (+{})", penalty / 1000),
                        FontSize::Small.into(),
                        Theme::Red.into(),
                    );
                } else if self.solves[i].moves.is_some() {
                    // Draw icon to show move data is available
                    let icon_rect = Rect::from_min_size(
                        Pos2::new(
                            content_area.left()
                                + col as f32 * col_width
                                + layout_metrics.solve_number_width
                                + layout_metrics.solve_time_width,
                            y + row as f32 * row_height
                                + ui.fonts().row_height(FontSize::Normal.into())
                                - ui.fonts().row_height(FontSize::Small.into()),
                        ),
                        Vec2::new(
                            layout_metrics.solve_penalty_width,
                            ui.fonts().row_height(FontSize::Small.into()),
                        ),
                    );
                    ui.allocate_rect(icon_rect, Sense::hover())
                        .on_hover_text("Analysis available for this solve");
                    ui.painter().text(
                        icon_rect.left_bottom(),
                        Align2::LEFT_BOTTOM,
                        "   📊",
                        FontSize::Small.into(),
                        Theme::Light.into(),
                    );
                }

                // Draw menu
                let menu_rect = Rect::from_min_size(
                    Pos2::new(
                        content_area.left()
                            + col as f32 * col_width
                            + layout_metrics.solve_number_width
                            + layout_metrics.solve_time_width
                            + layout_metrics.solve_penalty_width,
                        y + row as f32 * row_height
                            + ui.fonts().row_height(FontSize::Normal.into())
                            - ui.fonts().row_height(FontSize::Small.into()),
                    ),
                    Vec2::new(
                        layout_metrics.solve_menu_width,
                        ui.fonts().row_height(FontSize::Small.into()),
                    ),
                );
                let interact = ui.allocate_rect(menu_rect, Sense::click());
                ui.painter().text(
                    Pos2::new(menu_rect.left(), menu_rect.bottom()),
                    Align2::LEFT_BOTTOM,
                    " ☰",
                    FontSize::Small.into(),
                    if interact.hovered() {
                        Theme::Content.into()
                    } else {
                        Theme::Disabled.into()
                    },
                );

                // Check for menu interaction
                let popup_id = ui.make_persistent_id(format!("history-{}", self.solves[i].id));
                if interact.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                let old_visuals = ui.ctx().style().visuals.clone();
                ui.ctx().set_visuals(crate::style::popup_visuals());
                popup_below_widget(ui, popup_id, &interact, |ui| {
                    ui.set_min_width(180.0);
                    if ui
                        .add(
                            SelectableLabel::new(
                                match self.solves[i].penalty {
                                    Penalty::None => true,
                                    _ => false,
                                },
                                "No penalty",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(self.solves[i].id.clone(), Penalty::None);
                        let _ = history.local_commit();
                    }

                    if ui
                        .add(
                            SelectableLabel::new(
                                match self.solves[i].penalty {
                                    Penalty::Time(2000) => true,
                                    _ => false,
                                },
                                "2 second penalty",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(self.solves[i].id.clone(), Penalty::Time(2000));
                        let _ = history.local_commit();
                    }

                    if ui
                        .add(
                            SelectableLabel::new(
                                match self.solves[i].penalty {
                                    Penalty::DNF => true,
                                    _ => false,
                                },
                                "DNF",
                            )
                            .text_style(FontSize::Normal.into()),
                        )
                        .clicked()
                    {
                        history.penalty(self.solves[i].id.clone(), Penalty::DNF);
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
                        history.delete_solve(self.solves[i].id.clone());
                        let _ = history.local_commit();
                    }
                });
                ui.ctx().set_visuals(old_visuals);

                i += 1;
                if i >= self.solves.len() {
                    break;
                }
            }

            // Draw column separator
            let x = content_area.left()
                + col as f32 * col_width
                + layout_metrics.total_solve_width
                + SESSION_SEPARATOR_SIZE / 2.0;
            ui.painter().line_segment(
                [
                    Pos2::new(x, y),
                    Pos2::new(x, y + self.rows as f32 * row_height),
                ],
                Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                },
            );
        }
        y += self.rows as f32 * row_height;

        // Draw separator between solves and best times
        ui.painter().line_segment(
            [
                Pos2::new(content_area.left(), y + SESSION_SEPARATOR_SIZE / 2.0),
                Pos2::new(content_area.right(), y + SESSION_SEPARATOR_SIZE / 2.0),
            ],
            Stroke {
                width: 1.0,
                color: Theme::DarkBlue.into(),
            },
        );
        y += SESSION_SEPARATOR_SIZE;

        // Draw best solve
        let mut x = content_area.left();
        let max_x = x + layout_metrics.solve_content_width;
        if let Some(best_solve) = &self.best_solve {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best solve: ".into());
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Disabled.into());
            x += width;

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_solve.time));
            let width = galley.size.x;
            ui.painter().galley(
                Pos2::new(x, y),
                galley,
                if Some(best_solve.time) == all_time_best_solve {
                    Theme::Orange.into()
                } else {
                    Theme::Content.into()
                },
            );
            x += width + SESSION_BEST_PADDING;
        }

        // Draw best average of 5
        if let Some(best_ao5) = &self.best_ao5 {
            let label_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 5: ".into());
            let time_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao5.time));
            let label_width = label_galley.size.x;
            let time_width = time_galley.size.x;
            let width = label_width + time_width + SESSION_BEST_PADDING;

            if (x + width) > max_x {
                x = content_area.left();
                y += ui.fonts().row_height(FontSize::Normal.into());
            }

            ui.painter()
                .galley(Pos2::new(x, y), label_galley, Theme::Disabled.into());
            ui.painter().galley(
                Pos2::new(x + label_width, y),
                time_galley,
                if Some(best_ao5.time) == all_time_best_ao5 {
                    Theme::Orange.into()
                } else {
                    Theme::Content.into()
                },
            );
            x += width;
        }

        // Draw best average of 12
        if let Some(best_ao12) = &self.best_ao12 {
            let label_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 12: ".into());
            let time_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao12.time));
            let label_width = label_galley.size.x;
            let time_width = time_galley.size.x;
            let width = label_width + time_width + SESSION_BEST_PADDING;

            if (x + width) > max_x {
                x = content_area.left();
                y += ui.fonts().row_height(FontSize::Normal.into());
            }

            ui.painter()
                .galley(Pos2::new(x, y), label_galley, Theme::Disabled.into());
            ui.painter().galley(
                Pos2::new(x + label_width, y),
                time_galley,
                if Some(best_ao12.time) == all_time_best_ao12 {
                    Theme::Orange.into()
                } else {
                    Theme::Content.into()
                },
            );
            x += width;
        }

        // Draw session average
        if let Some(average) = &self.average {
            let label_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Session avg: ".into());
            let time_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(*average));
            let label_width = label_galley.size.x;
            let time_width = time_galley.size.x;
            let width = label_width + time_width;

            if (x + width) > max_x {
                x = content_area.left();
                y += ui.fonts().row_height(FontSize::Normal.into());
            }

            ui.painter()
                .galley(Pos2::new(x, y), label_galley, Theme::Disabled.into());
            ui.painter().galley(
                Pos2::new(x + label_width, y),
                time_galley,
                Theme::Content.into(),
            );
        }
    }
}

impl HistoryWidget {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            all_time_best_region: None,
            total_height: 0.0,
            cached_update_id: None,
            cached_best_columns: 0,
            cached_solve_columns: 0,
        }
    }

    fn generate_regions(
        &mut self,
        ui: &Ui,
        layout_metrics: &SolveLayoutMetrics,
        history: &mut History,
    ) {
        let mut all_time_best_solve: Option<BestSolve> = None;
        let mut all_time_best_ao5: Option<Average> = None;
        let mut all_time_best_ao12: Option<Average> = None;

        // Go through sessions, gather data about them, and create regions for them
        let mut session_regions = Vec::new();
        for session in history.sessions().values() {
            let solves: Vec<Solve> = session.to_vec(history);
            if solves.len() == 0 {
                // Skip empty sessions
                continue;
            }

            let last_solve = solves.last().unwrap().clone();

            // Get averages and bests
            let average = solves.as_slice().average();
            let best_solve = solves.as_slice().best();
            let best_ao5 = solves.as_slice().best_average(5);
            let best_ao12 = solves.as_slice().best_average(12);

            // Check for all time best solve
            if let Some(current_best) = &all_time_best_solve {
                if let Some(session_best) = &best_solve {
                    if session_best.time < current_best.time {
                        all_time_best_solve = best_solve.clone();
                    }
                }
            } else {
                all_time_best_solve = best_solve.clone();
            }

            // Check for all time best average of 5
            if let Some(current_best) = &all_time_best_ao5 {
                if let Some(session_best) = &best_ao5 {
                    if session_best.time < current_best.time {
                        all_time_best_ao5 = best_ao5.clone();
                    }
                }
            } else {
                all_time_best_ao5 = best_ao5.clone();
            }

            // Check for all time best average of 12
            if let Some(current_best) = &all_time_best_ao12 {
                if let Some(session_best) = &best_ao12 {
                    if session_best.time < current_best.time {
                        all_time_best_ao12 = best_ao12.clone();
                    }
                }
            } else {
                all_time_best_ao12 = best_ao12.clone();
            }

            // Calculate number of rows based on number of columns
            let rows =
                (solves.len() + layout_metrics.solve_columns - 1) / layout_metrics.solve_columns;

            // Construct session title
            let name = match session.name() {
                Some(name) => format!("{} - {}", &name, date_string(&last_solve.created)),
                None => date_string(&last_solve.created),
            };

            // Add the session to the region list
            session_regions.push(SessionRegion {
                session_id: session.id().into(),
                name,
                solves,
                last_solve,
                rows,
                best_solve,
                best_ao5,
                best_ao12,
                average,
            });
        }

        // Sort regions by solve time in descending order
        session_regions.sort_unstable_by(|a, b| b.last_solve.cmp(&a.last_solve));

        // Gather regions in generic form
        let mut regions: Vec<Box<dyn HistoryRegion>> = session_regions
            .drain(..)
            .map(|region| {
                let boxed: Box<dyn HistoryRegion> = Box::new(region);
                boxed
            })
            .collect();

        if regions.len() == 0 {
            // There are no sessions
            regions.push(Box::new(NoSolvesRegion));
            self.all_time_best_region = None;
        } else {
            // Add an all-time best region at the top
            self.all_time_best_region = Some(AllTimeBestRegion {
                best_solve: all_time_best_solve,
                best_ao5: all_time_best_ao5,
                best_ao12: all_time_best_ao12,
            });
        }

        // Lay out regions
        let mut y = 0.0;
        self.regions.clear();
        if let Some(region) = &self.all_time_best_region {
            let height = region.height(ui, layout_metrics);
            y += height + REGION_PADDING;
        }
        for region in regions {
            let height = region.height(ui, layout_metrics);
            self.regions.push(HistoryRegionLayout { region, y, height });
            y += height + REGION_PADDING;
        }
        self.total_height = y;
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            let number_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "9999.".into());
            let solve_time_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "9:59.99".into());
            let solve_penalty_galley = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), " (+2) ".into());
            let solve_menu_galley = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), " ☰".into());
            let best_time_galley = ui
                .fonts()
                .layout_single_line(FontSize::BestTime.into(), "99:59.99".into());
            let total_solve_width = number_galley.size.x
                + solve_time_galley.size.x
                + solve_penalty_galley.size.x
                + solve_menu_galley.size.x;

            let max_session_width =
                ui.max_rect().width() - REGION_PADDING * 2.0 - SESSION_REGION_BORDER * 2.0;
            let solve_columns =
                1.max((max_session_width / (total_solve_width + SESSION_SEPARATOR_SIZE)) as usize);
            let best_columns = 1.max(
                (ui.max_rect().width() / (best_time_galley.size.x + BEST_TIME_COL_PADDING))
                    as usize,
            );

            let solve_layout_metrics = SolveLayoutMetrics {
                solve_number_width: number_galley.size.x,
                solve_time_width: solve_time_galley.size.x,
                solve_penalty_width: solve_penalty_galley.size.x,
                solve_menu_width: solve_menu_galley.size.x,
                total_solve_width,
                best_solve_width: best_time_galley.size.x,
                best_columns,
                solve_columns,
                solve_content_width: (solve_columns as f32
                    * (total_solve_width + SESSION_SEPARATOR_SIZE))
                    - SESSION_SEPARATOR_SIZE,
            };

            if self.cached_update_id != Some(history.update_id())
                || self.cached_solve_columns != solve_columns
                || self.cached_best_columns != best_columns
            {
                self.cached_update_id = Some(history.update_id());
                self.cached_solve_columns = solve_columns;
                self.cached_best_columns = best_columns;
                self.generate_regions(ui, &solve_layout_metrics, history);
            }

            ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
            ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
            ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
            ScrollArea::auto_sized()
                .id_source("history")
                .show_viewport(ui, |ui, viewport| {
                    let (rect, _) = ui.allocate_at_least(
                        Vec2::new(ui.max_rect().width(), self.total_height),
                        Sense::hover(),
                    );
                    if let Some(region) = &self.all_time_best_region {
                        let height = region.height(ui, &solve_layout_metrics);
                        if height >= viewport.top() {
                            region.paint(
                                ui,
                                Rect::from_min_size(
                                    Pos2::new(rect.left(), rect.top()),
                                    Vec2::new(rect.width(), height),
                                ),
                                &solve_layout_metrics,
                                history,
                                &None,
                                details,
                            );
                        }
                    }
                    for region in &self.regions {
                        if region.y > viewport.bottom() {
                            break;
                        }
                        if region.y + region.height < viewport.top() {
                            continue;
                        }

                        region.region.paint(
                            ui,
                            Rect::from_min_size(
                                Pos2::new(rect.left(), rect.top() + region.y),
                                Vec2::new(rect.width(), region.height),
                            ),
                            &solve_layout_metrics,
                            history,
                            &self.all_time_best_region,
                            details,
                        );
                    }
                });
        });
    }
}
