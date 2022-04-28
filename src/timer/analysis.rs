use crate::font::FontSize;
use crate::theme::Theme;
use crate::widgets::{color_for_recognition_step_index, color_for_step_index, solve_time_string};
use egui::{Pos2, Rect, Ui, Vec2};
use tpscube_core::{AnalysisStepSummary, AnalysisSubstepTime};

const MIN_GRAPH_WIDTH: f32 = 200.0;

pub struct TimerPostAnalysis {
    steps: Vec<AnalysisStepSummary>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TimerPostAnalysisStyle {
    Full,
    ShortName,
    TimeAndMoves,
    TimeOnly,
}

impl TimerPostAnalysisStyle {
    fn has_bars(&self) -> bool {
        match self {
            TimerPostAnalysisStyle::Full | TimerPostAnalysisStyle::ShortName => true,
            TimerPostAnalysisStyle::TimeAndMoves | TimerPostAnalysisStyle::TimeOnly => false,
        }
    }

    fn has_move_count(&self) -> bool {
        match self {
            TimerPostAnalysisStyle::Full
            | TimerPostAnalysisStyle::ShortName
            | TimerPostAnalysisStyle::TimeAndMoves => true,
            TimerPostAnalysisStyle::TimeOnly => false,
        }
    }
}

impl TimerPostAnalysis {
    pub fn new(steps: Vec<AnalysisStepSummary>) -> Self {
        Self { steps }
    }

    pub fn present(&self) -> bool {
        self.steps.len() != 0
    }

    pub fn height(&self, ui: &Ui) -> f32 {
        ui.fonts().row_height(FontSize::Normal.into()) * (self.steps.len() + 2) as f32
    }

    pub fn move_count(&self) -> usize {
        self.steps
            .iter()
            .fold(0, |total, step| total + step.move_count)
    }

    pub fn render(&self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.left();

        // Measure name column
        let mut name_galleys = Vec::new();
        let mut short_name_galleys = Vec::new();
        let mut algorithm_galleys = Vec::new();
        let mut name_col_width: f32 = 0.0;
        let mut short_name_col_width: f32 = 0.0;
        for step in &self.steps {
            let name_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), step.name.clone());
            let short_name_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), step.short_name.clone());
            let algorithm_galley = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                if let Some(algorithm) = &step.algorithm {
                    format!(" {}", algorithm)
                } else {
                    "".into()
                },
            );
            name_col_width = name_col_width.max(name_galley.size.x + algorithm_galley.size.x);
            short_name_col_width = short_name_col_width.max(short_name_galley.size.x);

            name_galleys.push(name_galley);
            short_name_galleys.push(short_name_galley);
            algorithm_galleys.push(algorithm_galley);
        }

        // Compute graph width
        let mut total_graph_width = rect.width() - name_col_width - 16.0;
        let mut style = TimerPostAnalysisStyle::Full;
        if total_graph_width < MIN_GRAPH_WIDTH {
            total_graph_width = rect.width() - short_name_col_width - 16.0;
            name_col_width = short_name_col_width;
            if total_graph_width < MIN_GRAPH_WIDTH {
                style = TimerPostAnalysisStyle::TimeAndMoves;
            } else {
                style = TimerPostAnalysisStyle::ShortName;
            }
        }

        // Render step names
        let mut y = rect.top();
        for _ in &self.steps {
            if style == TimerPostAnalysisStyle::Full {
                let name_galley = name_galleys.remove(0);
                let name_width = name_galley.size.x;
                ui.painter()
                    .galley(Pos2::new(x, y), name_galley, Theme::Content.into());
                ui.painter().galley(
                    Pos2::new(x + name_width, y),
                    algorithm_galleys.remove(0),
                    Theme::Disabled.into(),
                );
            } else {
                ui.painter().galley(
                    Pos2::new(x, y),
                    short_name_galleys.remove(0),
                    Theme::Content.into(),
                );
            }
            y += ui.fonts().row_height(FontSize::Normal.into());
        }

        // Layout time and move strings for each step and determine size
        // of time and move columns.
        x += name_col_width + 16.0;
        let mut time_galleys = Vec::new();
        let mut move_galleys = Vec::new();
        let mut time_width: f32 = 0.0;
        let mut move_width: f32 = 0.0;
        let mut max_time = 0;
        let mut recognition_total_time = 0;
        let mut execution_total_time = 0;
        for step in &self.steps {
            let time = step.recognition_time + step.execution_time;
            recognition_total_time += step.recognition_time;
            execution_total_time += step.execution_time;
            max_time = max_time.max(time);
            let galley = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                if style.has_bars() {
                    format!("  {}", solve_time_string(time))
                } else {
                    format!("{}", solve_time_string(time))
                },
            );
            time_width = time_width.max(galley.size.x);
            time_galleys.push(galley);

            let galley = ui.fonts().layout_single_line(
                FontSize::Small.into(),
                format!("  ({} moves)", step.move_count),
            );
            move_width = move_width.max(galley.size.x);
            move_galleys.push(galley);
        }

        // If region is too small to show move counts, switch to time only
        if style == TimerPostAnalysisStyle::TimeAndMoves
            && (time_width + move_width) > (rect.right() - x)
        {
            style = TimerPostAnalysisStyle::TimeOnly;
        }

        // Render step time graph
        let bar_width = total_graph_width - (time_width + move_width);
        let mut y = rect.top();
        for step in &self.steps {
            let mut offset = 0.0;

            if style.has_bars() {
                // Render bars with darker color for the recognition parts of the step
                for substep in &step.substeps {
                    let (time, color) = match substep {
                        AnalysisSubstepTime::Recognition(time) => (
                            time,
                            color_for_recognition_step_index(step.major_step_index),
                        ),
                        AnalysisSubstepTime::Execution(time) => {
                            (time, color_for_step_index(step.major_step_index))
                        }
                    };
                    let time_bar_frac = *time as f32 / max_time as f32;
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(x + offset, y + 3.0),
                            Vec2::new(
                                time_bar_frac * bar_width,
                                ui.fonts().row_height(FontSize::Normal.into()) - 4.0,
                            ),
                        ),
                        0.0,
                        color,
                    );
                    offset += time_bar_frac * bar_width;
                }
            }

            // Draw time to the right of bar
            let galley = time_galleys.remove(0);
            let width = galley.size.x;
            ui.painter().galley(
                Pos2::new(x + offset, y),
                galley,
                color_for_step_index(step.major_step_index),
            );
            offset += width;

            if style.has_move_count() {
                // Draw move count to the right of time
                ui.painter().galley(
                    Pos2::new(
                        x + offset,
                        y + (ui.fonts().row_height(FontSize::Normal.into())
                            - ui.fonts().row_height(FontSize::Small.into())),
                    ),
                    move_galleys.remove(0),
                    color_for_step_index(step.major_step_index),
                );
            }

            y += ui.fonts().row_height(FontSize::Normal.into());
        }

        let galley = ui.fonts().layout_single_line(
            FontSize::Normal.into(),
            if style.has_bars() {
                format!(
                    "Recognize: {}   Execute: {}",
                    solve_time_string(recognition_total_time),
                    solve_time_string(execution_total_time)
                )
            } else if style.has_move_count() {
                format!(
                    "Rec: {}  Exe: {}",
                    solve_time_string(recognition_total_time),
                    solve_time_string(execution_total_time)
                )
            } else {
                format!(
                    "R: {} E: {}",
                    solve_time_string(recognition_total_time),
                    solve_time_string(execution_total_time)
                )
            },
        );

        y += 0.5 * ui.fonts().row_height(FontSize::Normal.into());

        x = (rect.width() - galley.size.x) / 2.0 + rect.left();
        ui.painter()
            .galley(Pos2::new(x, y), galley, Theme::Disabled.into());
    }
}
