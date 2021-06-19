use crate::font::FontSize;
use crate::style::content_visuals;
use crate::theme::Theme;
use crate::widgets::{date_string, solve_time_string};
use egui::{
    containers::ScrollArea, Align2, CentralPanel, CtxRef, Pos2, Rect, Sense, Stroke, Ui, Vec2,
};
use tpscube_core::{Average, BestSolve, History, Solve, SolveList};

const REGION_PADDING: f32 = 16.0;
const SESSION_REGION_BORDER: f32 = 8.0;
const SESSION_SEPARATOR_SIZE: f32 = 16.0;
const SESSION_BEST_PADDING: f32 = 32.0;
const BEST_TIME_COL_PADDING: f32 = 32.0;
const BEST_TIME_ROW_PADDING: f32 = 8.0;

trait HistoryRegion {
    fn height(&self, ui: &Ui) -> f32;
    fn paint(&self, ui: &Ui, rect: Rect, layout_metrics: &SolveLayoutMetrics);
}

struct AllTimeBestRegion {
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
    max_columns: usize,
}

struct SessionRegion {
    session_id: String,
    name: Option<String>,
    solves: Vec<Solve>,
    last_solve: Solve,
    rows: usize,
    columns: usize,
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
    total_height: f32,
    cached_update_id: Option<u64>,
    cached_best_columns: usize,
    cached_solve_columns: usize,
}

struct SolveLayoutMetrics {
    solve_number_width: f32,
    solve_time_width: f32,
    total_width: f32,
    best_solve_width: f32,
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
    fn height(&self, ui: &Ui) -> f32 {
        let rows = (self.columns() + self.max_columns - 1) / self.max_columns;
        rows as f32
            * (ui.fonts().row_height(FontSize::Normal.into())
                + ui.fonts().row_height(FontSize::BestTime.into())
                + BEST_TIME_ROW_PADDING)
            - BEST_TIME_ROW_PADDING
    }

    fn paint(&self, ui: &Ui, rect: Rect, layout_metrics: &SolveLayoutMetrics) {
        let mut best_count = self.columns();
        let mut row_columns_left = if best_count <= self.max_columns {
            best_count
        } else if self.max_columns == 2 {
            self.max_columns.min(best_count - self.max_columns)
        } else {
            self.max_columns
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
            row_columns_left = best_count.min(self.max_columns);
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
            row_columns_left = best_count.min(self.max_columns);
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
    fn height(&self, ui: &Ui) -> f32 {
        ui.fonts().row_height(FontSize::Normal.into()) * (self.rows as f32 + 1.0)
            + ui.fonts().row_height(FontSize::Section.into())
            + SESSION_REGION_BORDER
            + SESSION_SEPARATOR_SIZE * 2.0
    }

    fn paint(&self, ui: &Ui, rect: Rect, layout_metrics: &SolveLayoutMetrics) {
        // Draw session background
        let shaded_area = rect.shrink2(Vec2::new(REGION_PADDING, 0.0));
        ui.painter()
            .rect_filled(shaded_area, 0.0, Theme::Background);

        // Draw session name
        let content_area = shaded_area.shrink2(Vec2::new(SESSION_REGION_BORDER, 0.0));
        let name = match &self.name {
            Some(name) => &name,
            None => "Session",
        };
        ui.painter().text(
            content_area.left_top(),
            Align2::LEFT_TOP,
            format!("{} - {}", name, date_string(&self.last_solve.created)),
            FontSize::Section.into(),
            Theme::Blue.into(),
        );

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
        let col_width = layout_metrics.total_width + SESSION_SEPARATOR_SIZE;
        let row_height = ui.fonts().row_height(FontSize::Normal.into());
        let mut i = 0;
        for col in 0..self.columns {
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

                // Draw solve time
                ui.painter().galley(
                    Pos2::new(
                        content_area.left()
                            + col as f32 * col_width
                            + layout_metrics.solve_number_width
                            + layout_metrics.solve_time_width
                            - galley.size.x,
                        y + row as f32 * row_height,
                    ),
                    galley,
                    match time {
                        Some(_) => Theme::Content.into(),
                        None => Theme::Red.into(),
                    },
                );

                i += 1;
                if i >= self.solves.len() {
                    break;
                }
            }

            // Draw column separator
            let x = content_area.left()
                + col as f32 * col_width
                + layout_metrics.total_width
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
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Content.into());
            x += width + SESSION_BEST_PADDING;
        }

        // Draw best average of 5
        if let Some(best_ao5) = &self.best_ao5 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 5: ".into());
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Disabled.into());
            x += width;

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao5.time));
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Content.into());
            x += width + SESSION_BEST_PADDING;
        }

        // Draw best average of 12
        if let Some(best_ao12) = &self.best_ao12 {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Best avg of 12: ".into());
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Disabled.into());
            x += width;

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(best_ao12.time));
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Content.into());
            x += width + SESSION_BEST_PADDING;
        }

        // Draw session average
        if let Some(average) = &self.average {
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Session avg: ".into());
            let width = galley.size.x;
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Disabled.into());
            x += width;

            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), solve_time_string(*average));
            ui.painter()
                .galley(Pos2::new(x, y), galley, Theme::Content.into());
        }
    }
}

impl HistoryWidget {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            total_height: 0.0,
            cached_update_id: None,
            cached_best_columns: 0,
            cached_solve_columns: 0,
        }
    }

    fn generate_regions(&mut self, ui: &Ui, history: &mut History) {
        let columns = self.cached_solve_columns;

        let mut all_time_best_solve: Option<BestSolve> = None;
        let mut all_time_best_ao5: Option<Average> = None;
        let mut all_time_best_ao12: Option<Average> = None;

        // Go through sessions, gather data about them, and create regions for them
        let mut session_regions = Vec::new();
        for session in history.sessions().values() {
            let solves: Vec<Solve> = session.sorted_solves(history);
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
            let rows = (solves.len() + columns - 1) / columns;

            // Add the session to the region list
            session_regions.push(SessionRegion {
                session_id: session.id.clone(),
                name: session.name.clone(),
                solves,
                last_solve,
                rows,
                columns,
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

        // Add an all-time best region at the top
        regions.insert(
            0,
            Box::new(AllTimeBestRegion {
                best_solve: all_time_best_solve,
                best_ao5: all_time_best_ao5,
                best_ao12: all_time_best_ao12,
                max_columns: self.cached_best_columns,
            }),
        );

        // Lay out regions
        let mut y = 0.0;
        self.regions.clear();
        for region in regions {
            let height = region.height(ui);
            self.regions.push(HistoryRegionLayout { region, y, height });
            y += height + REGION_PADDING;
        }
        self.total_height = y;
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            let number_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "9999.".into());
            let solve_time_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "99:59.99".into());
            let best_time_galley = ui
                .fonts()
                .layout_single_line(FontSize::BestTime.into(), "99:59.99".into());
            let solve_layout_metrics = SolveLayoutMetrics {
                solve_number_width: number_galley.size.x,
                solve_time_width: solve_time_galley.size.x,
                total_width: number_galley.size.x + solve_time_galley.size.x,
                best_solve_width: best_time_galley.size.x,
            };

            let max_session_width =
                ui.max_rect().width() - REGION_PADDING * 2.0 - SESSION_REGION_BORDER * 2.0;
            let solve_columns = 1.max(
                (max_session_width / (solve_layout_metrics.total_width + SESSION_SEPARATOR_SIZE))
                    as usize,
            );
            let best_columns = 1.max(
                (ui.max_rect().width()
                    / (solve_layout_metrics.best_solve_width + BEST_TIME_COL_PADDING))
                    as usize,
            );

            if self.cached_update_id != Some(history.update_id())
                || self.cached_solve_columns != solve_columns
                || self.cached_best_columns != best_columns
            {
                self.cached_update_id = Some(history.update_id());
                self.cached_solve_columns = solve_columns;
                self.cached_best_columns = best_columns;
                self.generate_regions(ui, history);
            }

            ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
            ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
            ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
            ScrollArea::from_max_height(self.total_height)
                .id_source("history")
                .show(ui, |ui| {
                    let (rect, _) = ui.allocate_at_least(
                        Vec2::new(ui.max_rect().width(), self.total_height),
                        Sense::hover(),
                    );
                    for region in &self.regions {
                        region.region.paint(
                            ui,
                            Rect::from_min_size(
                                Pos2::new(rect.left(), rect.top() + region.y),
                                Vec2::new(rect.width(), region.height),
                            ),
                            &solve_layout_metrics,
                        );
                    }
                });
        });
    }
}
