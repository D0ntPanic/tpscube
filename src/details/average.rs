use crate::app::SolveDetails;
use crate::details::bar::SolveBar;
use crate::font::FontSize;
use crate::style::dialog_visuals;
use crate::theme::Theme;
use crate::widgets::{date_string, solve_time_string};
use egui::{CtxRef, CursorIcon, Label, Pos2, Rect, Sense, Vec2, Window};
use std::cmp::Ordering;
use tpscube_core::{
    Analysis, AnalysisStepSummary, AnalysisSummary, Cube, Cube3x3x3, CubeWithSolution, ListAverage,
    Solve,
};

const TARGET_MAX_WIDTH: f32 = 300.0;
const BAR_PADDING: f32 = 16.0;

pub struct AverageDetailsWindow {
    average: Option<u32>,
    solves: Vec<SolveWithAnalysis>,
}

struct SolveWithAnalysis {
    solve: Solve,
    summary: Vec<AnalysisStepSummary>,
    included_in_average: bool,
}

impl AverageDetailsWindow {
    pub fn new(solves: Vec<Solve>) -> Self {
        let average = solves.as_slice().average();

        // Analyze solves
        let mut solves_with_analysis = Vec::new();
        for solve in solves {
            let mut unsolved_state = Cube3x3x3::new();
            unsolved_state.do_moves(&solve.scramble);
            let analysis = if let Some(solution) = &solve.moves {
                Analysis::analyze(&CubeWithSolution {
                    initial_state: unsolved_state.clone(),
                    solution: solution.clone(),
                })
            } else {
                Analysis::default()
            };
            let summary = analysis.detailed_step_summary();
            solves_with_analysis.push(SolveWithAnalysis {
                solve,
                summary,
                included_in_average: true,
            });
        }

        // Sort solves by time, ensuring that DNF is considered the
        // maximum time.
        let mut sorted: Vec<usize> = (0..solves_with_analysis.len()).collect();
        sorted.sort_unstable_by(|a, b| {
            let a = solves_with_analysis[*a].solve.final_time();
            let b = solves_with_analysis[*b].solve.final_time();
            if a.is_none() && b.is_none() {
                Ordering::Equal
            } else if a.is_none() {
                Ordering::Greater
            } else if b.is_none() {
                Ordering::Less
            } else {
                let a = a.unwrap();
                let b = b.unwrap();
                a.cmp(&b)
            }
        });

        // Remove the best and worst time(s) as appropriate for the size of the set.
        // If there are less than 5 values, use an arithmetic mean and do not
        // eliminate any values.
        let to_remove = if sorted.len() >= 5 {
            (sorted.len() + 39) / 40
        } else {
            0
        };
        let len = sorted.len();
        for i in 0..to_remove {
            solves_with_analysis[sorted[i]].included_in_average = false;
            solves_with_analysis[sorted[len - (i + 1)]].included_in_average = false;
        }

        Self {
            average,
            solves: solves_with_analysis,
        }
    }

    pub fn update(&mut self, ctxt: &CtxRef, open: &mut bool, details: &mut Option<SolveDetails>) {
        let full_rect = ctxt.available_rect();
        let target_width = (full_rect.width() - 64.0).min(TARGET_MAX_WIDTH);

        // Find the longest time in the set of solves. This will be used to layout
        // the time display and the bar graphs.
        let max_time = self.solves.iter().fold(0, |max, solve| {
            if let Some(time) = solve.solve.final_time() {
                max.max(time)
            } else {
                max
            }
        });

        ctxt.set_visuals(dialog_visuals());

        Window::new(format!(
            "Average of {} - {}",
            self.solves.len(),
            date_string(&self.solves.last().unwrap().solve.created)
        ))
        .collapsible(false)
        .resizable(false)
        .open(open)
        .show(ctxt, |ui| {
            ui.vertical_centered(|ui| {
                // Display average time at top
                if let Some(average) = self.average {
                    ui.add(
                        Label::new(solve_time_string(average))
                            .text_style(FontSize::BestTime.into())
                            .text_color(Theme::Green),
                    );
                    ui.add_space(8.0);
                }

                // Get maximum width of solve number
                let galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    format!("{}.   ", self.solves.len() + 1),
                );
                let solve_num_width = galley.size.x;

                // Get maximum width of solve time
                let galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    format!("({})", solve_time_string(max_time)),
                );
                let solve_time_width = galley.size.x;

                // Compute column widths
                let solve_bar_width =
                    target_width - solve_num_width - solve_time_width - BAR_PADDING;
                let solve_time_offset = solve_num_width + solve_time_width;
                let solve_bar_offset = target_width - solve_bar_width;

                // Draw each solve
                for (i, solve) in self.solves.iter().enumerate() {
                    let (id, rect) = ui.allocate_space(Vec2::new(
                        target_width,
                        ui.fonts().row_height(FontSize::Normal.into()),
                    ));
                    let response = ui.interact(rect, id, Sense::click());

                    // Draw solve number
                    let galley = ui
                        .fonts()
                        .layout_single_line(FontSize::Normal.into(), format!("{}.", i + 1));
                    ui.painter()
                        .galley(rect.left_top(), galley, Theme::Disabled.into());

                    // Get solve time string and color
                    let (mut time, mut color) = if let Some(time) = solve.solve.final_time() {
                        (solve_time_string(time), Theme::Content)
                    } else {
                        ("DNF".into(), Theme::Red)
                    };

                    // If solve is not included in average, place it in parenthesis and render
                    // it in gray.
                    if !solve.included_in_average {
                        time = format!("({})", time);
                        color = Theme::Disabled;
                    }

                    // Draw solve time
                    let galley = ui.fonts().layout_single_line(FontSize::Normal.into(), time);
                    ui.painter().galley(
                        Pos2::new(rect.left() + solve_time_offset - galley.size.x, rect.top()),
                        galley,
                        if response.hovered() {
                            Theme::Blue.into()
                        } else {
                            color.into()
                        },
                    );

                    // Show solve bar to the right of the time
                    let bar = SolveBar::new(&solve.solve, &solve.summary, max_time, None);
                    bar.noninteractive(
                        ui,
                        Rect::from_min_size(
                            Pos2::new(rect.left() + solve_bar_offset, rect.top()),
                            Vec2::new(solve_bar_width, rect.height()),
                        ),
                    );

                    // Check for clicks on the solve
                    if response.on_hover_cursor(CursorIcon::PointingHand).clicked() {
                        *details = Some(SolveDetails::IndividualSolve(solve.solve.clone()));
                    }
                }
            });
        });
    }
}
