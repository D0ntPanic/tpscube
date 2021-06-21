use crate::font::{font_definitions, ScreenSize};
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::graph::GraphWidget;
use crate::history::HistoryWidget;
use crate::settings::SettingsWidget;
use crate::style::{base_visuals, content_visuals, header_visuals};
use crate::theme::Theme;
use crate::timer::TimerWidget;
use crate::widgets::CustomWidgets;
use anyhow::Result;
use egui::{widgets::Label, CentralPanel, Color32, CtxRef, Rect, Rgba, TopPanel, Vec2};
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
    timer_widget: TimerWidget,
    history_widget: HistoryWidget,
    graph_widget: GraphWidget,
    settings_widget: SettingsWidget,
    history: History,
    framerate: Option<Framerate>,
    timer_cube_rect: Option<Rect>,
    first_frame: bool,
    screen_size: ScreenSize,
}

pub struct ErrorApplication {
    message: String,
}

pub trait App {
    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(2560.0, 1600.0)
    }

    fn clear_color(&self) -> Rgba {
        Color32::from_rgba_premultiplied(12, 12, 12, 180).into()
    }

    fn setup(&mut self, _ctxt: &CtxRef) {}
    fn save(&mut self, _storage: &dyn epi::Storage) {}
    fn on_exit(&mut self) {}
    fn name(&self) -> &str;
    fn update(&mut self, ctxt: &CtxRef, frame: &mut epi::Frame<'_>);
    fn update_gl(&mut self, _ctxt: &CtxRef, _gl: &mut GlContext<'_, '_>) {}
}

impl Application {
    pub fn new() -> Result<Self> {
        let history = History::open()?;
        Ok(Application {
            mode: Mode::Timer,
            timer_widget: TimerWidget::new(),
            history_widget: HistoryWidget::new(),
            graph_widget: GraphWidget::new(),
            settings_widget: SettingsWidget::new(),
            history,
            framerate: None,
            timer_cube_rect: None,
            first_frame: true,
            screen_size: ScreenSize::Normal,
        })
    }
}

impl App for Application {
    fn setup(&mut self, ctxt: &CtxRef) {
        ctxt.set_fonts(font_definitions(self.screen_size));
        ctxt.set_visuals(base_visuals());
    }

    fn name(&self) -> &str {
        "TPS Cube"
    }

    fn update(&mut self, ctxt: &CtxRef, frame: &mut epi::Frame<'_>) {
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        let landscape = aspect > 1.0;
        let effective_height = if landscape {
            ctxt.available_rect().height()
        } else {
            ctxt.available_rect().height() * 0.75
        };
        let new_screen_size = if effective_height < 540.0 {
            ScreenSize::Small
        } else if effective_height < 800.0 {
            ScreenSize::Normal
        } else if effective_height < 1100.0 {
            ScreenSize::Large
        } else {
            ScreenSize::VeryLarge
        };

        if self.screen_size != new_screen_size {
            self.screen_size = new_screen_size;
            ctxt.set_fonts(font_definitions(self.screen_size));
        }

        ctxt.set_visuals(header_visuals());
        TopPanel::top("header").show(ctxt, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.style_mut().spacing.item_spacing.x = 20.0;

                    if ui
                        .header_label("⏱", "Timer", landscape, self.mode == Mode::Timer)
                        .clicked()
                    {
                        self.mode = Mode::Timer;
                    }

                    if ui
                        .header_label("📖", "History", landscape, self.mode == Mode::History)
                        .clicked()
                    {
                        self.mode = Mode::History;
                    }

                    if ui
                        .header_label("📉", "Graphs", landscape, self.mode == Mode::Graphs)
                        .clicked()
                    {
                        self.mode = Mode::Graphs;
                    }

                    if ui
                        .header_label("⚙", "Settings", landscape, self.mode == Mode::Settings)
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

        self.timer_cube_rect = None;

        match self.mode {
            Mode::Timer => self.timer_widget.update(
                ctxt,
                frame,
                &mut self.history,
                framerate,
                &mut self.timer_cube_rect,
            ),
            Mode::History => {
                self.history_widget.update(ctxt, frame, &mut self.history);
                framerate.set_target(None);
            }
            Mode::Graphs => {
                self.graph_widget.update(ctxt, frame, &mut self.history);
                framerate.set_target(None);
            }
            Mode::Settings => {
                self.settings_widget.update(ctxt, frame, &mut self.history);
                framerate.set_target(None);
            }
        }

        if self.first_frame {
            // On some devices the 3D elements don't render properly on the first frame. Render
            // a second frame immediately.
            ctxt.request_repaint();
            self.first_frame = false;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn update_gl(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>) {
        if let Some(rect) = &self.timer_cube_rect {
            self.timer_widget.paint_cube(ctxt, gl, rect).unwrap();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update_gl(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>) {
        if let Some(rect) = &self.timer_cube_rect {
            self.timer_widget.paint_cube(ctxt, gl, rect).unwrap();
        }
    }
}

impl ErrorApplication {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl App for ErrorApplication {
    fn setup(&mut self, ctxt: &CtxRef) {
        ctxt.set_fonts(font_definitions(ScreenSize::Normal));
        ctxt.set_visuals(base_visuals());
    }

    fn name(&self) -> &str {
        "TPS Cube"
    }

    fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>) {
        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(Label::new(format!("Error: {}", self.message)).text_color(Theme::Red));
            })
        });
    }
}
