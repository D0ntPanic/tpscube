use crate::font::{FontSize, LabelFontSize};
use crate::theme::Theme;
use egui::{
    popup::popup_below_widget, widgets::Label, Color32, Layout, Response, Sense, Stroke, Ui,
};
use tpscube_core::{History, Penalty, Solve};

pub trait CustomWidgets {
    fn header_label(&mut self, text: &str, active: bool) -> Response;
    fn section(&mut self, text: &str);
    fn solve_time(&mut self, time: u32);
    fn solve(&mut self, src: &str, idx: usize, solve: &Solve, history: &mut History);
}

pub fn solve_time_string(time: u32) -> String {
    let time = (time + 5) / 10;
    if time > 6000 {
        format!(
            "{}:{:02}.{:02}",
            time / 6000,
            (time % 6000) / 100,
            time % 100
        )
    } else {
        format!("{}.{:02}", time / 100, time % 100)
    }
}

pub fn solve_time_short_string(time: u32) -> String {
    let time = time / 100;
    if time > 600 {
        format!("{}:{:02}.{}", time / 600, (time % 600) / 10, time % 10)
    } else {
        format!("{}.{}", time / 10, time % 10)
    }
}

impl CustomWidgets for Ui {
    fn header_label(&mut self, text: &str, active: bool) -> Response {
        self.add(
            if active {
                let color: Color32 = Theme::Green.into();
                Label::new(text).text_color(color)
            } else {
                Label::new(text)
            }
            .sense(Sense::click()),
        )
    }

    fn section(&mut self, text: &str) {
        let blue: Color32 = Theme::Blue.into();
        self.add(
            Label::new(text)
                .font_size(FontSize::Section)
                .text_color(blue),
        );
        self.scope(|ui| {
            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                width: 1.0,
                color: Theme::DarkBlue.into(),
            };
            ui.separator();
        });
    }

    fn solve_time(&mut self, time: u32) {
        self.label(solve_time_string(time));
    }

    fn solve(&mut self, src: &str, idx: usize, solve: &Solve, history: &mut History) {
        // Change window theme so that popup menu stands out
        let old_visuals = self.ctx().style().visuals.clone();
        self.ctx().set_visuals(crate::style::popup_visuals());

        let idx_color: Color32 = Theme::Disabled.into();
        self.horizontal(|ui| {
            ui.add(Label::new(format!("{}.", idx + 1)).text_color(idx_color));
            ui.with_layout(Layout::right_to_left(), |ui| {
                let popup_id = ui.make_persistent_id(format!("{}-{}", src, solve.id));
                let response = ui.add(Label::new("  ☰").small().sense(Sense::click()));
                if response.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                popup_below_widget(ui, popup_id, &response, |ui| {
                    ui.set_min_width(180.0);
                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::None => true,
                                _ => false,
                            },
                            "No penalty",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::None);
                        let _ = history.local_commit();
                    }

                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::Time(2000) => true,
                                _ => false,
                            },
                            "2 second penalty",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::Time(2000));
                        let _ = history.local_commit();
                    }

                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::DNF => true,
                                _ => false,
                            },
                            "DNF",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::DNF);
                        let _ = history.local_commit();
                    }

                    ui.separator();

                    if ui.selectable_label(false, "Delete solve").clicked() {
                        history.delete_solve(solve.id.clone());
                        let _ = history.local_commit();
                    }
                });

                if let Some(time) = solve.final_time() {
                    ui.solve_time(time);
                } else {
                    let red: Color32 = Theme::Red.into();
                    ui.add(Label::new("DNF").text_color(red));
                }
            });
        });

        // Restore old theme
        self.ctx().set_visuals(old_visuals);
    }
}
