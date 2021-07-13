mod data;
mod plot;

use crate::font::FontSize;
use crate::style::content_visuals;
use crate::theme::Theme;
use data::{GraphData, Phase, Statistic};
use egui::{CentralPanel, CtxRef, Pos2, Rect, Vec2};
use plot::Plot;
use tpscube_core::History;

const GRAPH_PADDING: f32 = 16.0;

pub struct GraphWidget {
    statistic: Statistic,
    phase: Phase,
    average_size: usize,
    plot: Option<Plot>,
    update_id: Option<u64>,
}

impl GraphWidget {
    pub fn new() -> Self {
        Self {
            statistic: Statistic::Time,
            phase: Phase::EntireSolve,
            average_size: 5,
            plot: None,
            update_id: None,
        }
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            if self.update_id != Some(history.update_id()) {
                // If history has been updated, regenerate plot
                self.plot = None;
            }

            // Get plot data
            let plot = if let Some(plot) = &self.plot {
                plot
            } else {
                // No plot data cached, regenerate now
                self.plot = Some(
                    GraphData::new()
                        .statistic(self.statistic)
                        .phase(self.phase)
                        .average_size(self.average_size)
                        .build(history),
                );
                self.update_id = Some(history.update_id());
                self.plot.as_ref().unwrap()
            };

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
            plot.draw(ui, rect);
        });
    }
}
