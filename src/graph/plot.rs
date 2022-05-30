use crate::font::FontSize;
use crate::theme::Theme;
use crate::widgets::short_day_string;
use chrono::{DateTime, Local};
use egui::{
    epaint::{Mesh, TextureId, Vertex, WHITE_UV},
    Color32, CtxRef, Pos2, Rect, Response, Shape, Stroke, Ui, Vec2,
};

const AXIS_PADDING: f32 = 16.0;
const AXIS_LABEL_PADDING_FACTOR: f32 = 2.0;
const AXIS_LABEL_PADDING_WIDTH: f32 = 24.0;
const AXIS_TICK_SIZE: f32 = 4.0;

const MAX_POINTS: usize = 512;
const TARGET_MIN_POINTS: usize = 64;
const EXP_ZOOM_DIVISOR: f32 = 256.0;
const EPSILON: f32 = 0.0001;

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
    zoom: PlotZoom,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum YAxis {
    Time,
    MoveCount,
    TurnsPerSecond,
    SuccessRate,
}

struct PlotZoom {
    zoom: f32,
    start: f32,
}

impl Plot {
    pub fn title(&self) -> &str {
        match self {
            Plot::Single(plot) => plot.title(),
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            Plot::Single(plot) => plot.valid(),
        }
    }

    pub fn update(&mut self, ctxt: &CtxRef, ui: &mut Ui, rect: Rect, interact: Response) {
        match self {
            Plot::Single(plot) => plot.update(ctxt, ui, rect, interact),
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
            zoom: PlotZoom {
                zoom: 1.0,
                start: 0.0,
            },
        }
    }

    /// Assumes the points are added in chronological order
    pub fn push(&mut self, time: DateTime<Local>, value: f32) {
        self.points.push((time, value));
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn valid(&self) -> bool {
        self.points.len() >= 2
    }

    fn update_zoom(&mut self, ctxt: &CtxRef, ui: &Ui, x_delta: f32, y_delta: f32, rect: &Rect) {
        if x_delta.abs() < EPSILON && y_delta.abs() < EPSILON {
            // Don't do anything if no scrolling
            return;
        }

        // Compute where on a x axis the pointer is. This will be used to try and keep whatever
        // is under the cursor at the same position when zooming.
        let pointer_frac = if let Some(pos) = ui.input().pointer.interact_pos() {
            (pos.x - rect.left()) / rect.width()
        } else {
            0.5
        };

        if x_delta.abs() > y_delta.abs() {
            // Horizontal scroll, ignore vertical component in the case of trackpads
            let frac = x_delta * self.zoom.zoom / rect.width();
            self.zoom.start = (self.zoom.start - frac).max(0.0).min(1.0 - self.zoom.zoom);
        } else {
            // Vertical scroll for zoom, ignore horizontal component in the case of trackpads
            let min_zoom = (TARGET_MIN_POINTS as f32 / self.points.len() as f32).min(1.0);
            let new_zoom = 2.0f32
                .powf(self.zoom.zoom.log2() + y_delta / EXP_ZOOM_DIVISOR)
                .max(min_zoom)
                .min(1.0);
            let old_pointer_pos = self.zoom.start + pointer_frac * self.zoom.zoom;
            let new_start = old_pointer_pos - pointer_frac * new_zoom;
            self.zoom.zoom = new_zoom;
            self.zoom.start = new_start.max(0.0).min(1.0 - self.zoom.zoom);
        }

        // Repaint to update graph after scroll
        ctxt.request_repaint();
    }

    pub fn update(&mut self, ctxt: &CtxRef, ui: &mut Ui, rect: Rect, interact: Response) {
        let painter = ui.painter();
        let max_value = if self.y_axis == YAxis::SuccessRate {
            100.0
        } else {
            self.points.iter().fold(0.0, |max, value| value.1.max(max))
        };

        let points_to_show = (self.points.len() as f32 * self.zoom.zoom) as usize;

        // If there are too many points to show, combine points. Always divide the
        // points by a power of two for easy combining and minimal jitter.
        let mut combined_points = 1;
        while points_to_show / combined_points > MAX_POINTS {
            combined_points *= 2;
        }

        // Get index of first point in plot according to the current scroll. Always pick an
        // index that is a multiple of `combined_points` to avoid jitter.
        let first_point = (self.points.len() as f32 * self.zoom.start / combined_points as f32)
            as usize
            * combined_points;
        if first_point >= self.points.len() {
            return;
        }

        // Get number of points to plot based on zoom
        let points_to_show = points_to_show.min(self.points.len() - first_point);
        let end_point = first_point + points_to_show;

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
        while (value as f32) <= max_value {
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
                    YAxis::SuccessRate => {
                        format!("{}%", value)
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
        for (idx, value) in (&self.points[first_point..end_point])
            .chunks(combined_points)
            .enumerate()
        {
            let value = value.last().unwrap();
            let day = short_day_string(&value.0);
            if Some(day.clone()) != last_day {
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), day.clone());
                let width = galley.size.x;

                let x = plot_area.left()
                    + plot_area.width() * idx as f32 / (points_to_show / combined_points) as f32;
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
        for (idx, value) in (&self.points[first_point..end_point])
            .chunks(combined_points)
            .enumerate()
        {
            if value.len() < combined_points {
                // Don't show trailing data as it isn't averaged and often causes
                // jitter at the end of the graph.
                continue;
            }

            let sum = value.iter().fold(0.0, |sum, value| sum + value.1);
            let value = sum / value.len() as f32;
            points.push((
                Pos2::new(
                    plot_area.left()
                        + plot_area.width() * idx as f32
                            / (points_to_show / combined_points) as f32,
                    plot_area.top() + plot_area.height() * (1.0 - value / max_value),
                ),
                value / max_value,
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

        if interact.dragged() {
            self.update_zoom(
                ctxt,
                ui,
                ui.input().pointer.delta().x,
                ui.input().pointer.delta().y,
                &plot_area,
            );
        } else if ui.rect_contains_pointer(rect) {
            let scroll_delta = ctxt.input().scroll_delta;
            self.update_zoom(ctxt, ui, scroll_delta.x, scroll_delta.y, &plot_area);
        }
    }
}

impl From<SinglePlot> for Plot {
    fn from(plot: SinglePlot) -> Self {
        Self::Single(plot)
    }
}
