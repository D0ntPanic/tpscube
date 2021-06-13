use crate::font::font_definitions;
use crate::framerate::Framerate;
use crate::style::{base_visuals, content_visuals, header_visuals};
use crate::theme::Theme;
use crate::timer::Timer;
use crate::widgets::CustomWidgets;
use anyhow::Result;
use eframe::{
    egui::{widgets::Label, CentralPanel, Color32, CtxRef, TopPanel, Vec2},
    epi,
};
use tpscube_core::History;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Timer,
    History,
    Graphs,
    Settings,
}

pub struct Application {
    mode: Mode,
    timer: Timer,
    history: History,
    framerate: Option<Framerate>,
}

pub struct ErrorApplication {
    message: String,
}

impl Application {
    pub fn new() -> Result<Self> {
        let history = History::open()?;
        Ok(Application {
            mode: Mode::Timer,
            timer: Timer::new(),
            history,
            framerate: None,
        })
    }
}

impl epi::App for Application {
    fn setup(&mut self, ctxt: &CtxRef) {
        ctxt.set_fonts(font_definitions());
        ctxt.set_visuals(base_visuals());
    }

    fn name(&self) -> &str {
        "TPS Cube"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(2560.0, 1600.0)
    }

    fn update(&mut self, ctxt: &CtxRef, frame: &mut epi::Frame<'_>) {
        ctxt.set_visuals(header_visuals());
        TopPanel::top("header").show(ctxt, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.style_mut().spacing.item_spacing.x = 20.0;

                    if ui
                        .header_label("⏱  Timer", self.mode == Mode::Timer)
                        .clicked()
                    {
                        self.mode = Mode::Timer;
                    }

                    if ui
                        .header_label("📖  History", self.mode == Mode::History)
                        .clicked()
                    {
                        self.mode = Mode::History;
                    }

                    if ui
                        .header_label("📉  Graphs", self.mode == Mode::Graphs)
                        .clicked()
                    {
                        self.mode = Mode::Graphs;
                    }

                    if ui
                        .header_label("⚙  Settings", self.mode == Mode::Settings)
                        .clicked()
                    {
                        self.mode = Mode::Settings;
                    }
                });

                ui.add_space(5.0);
            });
        });

        let framerate = if let Some(framerate) = &self.framerate {
            framerate
        } else {
            self.framerate = Some(Framerate::new(frame.repaint_signal().clone()));
            self.framerate.as_ref().unwrap()
        };

        match self.mode {
            Mode::Timer => self.timer.update(ctxt, frame, &mut self.history, framerate),
            _ => framerate.set_target(None),
        }
    }
}

impl ErrorApplication {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl epi::App for ErrorApplication {
    fn setup(&mut self, ctxt: &CtxRef) {
        ctxt.set_fonts(font_definitions());
        ctxt.set_visuals(base_visuals());
    }

    fn name(&self) -> &str {
        "TPS Cube"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(2560.0, 1600.0)
    }

    fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.centered_and_justified(|ui| {
                let red: Color32 = Theme::Red.into();
                ui.add(Label::new(format!("Error: {}", self.message)).text_color(red));
            })
        });
    }
}
