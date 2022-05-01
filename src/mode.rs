use crate::theme::Theme;
use crate::widgets::CustomWidgets;
use egui::{CtxRef, Label, Sense, Ui, Window};
use tpscube_core::SolveType;

pub struct SolveTypeSelectWindow {
    solve_type: SolveType,
}

impl SolveTypeSelectWindow {
    pub fn new(solve_type: SolveType) -> Self {
        Self { solve_type }
    }

    fn option(
        &self,
        ui: &mut Ui,
        selected: &mut Option<SolveType>,
        solve_type: SolveType,
        name: &str,
    ) {
        let mut label = Label::new(name).sense(Sense::click());
        if self.solve_type == solve_type {
            label = label.text_color(Theme::Green);
        }
        if ui.add(label).clicked() {
            *selected = Some(solve_type);
        }
    }

    pub fn update(&self, ctxt: &CtxRef, open: &mut bool, selected: &mut Option<SolveType>) {
        Window::new("Select Puzzle")
            .collapsible(false)
            .resizable(false)
            .scroll(true)
            .open(open)
            .show(ctxt, |ui| {
                ui.vertical(|ui| {
                    ui.section("Standard Cubes");
                    self.option(ui, selected, SolveType::Standard2x2x2, "2x2x2");
                    self.option(ui, selected, SolveType::Standard3x3x3, "3x3x3");
                    self.option(ui, selected, SolveType::Standard4x4x4, "4x4x4");
                    self.option(ui, selected, SolveType::OneHanded3x3x3, "3x3x3 One Handed");

                    ui.section("Blindfolded");
                    self.option(ui, selected, SolveType::Blind3x3x3, "3x3x3 Blindfolded");
                    self.option(ui, selected, SolveType::Blind4x4x4, "4x4x4 Blindfolded");
                });
            });
    }
}
