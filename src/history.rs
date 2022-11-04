use crate::algorithms::AlgorithmRender;
use crate::app::SolveDetails;
use crate::font::FontSize;
use crate::style::content_visuals;
use crate::theme::Theme;
use crate::widgets::{date_string, solve_time_string};
use egui::{
    containers::ScrollArea, popup_below_widget, Align2, CentralPanel, CtxRef, CursorIcon, Pos2,
    Rect, SelectableLabel, Sense, Stroke, Ui, Vec2,
};
use tpscube_core::{
    Algorithm, Average, BestSolve, Cube, Cube3x3x3, CubeFace, History, InitialCubeState,
    KnownAlgorithms, ListAverage, OLLAlgorithm, PLLAlgorithm, Penalty, Solve, SolveList, SolveType,
};

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
        layout_metrics_moves: &SolveLayoutMetrics,
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
    best_ao50: Option<Average>,
    best_ao100: Option<Average>,
    running_best_ao50: Option<Average>,
    running_best_ao100: Option<Average>,
    running_last_ao50: Option<Average>,
    running_last_ao100: Option<Average>,
}

struct SessionRegion {
    session_id: String,
    name: String,
    solve_type: SolveType,
    solves: Vec<Solve>,
    has_moves: bool,
    last_solve: Solve,
    rows: usize,
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
    best_ao50: Option<Average>,
    best_ao100: Option<Average>,
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
    cached_solve_type: SolveType,
}

#[derive(Copy, Clone)]
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

pub trait Max<Rhs = Self> {
    type Output;
    fn max(self, rhs: Rhs) -> Self::Output;
}

impl Max<f32> for f32 {
    type Output = f32;
    fn max(self, rhs: f32) -> f32 {
        if self > rhs {
            self
        } else {
            rhs
        }
    }
}

impl Max<SolveLayoutMetrics> for SolveLayoutMetrics {
    type Output = SolveLayoutMetrics;
    fn max(self, rhs: SolveLayoutMetrics) -> SolveLayoutMetrics {
        SolveLayoutMetrics {
            solve_number_width: self.solve_number_width.max(rhs.solve_number_width),
            solve_time_width: self.solve_time_width.max(rhs.solve_time_width),
            solve_penalty_width: self.solve_penalty_width.max(rhs.solve_penalty_width),
            solve_menu_width: self.solve_menu_width.max(rhs.solve_menu_width),
            total_solve_width: self.total_solve_width.max(rhs.total_solve_width),
            best_solve_width: self.best_solve_width.max(rhs.best_solve_width),
            best_columns: self.best_columns.max(rhs.best_columns),
            solve_columns: self.solve_columns.max(rhs.solve_columns),
            solve_content_width: self.solve_content_width.max(rhs.solve_content_width),
        }
    }
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
        _layout_metrics_moves: &SolveLayoutMetrics,
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
    fn columns(&self) -> (usize, usize) {
        let mut best_count = 0;
        let mut running_best_count = 0;
        if self.best_solve.is_some() {
            best_count += 1;
        }
        if self.best_ao5.is_some() {
            best_count += 1;
        }
        if self.best_ao12.is_some() {
            best_count += 1;
        }
        if self.best_ao50.is_some() {
            best_count += 1;
        }
        if self.best_ao100.is_some() {
            best_count += 1;
        }
        if self.running_best_ao50.is_some() {
            running_best_count += 1;
        }
        if self.running_best_ao100.is_some() {
            running_best_count += 1;
        }
        (best_count, running_best_count)
    }
}

impl HistoryRegion for AllTimeBestRegion {
    fn height(&self, ui: &Ui, layout_metrics: &SolveLayoutMetrics) -> f32 {
        let (best_columns, running_best_columns) = self.columns();
        if running_best_columns == 0 {
            let rows =
                (best_columns + layout_metrics.best_columns - 1) / layout_metrics.best_columns;
            rows as f32
                * (ui.fonts().row_height(FontSize::Normal.into())
                    + ui.fonts().row_height(FontSize::BestTime.into())
                    + BEST_TIME_ROW_PADDING)
                - BEST_TIME_ROW_PADDING
        } else if layout_metrics.best_columns == 1 {
            best_columns as f32
                * (ui.fonts().row_height(FontSize::Normal.into())
                    + ui.fonts().row_height(FontSize::BestTime.into())
                    + BEST_TIME_ROW_PADDING)
                + running_best_columns as f32
                    * (ui.fonts().row_height(FontSize::Normal.into())
                        + 2.0 * ui.fonts().row_height(FontSize::Section.into())
                        + BEST_TIME_ROW_PADDING)
                - BEST_TIME_ROW_PADDING
        } else {
            let rows = (best_columns + running_best_columns + layout_metrics.best_columns - 1)
                / layout_metrics.best_columns;
            (rows - 1) as f32
                * (ui.fonts().row_height(FontSize::Normal.into())
                    + ui.fonts().row_height(FontSize::BestTime.into())
                    + BEST_TIME_ROW_PADDING)
                + (ui.fonts().row_height(FontSize::Normal.into())
                    + 2.0 * ui.fonts().row_height(FontSize::Section.into())
                    + BEST_TIME_ROW_PADDING)
                - BEST_TIME_ROW_PADDING
        }
    }

    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        layout_metrics: &SolveLayoutMetrics,
        _layout_metrics_moves: &SolveLayoutMetrics,
        _history: &mut History,
        _all_time_best: &Option<AllTimeBestRegion>,
        details: &mut Option<SolveDetails>,
    ) {
        let (mut best_count, mut running_best_count) = self.columns();
        let mut row_columns_left =
            if (best_count + running_best_count) <= layout_metrics.best_columns {
                best_count + running_best_count
            } else if best_count <= layout_metrics.best_columns {
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
            row_columns_left = if (best_count + running_best_count) <= layout_metrics.best_columns {
                best_count + running_best_count
            } else if best_count == 0 {
                running_best_count.min(layout_metrics.best_columns)
            } else {
                best_count.min(layout_metrics.best_columns)
            };
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = if (best_count + running_best_count) <= layout_metrics.best_columns {
                best_count + running_best_count
            } else if best_count == 0 {
                running_best_count.min(layout_metrics.best_columns)
            } else {
                best_count.min(layout_metrics.best_columns)
            };
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = if (best_count + running_best_count) <= layout_metrics.best_columns {
                best_count + running_best_count
            } else if best_count == 0 {
                running_best_count.min(layout_metrics.best_columns)
            } else {
                best_count.min(layout_metrics.best_columns)
            };
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw best average of 50
        if let Some(average) = &self.best_ao50 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 50".into());
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = if (best_count + running_best_count) <= layout_metrics.best_columns {
                best_count + running_best_count
            } else if best_count == 0 {
                running_best_count.min(layout_metrics.best_columns)
            } else {
                best_count.min(layout_metrics.best_columns)
            };
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw best average of 100
        if let Some(average) = &self.best_ao100 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 100".into());
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = running_best_count.min(layout_metrics.best_columns);
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw running best average of 50
        if let Some(average) = &self.running_best_ao50 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Running ao50".into());
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y,
                ),
                galley,
                Theme::Content.into(),
            );

            let line = format!("Best  {}", solve_time_string(average.time));
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Section.into(), line);
            let rect = Rect::from_min_size(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    // Use smaller font to fit in two rows for running best and last averages
                    y + ui.fonts().row_height(FontSize::Section.into()),
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            // Draw running last average of 50
            if let Some(average) = &self.running_last_ao50 {
                let line = format!("Last  {}", solve_time_string(average.time));
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Section.into(), line);
                let rect = Rect::from_min_size(
                    Pos2::new(
                        x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                        y + ui.fonts().row_height(FontSize::Normal.into())
                            // Use smaller font to fit in two rows for running best and last averages
                            + ui.fonts().row_height(FontSize::Section.into()),
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
                    *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
                }
            }

            x += layout_metrics.best_solve_width + BEST_TIME_COL_PADDING;
            running_best_count -= 1;
            row_columns_left -= 1;
        }

        if row_columns_left == 0 {
            row_columns_left = running_best_count.min(layout_metrics.best_columns);
            x = rect.center().x
                - (row_columns_left as f32
                    * (layout_metrics.best_solve_width + BEST_TIME_COL_PADDING)
                    - BEST_TIME_COL_PADDING)
                    / 2.0;
            y += ui.fonts().row_height(FontSize::Normal.into())
                + 2.0 * ui.fonts().row_height(FontSize::Section.into())
                + BEST_TIME_ROW_PADDING;
        }

        // Draw running best average of 100
        if let Some(average) = &self.running_best_ao100 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Running ao100".into());
            ui.painter().galley(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    y,
                ),
                galley,
                Theme::Content.into(),
            );
            let line = format!("Best  {}", solve_time_string(average.time));
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Section.into(), line);
            let rect = Rect::from_min_size(
                Pos2::new(
                    x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                    // Use smaller font to fit in two rows for running best and last averages
                    y + ui.fonts().row_height(FontSize::Section.into()),
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
                *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
            }

            // Draw running last average of 100
            if let Some(average) = &self.running_last_ao100 {
                let line = format!("Last  {}", solve_time_string(average.time));
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Section.into(), line);
                let rect = Rect::from_min_size(
                    Pos2::new(
                        x + layout_metrics.best_solve_width / 2.0 - galley.size.x / 2.0,
                        y + ui.fonts().row_height(FontSize::Normal.into())
                            // Use smaller font to fit in two rows for running best and last averages
                            + ui.fonts().row_height(FontSize::Section.into()),
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
                    *details = Some(SolveDetails::AverageOfSolves(average.solves.clone()));
                }
            }
        }
    }
}

impl SessionRegion {
    fn paint_standard_solve(
        &self,
        ui: &mut Ui,
        x: f32,
        y: f32,
        i: usize,
        solve: &Solve,
        layout_metrics: &SolveLayoutMetrics,
        history: &mut History,
        all_time_best_solve: Option<u32>,
        details: &mut Option<SolveDetails>,
    ) {
        // Draw solve number
        ui.painter().text(
            Pos2::new(x, y),
            Align2::LEFT_TOP,
            format!("{}.", i + 1),
            FontSize::Normal.into(),
            Theme::Disabled.into(),
        );

        // Layout solve time for right alignment
        let time = solve.final_time();
        let galley = ui.fonts().layout_single_line(
            FontSize::Normal.into(),
            match time {
                Some(time) => format!(
                    "{}{}",
                    solve_time_string(time),
                    if let Some(moves) = &solve.moves {
                        let time = (time + 5) / 10;
                        if time != 0 {
                            let tps = moves.len() as u32 * 1000 / time;
                            format!(" ({}/{}.{})", moves.len(), tps / 10, tps % 10)
                        } else {
                            "".into()
                        }
                    } else {
                        "".into()
                    }
                ),
                None => "DNF".into(),
            },
        );

        let solve_time_rect = Rect::from_min_size(
            Pos2::new(
                x + layout_metrics.solve_number_width + layout_metrics.solve_time_width
                    - galley.size.x,
                y,
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
            *details = Some(SolveDetails::IndividualSolve(solve.clone()));
        }

        if let Penalty::Time(penalty) = solve.penalty {
            // Draw penalty
            ui.painter().text(
                Pos2::new(
                    x + layout_metrics.solve_number_width + layout_metrics.solve_time_width,
                    y + ui.fonts().row_height(FontSize::Normal.into()),
                ),
                Align2::LEFT_BOTTOM,
                format!(" (+{})", penalty / 1000),
                FontSize::Small.into(),
                Theme::Red.into(),
            );
        } else if solve.moves.is_some() {
            // Draw icon to show move data is available
            let icon_rect = Rect::from_min_size(
                Pos2::new(
                    x + layout_metrics.solve_number_width + layout_metrics.solve_time_width,
                    y + ui.fonts().row_height(FontSize::Normal.into())
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
                x + layout_metrics.solve_number_width
                    + layout_metrics.solve_time_width
                    + layout_metrics.solve_penalty_width,
                y + ui.fonts().row_height(FontSize::Normal.into())
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
        let popup_id = ui.make_persistent_id(format!("history-{}", solve.id));
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
                    SelectableLabel::new(false, "Delete solve").text_style(FontSize::Normal.into()),
                )
                .clicked()
            {
                history.delete_solve(solve.id.clone());
                let _ = history.local_commit();
            }
        });
        ui.ctx().set_visuals(old_visuals);
    }

    fn paint_last_layer_training_solve(
        &self,
        ui: &mut Ui,
        x: f32,
        y: f32,
        i: usize,
        solve: &Solve,
        algorithm: Algorithm,
        layout_metrics: &SolveLayoutMetrics,
        history: &mut History,
        details: &mut Option<SolveDetails>,
    ) {
        let moves = match algorithm {
            Algorithm::OLL(oll) => KnownAlgorithms::oll(oll)[0].clone(),
            Algorithm::PLL(pll) => KnownAlgorithms::pll(pll)[0].clone(),
        };

        let cube_size = ui.fonts().row_height(FontSize::Normal.into()) * 1.85;
        algorithm.draw(
            ui,
            &moves,
            ui.fonts().row_height(FontSize::Normal.into()) * 2.0,
            false,
            Some(Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(cube_size, cube_size),
            )),
        );

        // Draw solve number
        ui.painter().text(
            Pos2::new(x + cube_size + 8.0, y),
            Align2::LEFT_TOP,
            format!("{}.", i + 1),
            FontSize::Normal.into(),
            Theme::Disabled.into(),
        );

        // Layout solve time for right alignment
        let time = solve.final_time();
        let galley = ui.fonts().layout_single_line(
            match solve.penalty {
                Penalty::RecognitionDNF | Penalty::ExecutionDNF => FontSize::Small.into(),
                _ => FontSize::Normal.into(),
            },
            match time {
                Some(time) => format!(
                    "{}{}",
                    solve_time_string(time),
                    if let Some(moves) = &solve.moves {
                        let time = (time + 5) / 10;
                        let tps = moves.len() as u32 * 1000 / time;
                        format!(" ({}/{}.{})", moves.len(), tps / 10, tps % 10)
                    } else {
                        format!("")
                    }
                ),
                None => match solve.penalty {
                    Penalty::RecognitionDNF => "Misrecognize".into(),
                    Penalty::ExecutionDNF => "Misexecute".into(),
                    _ => "DNF".into(),
                },
            },
        );

        let solve_time_rect = Rect::from_min_size(
            Pos2::new(
                x + layout_metrics.solve_number_width
                    + layout_metrics.solve_time_width
                    + layout_metrics.solve_penalty_width
                    - galley.size.x,
                y + ui.fonts().row_height(FontSize::Normal.into())
                    + (ui.fonts().row_height(FontSize::Normal.into()) - galley.size.y) / 2.0,
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
                    Some(_) => Theme::Content.into(),
                    None => Theme::Red.into(),
                }
            },
        );

        // Check for click on solve time
        if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
            *details = Some(SolveDetails::IndividualSolve(solve.clone()));
        }

        // Layout algorithm name for right alignment
        let galley = ui.fonts().layout_single_line(
            FontSize::Normal.into(),
            match algorithm {
                Algorithm::OLL(oll) => format!("#{}", oll.as_number()),
                _ => algorithm.to_string(),
            },
        );

        let alg_name_rect = Rect::from_min_size(
            Pos2::new(
                x + layout_metrics.solve_number_width
                    + layout_metrics.solve_time_width
                    + layout_metrics.solve_penalty_width
                    - galley.size.x,
                y,
            ),
            galley.size,
        );

        // Draw algorithm name
        ui.painter()
            .galley(alg_name_rect.left_top(), galley, Theme::Disabled.into());

        // Draw menu
        let menu_rect = Rect::from_min_size(
            Pos2::new(
                x + layout_metrics.solve_number_width
                    + layout_metrics.solve_time_width
                    + layout_metrics.solve_penalty_width,
                y + ui.fonts().row_height(FontSize::Normal.into()) * 2.0
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
        let popup_id = ui.make_persistent_id(format!("history-{}", solve.id));
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
        ui.ctx().set_visuals(old_visuals);
    }
}

impl HistoryRegion for SessionRegion {
    fn height(&self, ui: &Ui, layout_metrics: &SolveLayoutMetrics) -> f32 {
        // Layout best solve and average region to determine line wrapping
        if self.solve_type.is_last_layer_training() {
            ui.fonts().row_height(FontSize::Normal.into()) * (self.rows as f32) * 2.1
                + ui.fonts().row_height(FontSize::Section.into())
                + SESSION_REGION_BORDER
                + SESSION_SEPARATOR_SIZE
        } else {
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
                        .layout_single_line(
                            FontSize::Normal.into(),
                            solve_time_string(best_ao5.time),
                        )
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
                        .layout_single_line(
                            FontSize::Normal.into(),
                            solve_time_string(best_ao12.time),
                        )
                        .size
                        .x
                    + SESSION_BEST_PADDING;
                if (x + width) > layout_metrics.solve_content_width {
                    x = 0.0;
                    lines += 1;
                }
                x += width;
            }

            if let Some(best_ao50) = &self.best_ao50 {
                let width = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), "Best avg of 50: ".into())
                    .size
                    .x
                    + ui.fonts()
                        .layout_single_line(
                            FontSize::Normal.into(),
                            solve_time_string(best_ao50.time),
                        )
                        .size
                        .x
                    + SESSION_BEST_PADDING;
                if (x + width) > layout_metrics.solve_content_width {
                    x = 0.0;
                    lines += 1;
                }
                x += width;
            }

            if let Some(best_ao100) = &self.best_ao100 {
                let width = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), "Best avg of 100: ".into())
                    .size
                    .x
                    + ui.fonts()
                        .layout_single_line(
                            FontSize::Normal.into(),
                            solve_time_string(best_ao100.time),
                        )
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
    }

    fn paint(
        &self,
        ui: &mut Ui,
        rect: Rect,
        layout_metrics: &SolveLayoutMetrics,
        layout_metrics_moves: &SolveLayoutMetrics,
        history: &mut History,
        all_time_best: &Option<AllTimeBestRegion>,
        details: &mut Option<SolveDetails>,
    ) {
        let solves_layout_metrics = if self.has_moves {
            layout_metrics_moves
        } else {
            layout_metrics
        };

        let layout_metrics = layout_metrics.max(*layout_metrics_moves);

        let (
            all_time_best_solve,
            all_time_best_ao5,
            all_time_best_ao12,
            all_time_best_ao50,
            all_time_best_ao100,
        ) = match all_time_best {
            Some(region) => (
                region.best_solve.as_ref().map(|best| best.time),
                region.best_ao5.as_ref().map(|best| best.time),
                region.best_ao12.as_ref().map(|best| best.time),
                region.best_ao50.as_ref().map(|best| best.time),
                region.best_ao100.as_ref().map(|best| best.time),
            ),
            None => (None, None, None, None, None),
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
        let col_width = solves_layout_metrics.total_solve_width + SESSION_SEPARATOR_SIZE;
        let row_height = ui.fonts().row_height(FontSize::Normal.into());
        let mut i = 0;
        for col in 0..solves_layout_metrics.solve_columns {
            if i >= self.solves.len() {
                break;
            }

            for row in 0..self.rows {
                match self.solve_type {
                    SolveType::OLLTraining => {
                        let mut cube = Cube3x3x3::new();
                        cube.do_moves(&self.solves[i].scramble);
                        let algorithm = OLLAlgorithm::from_cube(&cube.as_faces(), CubeFace::Top);
                        if let Some(algorithm) = algorithm {
                            self.paint_last_layer_training_solve(
                                ui,
                                content_area.left() + col as f32 * col_width,
                                y + row as f32 * row_height * 2.1,
                                i,
                                &self.solves[i],
                                Algorithm::OLL(algorithm),
                                solves_layout_metrics,
                                history,
                                details,
                            )
                        } else {
                            self.paint_standard_solve(
                                ui,
                                content_area.left() + col as f32 * col_width,
                                y + row as f32 * row_height * 2.1,
                                i,
                                &self.solves[i],
                                solves_layout_metrics,
                                history,
                                all_time_best_solve,
                                details,
                            )
                        }
                    }
                    SolveType::PLLTraining => {
                        let mut cube = Cube3x3x3::new();
                        cube.do_moves(&self.solves[i].scramble);
                        let algorithm = PLLAlgorithm::from_cube(&cube.as_faces(), CubeFace::Top);
                        if let Some(algorithm) = algorithm {
                            self.paint_last_layer_training_solve(
                                ui,
                                content_area.left() + col as f32 * col_width,
                                y + row as f32 * row_height * 2.1,
                                i,
                                &self.solves[i],
                                Algorithm::PLL(algorithm),
                                solves_layout_metrics,
                                history,
                                details,
                            )
                        } else {
                            self.paint_standard_solve(
                                ui,
                                content_area.left() + col as f32 * col_width,
                                y + row as f32 * row_height * 2.1,
                                i,
                                &self.solves[i],
                                solves_layout_metrics,
                                history,
                                all_time_best_solve,
                                details,
                            )
                        }
                    }
                    _ => self.paint_standard_solve(
                        ui,
                        content_area.left() + col as f32 * col_width,
                        y + row as f32 * row_height,
                        i,
                        &self.solves[i],
                        solves_layout_metrics,
                        history,
                        all_time_best_solve,
                        details,
                    ),
                }

                i += 1;
                if i >= self.solves.len() {
                    break;
                }
            }

            // Draw column separator
            let x = content_area.left()
                + col as f32 * col_width
                + solves_layout_metrics.total_solve_width
                + SESSION_SEPARATOR_SIZE / 2.0;
            ui.painter().line_segment(
                [
                    Pos2::new(x, y),
                    Pos2::new(
                        x,
                        y + self.rows as f32
                            * row_height
                            * if self.solve_type.is_last_layer_training() {
                                2.1
                            } else {
                                1.0
                            },
                    ),
                ],
                Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                },
            );
        }

        if !self.solve_type.is_last_layer_training() {
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

                let galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    solve_time_string(best_solve.time),
                );
                let width = galley.size.x;
                let rect = Rect::from_min_size(Pos2::new(x, y), galley.size);
                let interact = ui.allocate_rect(rect, Sense::click());
                ui.painter().galley(
                    rect.left_top(),
                    galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else if Some(best_solve.time) == all_time_best_solve {
                        Theme::Orange.into()
                    } else {
                        Theme::Content.into()
                    },
                );
                x += width + SESSION_BEST_PADDING;

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::IndividualSolve(best_solve.solve.clone()));
                }
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
                let rect = Rect::from_min_size(Pos2::new(x + label_width, y), time_galley.size);
                let interact = ui.allocate_rect(rect, Sense::click());
                ui.painter().galley(
                    rect.left_top(),
                    time_galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else if Some(best_ao5.time) == all_time_best_ao5 {
                        Theme::Orange.into()
                    } else {
                        Theme::Content.into()
                    },
                );
                x += width;

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::AverageOfSolves(best_ao5.solves.clone()));
                }
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
                let rect = Rect::from_min_size(Pos2::new(x + label_width, y), time_galley.size);
                let interact = ui.allocate_rect(rect, Sense::click());
                ui.painter().galley(
                    rect.left_top(),
                    time_galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else if Some(best_ao12.time) == all_time_best_ao12 {
                        Theme::Orange.into()
                    } else {
                        Theme::Content.into()
                    },
                );
                x += width;

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::AverageOfSolves(best_ao12.solves.clone()));
                }
            }

            // Draw best average of 50
            if let Some(best_ao50) = &self.best_ao50 {
                let label_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), "Best avg of 50: ".into());
                let time_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao50.time));
                let label_width = label_galley.size.x;
                let time_width = time_galley.size.x;
                let width = label_width + time_width + SESSION_BEST_PADDING;

                if (x + width) > max_x {
                    x = content_area.left();
                    y += ui.fonts().row_height(FontSize::Normal.into());
                }

                ui.painter()
                    .galley(Pos2::new(x, y), label_galley, Theme::Disabled.into());
                let rect = Rect::from_min_size(Pos2::new(x + label_width, y), time_galley.size);
                let interact = ui.allocate_rect(rect, Sense::click());
                ui.painter().galley(
                    rect.left_top(),
                    time_galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else if Some(best_ao50.time) == all_time_best_ao50 {
                        Theme::Orange.into()
                    } else {
                        Theme::Content.into()
                    },
                );
                x += width;

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::AverageOfSolves(best_ao50.solves.clone()));
                }
            }

            // Draw best average of 100
            if let Some(best_ao100) = &self.best_ao100 {
                let label_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), "Best avg of 100: ".into());
                let time_galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    solve_time_string(best_ao100.time),
                );
                let label_width = label_galley.size.x;
                let time_width = time_galley.size.x;
                let width = label_width + time_width + SESSION_BEST_PADDING;

                if (x + width) > max_x {
                    x = content_area.left();
                    y += ui.fonts().row_height(FontSize::Normal.into());
                }

                ui.painter()
                    .galley(Pos2::new(x, y), label_galley, Theme::Disabled.into());
                let rect = Rect::from_min_size(Pos2::new(x + label_width, y), time_galley.size);
                let interact = ui.allocate_rect(rect, Sense::click());
                ui.painter().galley(
                    rect.left_top(),
                    time_galley,
                    if interact.hovered() {
                        Theme::Blue.into()
                    } else if Some(best_ao100.time) == all_time_best_ao100 {
                        Theme::Orange.into()
                    } else {
                        Theme::Content.into()
                    },
                );
                x += width;

                // Check for click on solve time
                if interact.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                    *details = Some(SolveDetails::AverageOfSolves(best_ao100.solves.clone()));
                }
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
            cached_solve_type: SolveType::Standard3x3x3,
        }
    }

    fn generate_regions(
        &mut self,
        ui: &Ui,
        layout_metrics: &SolveLayoutMetrics,
        layout_metrics_moves: &SolveLayoutMetrics,
        history: &mut History,
        solve_type: SolveType,
    ) {
        let mut all_time_best_solve: Option<BestSolve> = None;
        let mut all_time_best_ao5: Option<Average> = None;
        let mut all_time_best_ao12: Option<Average> = None;
        let mut all_time_best_ao50: Option<Average> = None;
        let mut all_time_best_ao100: Option<Average> = None;

        // Go through sessions, gather data about them, and create regions for them
        let mut session_regions = Vec::new();
        let mut all_solves: Vec<Solve> = Vec::new();
        for session in history.sessions().values() {
            let solves: Vec<Solve> = session.to_vec(history);
            let has_moves = solves
                .as_slice()
                .into_iter()
                .any(|s| s.moves.is_some() && s.moves.as_ref().unwrap().len() > 0);
            if solves.len() == 0 {
                // Skip empty sessions
                continue;
            }
            if session.solve_type() != solve_type {
                // Skip sessions that aren't the active solve type
                continue;
            }
            all_solves.extend_from_slice(solves.as_slice());

            let layout_metrics = if has_moves {
                layout_metrics_moves
            } else {
                layout_metrics
            };

            let last_solve = solves.last().unwrap().clone();

            // Get averages and bests
            let average = solves.as_slice().average();
            let best_solve = solves.as_slice().best();
            let best_ao5 = solves.as_slice().best_average(5);
            let best_ao12 = solves.as_slice().best_average(12);
            let best_ao50 = solves.as_slice().best_average(50);
            let best_ao100 = solves.as_slice().best_average(100);

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

            // Check for all time best average of 50
            if let Some(current_best) = &all_time_best_ao50 {
                if let Some(session_best) = &best_ao50 {
                    if session_best.time < current_best.time {
                        all_time_best_ao50 = best_ao50.clone();
                    }
                }
            } else {
                all_time_best_ao50 = best_ao50.clone();
            }

            // Check for all time best average of 100
            if let Some(current_best) = &all_time_best_ao100 {
                if let Some(session_best) = &best_ao100 {
                    if session_best.time < current_best.time {
                        all_time_best_ao100 = best_ao100.clone();
                    }
                }
            } else {
                all_time_best_ao100 = best_ao100.clone();
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
                solve_type,
                solves,
                has_moves,
                last_solve,
                rows,
                best_solve,
                best_ao5,
                best_ao12,
                best_ao50,
                best_ao100,
                average,
            });
        }

        all_solves.sort_by_key(|solve| solve.created);

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
            if solve_type.is_last_layer_training() {
                self.all_time_best_region = None;
            } else {
                let running_best_ao50 = all_solves.as_slice().best_average(50);
                let running_last_ao50 = all_solves.as_slice().last_average(50);
                let running_best_ao100 = all_solves.as_slice().best_average(100);
                let running_last_ao100 = all_solves.as_slice().last_average(100);

                // Add an all-time best region at the top
                self.all_time_best_region = Some(AllTimeBestRegion {
                    best_solve: all_time_best_solve,
                    best_ao5: all_time_best_ao5,
                    best_ao12: all_time_best_ao12,
                    best_ao50: all_time_best_ao50,
                    best_ao100: all_time_best_ao100,
                    running_best_ao50: running_best_ao50,
                    running_best_ao100: running_best_ao100,
                    running_last_ao50: running_last_ao50,
                    running_last_ao100: running_last_ao100,
                });
            }
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
        solve_type: SolveType,
    ) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            let number_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "9999.".into());
            let solve_time_moves_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "9:59.99 (999/9.9)".into());
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
            let total_solve_moves_width = number_galley.size.x
                + solve_time_moves_galley.size.x
                + solve_penalty_galley.size.x
                + solve_menu_galley.size.x;

            let max_session_width =
                ui.max_rect().width() - REGION_PADDING * 2.0 - SESSION_REGION_BORDER * 2.0;
            let solve_columns =
                1.max((max_session_width / (total_solve_width + SESSION_SEPARATOR_SIZE)) as usize);
            let solve_columns_moves = 1.max(
                (max_session_width / (total_solve_moves_width + SESSION_SEPARATOR_SIZE)) as usize,
            );
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

            let solve_layout_metrics_moves = SolveLayoutMetrics {
                solve_number_width: number_galley.size.x,
                solve_time_width: solve_time_moves_galley.size.x,
                solve_penalty_width: solve_penalty_galley.size.x,
                solve_menu_width: solve_menu_galley.size.x,
                total_solve_width: total_solve_moves_width,
                best_solve_width: best_time_galley.size.x,
                best_columns,
                solve_columns: solve_columns_moves,
                solve_content_width: (solve_columns_moves as f32
                    * (total_solve_moves_width + SESSION_SEPARATOR_SIZE))
                    - SESSION_SEPARATOR_SIZE,
            };

            if self.cached_update_id != Some(history.update_id())
                || self.cached_solve_columns != solve_columns
                || self.cached_best_columns != best_columns
                || self.cached_solve_type != solve_type
            {
                self.cached_update_id = Some(history.update_id());
                self.cached_solve_columns = solve_columns;
                self.cached_best_columns = best_columns;
                self.cached_solve_type = solve_type;
                self.generate_regions(
                    ui,
                    &solve_layout_metrics,
                    &solve_layout_metrics_moves,
                    history,
                    solve_type,
                );
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
                                &solve_layout_metrics_moves,
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
                            &solve_layout_metrics_moves,
                            history,
                            &self.all_time_best_region,
                            details,
                        );
                    }
                });
        });
    }
}
