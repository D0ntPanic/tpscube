use crate::style::content_visuals;
use crate::theme::Theme;
use egui::{widgets::Label, CentralPanel, CtxRef};
use tpscube_core::History;

pub struct GraphWidget;

impl GraphWidget {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(Label::new("Coming soon").text_color(Theme::Disabled));
            });
        });
    }
}
