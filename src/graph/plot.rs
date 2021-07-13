use crate::font::FontSize;
use crate::theme::Theme;
use crate::widgets::short_day_string;
use chrono::{DateTime, Local};
use egui::{
    epaint::{Mesh, TextureId, Vertex, WHITE_UV},
    Color32, Pos2, Rect, Shape, Stroke, Ui, Vec2,
};

const AXIS_PADDING: f32 = 16.0;
const AXIS_LABEL_PADDING_FACTOR: f32 = 2.0;
const AXIS_LABEL_PADDING_WIDTH: f32 = 24.0;
const AXIS_TICK_SIZE: f32 = 4.0;

const MAX_POINTS: usize = 1000;

#[cfg(target_arch = "wasm32")]
const MIN_ALPHA: u8 = 128;
#[cfg(target_arch = "wasm32")]
const MAX_ALPHA: u8 = 224;

#[cfg(not(target_arch = "wasm32"))]
const MIN_ALPHA: u8 = 32;
#[cfg(not(target_arch = "wasm32"))]
const MAX_ALPHA: u8 = 128;

pub enum Plot {
    Single(SinglePlot),
}

pub struct SinglePlot {
    title: String,
    y_axis: YAxis,
    points: Vec<(DateTime<Local>, f32)>,
    color: Color32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum YAxis {
    Time,
    MoveCount,
}

impl Plot {
    pub fn title(&self) -> &str {
        match self {
            Plot::Single(plot) => plot.title(),
        }
    }

    pub fn draw(&self, ui: &mut Ui, rect: Rect) {
        match self {
            Plot::Single(plot) => plot.draw(ui, rect),
        }
    }
}

impl SinglePlot {
    pub fn new(title: String, y_axis: YAxis, color: Color32) -> Self {
        Self {
            title,
            y_axis,
            points: Vec::new(),
            color,
        }
    }

    /// Assumes the points are added in chronological order
    pub fn push(&mut self, time: DateTime<Local>, value: f32) {
        self.points.push((time, value));
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn finalize(&mut self) {
        // Ensure the number of points is reasonable
        while self.points.len() > MAX_POINTS {
            let mut folded = Vec::new();
            for pair in self.points.as_slice().windows(2) {
                folded.push((pair[1].0, (pair[0].1 + pair[1].1) / 2.0));
            }
            self.points = folded;
        }
    }

    pub fn draw(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        let max_value = self.points.iter().fold(0.0, |max, value| value.1.max(max));

        // Subtract out x axis labels from plot area
        let axis_label_height = ui.fonts().row_height(FontSize::Normal.into());
        let plot_area = Rect::from_min_size(
            rect.left_top(),
            Vec2::new(
                rect.width(),
                rect.height() - axis_label_height - AXIS_PADDING,
            ),
        );

        // Pick labels for y axis based on what will fit and a natural step for the
        // type of data.
        let mut max_labels =
            (plot_area.height() / (axis_label_height * AXIS_LABEL_PADDING_FACTOR)) as usize;
        if max_labels < 4 {
            max_labels = 4;
        }
        let min_step = max_value as usize / max_labels;
        let max_step = max_value as usize / 4;
        let step = if max_step < 1 {
            1
        } else if min_step < 2 && max_step > 2 {
            2
        } else if min_step < 5 && max_step > 5 {
            5
        } else if min_step < 10 && max_step > 10 {
            10
        } else if self.y_axis == YAxis::Time && min_step < 15 && max_step > 15 {
            15
        } else if min_step < 20 && max_step > 20 {
            20
        } else if self.y_axis == YAxis::Time && min_step < 30 && max_step > 30 {
            30
        } else if self.y_axis != YAxis::Time && min_step < 50 && max_step > 50 {
            50
        } else if self.y_axis == YAxis::Time && min_step < 60 && max_step > 60 {
            60
        } else if self.y_axis != YAxis::Time && min_step < 100 && max_step > 100 {
            100
        } else {
            max_step
        };

        // Lay out y axis labels
        let mut y_axis_labels = Vec::new();
        let mut value = step;
        let mut max_width = 0.0;
        while (value as f32) < max_value {
            let galley = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                match self.y_axis {
                    YAxis::Time => {
                        if value >= 60 {
                            format!("{}:{:02}", value / 60, value % 60)
                        } else {
                            format!("{}", value)
                        }
                    }
                    _ => format!("{}", value),
                },
            );
            max_width = galley.size.x.max(max_width);
            y_axis_labels.push((
                galley,
                plot_area.top() + plot_area.height() * (1.0 - value as f32 / max_value),
            ));

            value += step;
        }

        // Subtract out y axis labels from plot area
        let plot_area = Rect::from_min_size(
            Pos2::new(plot_area.left() + max_width + AXIS_PADDING, plot_area.top()),
            Vec2::new(
                plot_area.width() - max_width - AXIS_PADDING,
                plot_area.height(),
            ),
        );

        // Draw x axis labels
        let mut last_day = None;
        let mut min_x = plot_area.left();
        let max_x = plot_area.right();
        for (idx, value) in self.points.iter().enumerate() {
            let day = short_day_string(&value.0);
            if Some(day.clone()) != last_day {
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), day.clone());
                let width = galley.size.x;

                let x =
                    plot_area.left() + plot_area.width() * idx as f32 / self.points.len() as f32;
                let left = x - width / 2.0;
                let right = x + width / 2.0;

                if left > min_x && right < max_x {
                    painter.galley(
                        Pos2::new(left, plot_area.bottom() + AXIS_PADDING),
                        galley,
                        Theme::Content.into(),
                    );

                    painter.line_segment(
                        [
                            Pos2::new(x, plot_area.bottom()),
                            Pos2::new(x, plot_area.bottom() + AXIS_TICK_SIZE),
                        ],
                        Stroke {
                            width: 2.0,
                            color: Theme::Content.into(),
                        },
                    );

                    min_x = right + AXIS_LABEL_PADDING_WIDTH;
                }

                last_day = Some(day);
            }
        }

        // Compute locations of each plot point
        let mut points = Vec::new();
        for (idx, value) in self.points.iter().enumerate() {
            points.push((
                Pos2::new(
                    plot_area.left() + plot_area.width() * idx as f32 / self.points.len() as f32,
                    plot_area.top() + plot_area.height() * (1.0 - value.1 / max_value),
                ),
                value.1 / max_value,
            ));
        }

        // Draw y axis labels and grid lines
        for label in y_axis_labels {
            painter.galley(
                Pos2::new(
                    plot_area.left() - AXIS_PADDING - label.0.size.x,
                    label.1 - label.0.size.y / 2.0,
                ),
                label.0,
                Theme::Content.into(),
            );

            painter.line_segment(
                [
                    Pos2::new(plot_area.left() - AXIS_TICK_SIZE, label.1),
                    Pos2::new(plot_area.left(), label.1),
                ],
                Stroke {
                    width: 2.0,
                    color: Theme::Content.into(),
                },
            );

            painter.line_segment(
                [
                    Pos2::new(plot_area.left(), label.1),
                    Pos2::new(plot_area.right(), label.1),
                ],
                Stroke {
                    width: 1.0,
                    color: Theme::BackgroundHighlight.into(),
                },
            );
        }

        // Draw vertical line at end of plot
        if let Some(last) = points.last() {
            painter.line_segment(
                [Pos2::new(last.0.x, plot_area.bottom()), last.0],
                Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                },
            );
        }

        // Draw plot lines
        painter.line_segment(
            [
                Pos2::new(plot_area.left(), plot_area.top() - 1.0),
                plot_area.left_bottom(),
            ],
            Stroke {
                width: 2.0,
                color: Theme::Content.into(),
            },
        );
        painter.line_segment(
            [
                plot_area.right_bottom(),
                Pos2::new(plot_area.left() - 1.0, plot_area.bottom()),
            ],
            Stroke {
                width: 2.0,
                color: Theme::Content.into(),
            },
        );

        // Create mesh for plot fill
        let mut verts = Vec::new();
        let mut idx = Vec::new();
        for pt in &points {
            let start = verts.len() as u32;
            verts.push(Vertex {
                pos: Pos2::new(pt.0.x, plot_area.bottom()),
                uv: WHITE_UV,
                color: Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    MIN_ALPHA,
                ),
            });
            verts.push(Vertex {
                pos: Pos2::new(pt.0.x, pt.0.y),
                uv: WHITE_UV,
                color: Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    MIN_ALPHA + ((MAX_ALPHA - MIN_ALPHA) as f32 * pt.1) as u8,
                ),
            });
            if start >= 2 {
                idx.push(start - 2);
                idx.push(start - 1);
                idx.push(start + 1);
                idx.push(start - 2);
                idx.push(start + 1);
                idx.push(start);
            }
        }

        // Draw background fill
        painter.add(Shape::mesh(Mesh {
            indices: idx,
            vertices: verts,
            texture_id: TextureId::Egui,
        }));

        // Draw line in graph
        for segment in points.as_slice().windows(2) {
            painter.line_segment(
                [segment[0].0, segment[1].0],
                Stroke {
                    width: 2.0,
                    color: self.color,
                },
            )
        }
    }
}

impl From<SinglePlot> for Plot {
    fn from(plot: SinglePlot) -> Self {
        Self::Single(plot)
    }
}
