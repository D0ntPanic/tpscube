use crate::theme::Theme;
use crate::widgets::{color_for_recognition_step_index, color_for_step_index};
use egui::{CtxRef, Pos2, Rect, Response, Stroke, Ui, Vec2};
use tpscube_core::{AnalysisStepSummary, Solve};

const SOLVE_BAR_HEIGHT: f32 = 4.0;
const SOLVE_STEP_SEPARATOR_HEIGHT: f32 = 10.0;
const CURSOR_HEIGHT: f32 = 16.0;

pub struct SolveBar<'a> {
    solve: &'a Solve,
    summary: &'a [AnalysisStepSummary],
    max_time: u32,
    cursor: Option<f32>,
}

impl<'a> SolveBar<'a> {
    pub fn new(
        solve: &'a Solve,
        summary: &'a [AnalysisStepSummary],
        max_time: u32,
        cursor: Option<f32>,
    ) -> Self {
        Self {
            solve,
            summary,
            max_time,
            cursor,
        }
    }

    fn draw_time_only(&self, ui: &mut Ui, rect: Rect) {
        let total = self.max_time as f32 / 1000.0;
        if let Some(time) = self.solve.final_time() {
            ui.painter().rect_filled(
                Rect::from_min_size(
                    Pos2::new(rect.left(), rect.center().y - SOLVE_BAR_HEIGHT / 2.0),
                    Vec2::new(
                        rect.width() * time as f32 / 1000.0 / total,
                        SOLVE_BAR_HEIGHT,
                    ),
                ),
                0.0,
                Theme::Disabled,
            );

            let x = rect.width() * time as f32 / 1000.0 / total + rect.left();
            ui.painter().line_segment(
                [
                    Pos2::new(x, rect.center().y - SOLVE_STEP_SEPARATOR_HEIGHT / 2.0),
                    Pos2::new(x, rect.center().y + SOLVE_STEP_SEPARATOR_HEIGHT / 2.0),
                ],
                Stroke {
                    width: 2.0,
                    color: Theme::Content.into(),
                },
            );
        }
    }

    fn draw_analysis(&self, ui: &mut Ui, rect: Rect) {
        let mut start = 0.0;
        let mut last_index = 0;
        let total = self.max_time as f32 / 1000.0;
        let mut separators = Vec::new();
        for step in self.summary {
            if step.major_step_index != last_index {
                // Queue up stage separators and draw them after the bars.
                // This prevents artifacts from drawing over the lines.
                separators.push((
                    [
                        Pos2::new(
                            rect.width() * start / total + rect.left(),
                            rect.center().y - SOLVE_STEP_SEPARATOR_HEIGHT / 2.0,
                        ),
                        Pos2::new(
                            rect.width() * start / total + rect.left(),
                            rect.center().y + SOLVE_STEP_SEPARATOR_HEIGHT / 2.0,
                        ),
                    ],
                    Stroke {
                        width: 1.0,
                        color: color_for_step_index(last_index),
                    },
                ));
            }

            // Draw recognition portion
            ui.painter().rect_filled(
                Rect::from_min_size(
                    Pos2::new(
                        rect.width() * start / total + rect.left(),
                        rect.center().y - SOLVE_BAR_HEIGHT / 2.0,
                    ),
                    Vec2::new(
                        rect.width() * step.recognition_time as f32 / 1000.0 / total,
                        SOLVE_BAR_HEIGHT,
                    ),
                ),
                0.0,
                color_for_recognition_step_index(step.major_step_index),
            );
            start += step.recognition_time as f32 / 1000.0;

            // Draw execution portion
            ui.painter().rect_filled(
                Rect::from_min_size(
                    Pos2::new(
                        rect.width() * start / total + rect.left(),
                        rect.center().y - SOLVE_BAR_HEIGHT / 2.0,
                    ),
                    Vec2::new(
                        rect.width() * step.execution_time as f32 / 1000.0 / total,
                        SOLVE_BAR_HEIGHT,
                    ),
                ),
                0.0,
                color_for_step_index(step.major_step_index),
            );
            start += step.execution_time as f32 / 1000.0;

            last_index = step.major_step_index;
        }

        // Add separator for final stage at end
        separators.push((
            [
                Pos2::new(
                    rect.width() * start / total + rect.left(),
                    rect.center().y - SOLVE_STEP_SEPARATOR_HEIGHT / 2.0,
                ),
                Pos2::new(
                    rect.width() * start / total + rect.left(),
                    rect.center().y + SOLVE_STEP_SEPARATOR_HEIGHT / 2.0,
                ),
            ],
            Stroke {
                width: 1.0,
                color: color_for_step_index(last_index),
            },
        ));

        // Draw solve stage separators
        for line in separators {
            ui.painter().line_segment(line.0, line.1);
        }

        if let Some(cursor) = self.cursor {
            // Draw indicator of cursor position
            ui.painter().line_segment(
                [
                    Pos2::new(
                        rect.width() * cursor / total + rect.left(),
                        rect.center().y - CURSOR_HEIGHT / 2.0,
                    ),
                    Pos2::new(
                        rect.width() * cursor / total + rect.left(),
                        rect.center().y + CURSOR_HEIGHT / 2.0,
                    ),
                ],
                Stroke {
                    width: 2.0,
                    color: Theme::Content.into(),
                },
            );
        }
    }

    fn draw(&self, ui: &mut Ui, rect: Rect) {
        if self.summary.len() == 0 {
            self.draw_time_only(ui, rect);
        } else {
            self.draw_analysis(ui, rect);
        }
    }

    pub fn interactive(
        &self,
        ctxt: &CtxRef,
        ui: &mut Ui,
        rect: Rect,
        response: Response,
    ) -> Option<f32> {
        self.draw(ui, rect);

        // Check for click on solve bar and return that location
        if response.clicked() || response.dragged() {
            if let Some(pos) = ctxt.input().pointer.interact_pos() {
                let frac = (pos.x - rect.left()) / rect.width();
                let frac = frac.min(1.0).max(0.0);
                return Some(self.max_time as f32 * frac / 1000.0);
            }
        }
        None
    }

    pub fn noninteractive(&self, ui: &mut Ui, rect: Rect) {
        self.draw(ui, rect);
    }
}
