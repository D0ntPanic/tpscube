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
use egui::{
    widgets::Label, CentralPanel, Color32, CtxRef, Layout, Rect, Rgba, Sense, Stroke, TextureId,
    TopBottomPanel, Vec2,
};
use image::GenericImageView;
use tpscube_core::{History, SyncStatus};

#[cfg(not(target_arch = "wasm32"))]
use crate::bluetooth::BluetoothState;

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
    bluetooth_cube_rect: Option<Rect>,
    first_frame: bool,
    screen_size: ScreenSize,

    #[cfg(not(target_arch = "wasm32"))]
    bluetooth: BluetoothState,

    bluetooth_icon: Icon,
    bluetooth_dialog_open: bool,
}

pub struct ErrorApplication {
    message: String,
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color32>,
    texture_id: Option<TextureId>,
}

enum IconState {
    Inactive,
    Hovered,
    Active,
}

struct Icon {
    inactive: Image,
    hover: Image,
    active: Image,
    state: IconState,
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

        let bluetooth_inactive =
            Image::new(include_bytes!("../images/bluetooth_deselect.png")).unwrap();
        let bluetooth_hover = Image::new(include_bytes!("../images/bluetooth_hover.png")).unwrap();
        let bluetooth_active =
            Image::new(include_bytes!("../images/bluetooth_active.png")).unwrap();
        let bluetooth_icon = Icon {
            inactive: bluetooth_inactive,
            hover: bluetooth_hover,
            active: bluetooth_active,
            state: IconState::Inactive,
        };

        Ok(Application {
            mode: Mode::Timer,
            timer_widget: TimerWidget::new(),
            history_widget: HistoryWidget::new(),
            graph_widget: GraphWidget::new(),
            settings_widget: SettingsWidget::new(),
            history,
            framerate: None,
            timer_cube_rect: None,
            bluetooth_cube_rect: None,
            first_frame: true,
            screen_size: ScreenSize::Normal,

            #[cfg(not(target_arch = "wasm32"))]
            bluetooth: BluetoothState::new(),

            bluetooth_icon,
            bluetooth_dialog_open: false,
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
        TopBottomPanel::top("header").show(ctxt, |ui| {
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

                    // Check status of sync and create tooltip text for sync button
                    let sync_status = self.history.check_sync_status();
                    let local_count = self.history.local_action_count();
                    let local_status = match local_count {
                        0 => "No new solves to sync.".into(),
                        count => format!("{} actions to sync.", count),
                    };
                    let sync_status = match sync_status {
                        SyncStatus::NotSynced => local_status,
                        SyncStatus::SyncPending => {
                            if local_count != 0 {
                                format!("{}\nSync in progress...", local_status)
                            } else {
                                "Sync in progress...".into()
                            }
                        }
                        SyncStatus::SyncFailed(message) => {
                            format!("{}\nSync failed: {}", local_status, message)
                        }
                        SyncStatus::SyncComplete => {
                            if local_count != 0 {
                                local_status
                            } else {
                                "Sync complete".into()
                            }
                        }
                    };

                    // Show icons on the right of the header
                    ui.style_mut().spacing.item_spacing.x = 12.0;
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        // Show sync button
                        if self.history.sync_in_progress() {
                            ui.style_mut().visuals.widgets.inactive.fg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Blue.into(),
                            };
                            ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Blue.into(),
                            };
                            ui.style_mut().visuals.widgets.active.fg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Blue.into(),
                            };
                        }
                        if ui
                            .add(
                                Label::new(if local_count == 0 {
                                    "🔃".into()
                                } else {
                                    format!("🔃 {}", local_count)
                                })
                                .sense(Sense::click()),
                            )
                            .on_hover_text(sync_status)
                            .clicked()
                        {
                            self.history.start_sync();
                        }

                        // Show bluetooth button
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(texture_id) = self.bluetooth_icon.texture(frame) {
                            let response = ui.add(
                                egui::Image::new(texture_id, Vec2::new(20.0, 20.0))
                                    .sense(Sense::click()),
                            );
                            if response.hovered() {
                                self.bluetooth_icon.state = IconState::Hovered;
                            } else if self.bluetooth.active() {
                                self.bluetooth_icon.state = IconState::Active;
                            } else {
                                self.bluetooth_icon.state = IconState::Inactive;
                            }
                            if response.clicked() {
                                if self.bluetooth.active() {
                                    self.bluetooth.disconnect();
                                } else {
                                    self.bluetooth_dialog_open = true;
                                    self.bluetooth.start_connect_flow(frame);
                                }
                            }
                            response.on_hover_ui(|ui| {
                                ui.add(
                                    Label::new(self.bluetooth.status())
                                        .text_color(self.bluetooth.status_color()),
                                );
                            });
                        }
                    });
                });

                ui.add_space(5.0);
            });
        });

        let framerate = if let Some(framerate) = &mut self.framerate {
            framerate
        } else {
            self.framerate = Some(Framerate::new(frame.repaint_signal().clone()));
            self.framerate.as_mut().unwrap()
        };

        self.timer_cube_rect = None;
        self.bluetooth_cube_rect = None;

        if self.history.sync_in_progress() {
            framerate.request(Some(10));
        }

        match self.mode {
            Mode::Timer => {
                #[cfg(target_arch = "wasm32")]
                let (bluetooth_state, bluetooth_moves) = (None, Vec::new());
                #[cfg(not(target_arch = "wasm32"))]
                let (bluetooth_state, bluetooth_moves) =
                    if !self.bluetooth_dialog_open && self.bluetooth.ready() {
                        let moves = self.bluetooth.new_moves();
                        let state = self.bluetooth.cube_state();
                        (Some(state), moves)
                    } else {
                        (None, Vec::new())
                    };

                self.timer_widget.update(
                    ctxt,
                    frame,
                    &mut self.history,
                    bluetooth_state,
                    bluetooth_moves,
                    framerate,
                    &mut self.timer_cube_rect,
                )
            }
            Mode::History => self.history_widget.update(ctxt, frame, &mut self.history),
            Mode::Graphs => self.graph_widget.update(ctxt, frame, &mut self.history),
            Mode::Settings => self.settings_widget.update(ctxt, frame, &mut self.history),
        }

        #[cfg(not(target_arch = "wasm32"))]
        if self.bluetooth_dialog_open {
            let mut open = true;
            self.bluetooth.update(
                ctxt,
                frame,
                framerate,
                &mut self.bluetooth_cube_rect,
                &mut open,
            );
            if !open {
                self.bluetooth_dialog_open = false;
                self.bluetooth.close();
            }

            if self.bluetooth.finished() {
                self.bluetooth_dialog_open = false;
            }
        }

        framerate.commit();

        if self.first_frame {
            // On some devices the 3D elements don't render properly on the first frame. Render
            // a second frame immediately.
            ctxt.request_repaint();
            self.first_frame = false;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn update_gl(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>) {
        if !self.bluetooth_dialog_open {
            if let Some(rect) = &self.timer_cube_rect {
                self.timer_widget.paint_cube(ctxt, gl, rect).unwrap();
            }
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(rect) = &self.bluetooth_cube_rect {
                self.bluetooth.paint_cube(ctxt, gl, rect).unwrap();
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update_gl(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>) {
        if !self.bluetooth_dialog_open {
            if let Some(rect) = &self.timer_cube_rect {
                self.timer_widget.paint_cube(ctxt, gl, rect).unwrap();
            }
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(rect) = &self.bluetooth_cube_rect {
                self.bluetooth.paint_cube(ctxt, gl, rect).unwrap();
            }
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

impl Image {
    fn new(png: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(png)?;
        let image_rgb = image.to_rgba8();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let pixels = image_rgb
            .into_vec()
            .chunks(4)
            .map(|rgba| Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]))
            .collect();
        Ok(Self {
            width,
            height,
            pixels,
            texture_id: None,
        })
    }

    fn texture(&mut self, frame: &mut epi::Frame<'_>) -> Option<TextureId> {
        if let Some(texture_id) = self.texture_id {
            Some(texture_id)
        } else {
            self.texture_id = Some(
                frame
                    .tex_allocator()
                    .alloc_srgba_premultiplied((self.width, self.height), &self.pixels),
            );
            self.texture_id
        }
    }
}

impl Icon {
    fn texture(&mut self, frame: &mut epi::Frame<'_>) -> Option<TextureId> {
        match self.state {
            IconState::Inactive => self.inactive.texture(frame),
            IconState::Hovered => self.hover.texture(frame),
            IconState::Active => self.active.texture(frame),
        }
    }
}
