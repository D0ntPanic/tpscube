use crate::cube::CubeRenderer;
use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::style::dialog_visuals;
use crate::theme::Theme;
use anyhow::{anyhow, Result};
use egui::{
    Color32, CtxRef, Direction, Label, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2, Window,
};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tpscube_core::{BluetoothCube, BluetoothCubeState, Cube, Cube3x3x3, TimedMove};

#[derive(Copy, Clone, PartialEq, Eq)]
enum BluetoothMode {
    DiscoverDevices,
    WaitForConnection,
    CheckState,
    ResetState,
    Finished,
    Error,
}

pub struct BluetoothState {
    mode: BluetoothMode,
    cube: Option<BluetoothCube>,
    error: Option<String>,
    renderer: CubeRenderer,
    move_queue: Arc<Mutex<Vec<(Vec<TimedMove>, Cube3x3x3)>>>,
    cube_state: Cube3x3x3,
}

impl BluetoothState {
    pub fn new() -> Self {
        Self {
            mode: BluetoothMode::DiscoverDevices,
            cube: None,
            error: None,
            renderer: CubeRenderer::new(),
            move_queue: Arc::new(Mutex::new(Vec::new())),
            cube_state: Cube3x3x3::new(),
        }
    }

    pub fn active(&self) -> bool {
        if let Some(cube) = &self.cube {
            if let Ok(state) = cube.state() {
                state == BluetoothCubeState::Connected
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn status(&self) -> String {
        if let Some(cube) = &self.cube {
            match cube.state() {
                Ok(BluetoothCubeState::Connected) => {
                    let mut string = if let Ok(Some(name)) = cube.name() {
                        format!("Connected to {}", name)
                    } else {
                        "Connected to Bluetooth cube".into()
                    };
                    if let Ok(Some(battery)) = cube.battery_percentage() {
                        string += &format!("\n🔋 {}%", battery);
                        if let Ok(Some(charging)) = cube.battery_charging() {
                            if charging {
                                string += " ⚡";
                            }
                        }
                    }
                    string
                }
                Ok(BluetoothCubeState::Connecting) => "Connecting...".into(),
                Ok(BluetoothCubeState::Discovering) => "Disconnected".into(),
                Ok(BluetoothCubeState::Desynced) => "Cube state desynced".into(),
                Ok(BluetoothCubeState::Error) => "Internal error".into(),
                Err(error) => format!("Connection error: {}", error),
            }
        } else {
            "Disconnected".into()
        }
    }

    pub fn status_color(&self) -> Color32 {
        if let Some(cube) = &self.cube {
            match cube.state() {
                Ok(BluetoothCubeState::Connected) => Theme::Content.into(),
                Ok(BluetoothCubeState::Desynced) => Theme::Red.into(),
                Err(_) => Theme::Red.into(),
                _ => Theme::Disabled.into(),
            }
        } else {
            Theme::Disabled.into()
        }
    }

    pub fn finished(&self) -> bool {
        self.mode == BluetoothMode::Finished
    }

    pub fn ready(&self) -> bool {
        self.finished() && self.active()
    }

    pub fn cube_state(&self) -> Cube3x3x3 {
        self.cube_state.clone()
    }

    pub fn name(&self) -> Option<String> {
        if let Some(cube) = &self.cube {
            if let Ok(name) = cube.name() {
                name
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn new_moves(&mut self) -> Vec<TimedMove> {
        let mut move_queue = self.move_queue.lock().unwrap();
        let mut result = Vec::new();
        for (moves, state) in move_queue.deref() {
            for mv in moves {
                result.push(mv.clone());
            }
            self.cube_state = state.clone();
        }
        move_queue.clear();
        result
    }

    pub fn disconnect(&mut self) {
        if let Some(cube) = &self.cube {
            cube.disconnect();
        }
    }

    pub fn start_connect_flow(&mut self, frame: &epi::Frame<'_>) {
        self.disconnect();
        self.mode = BluetoothMode::DiscoverDevices;
        if self.cube.is_none() {
            let cube = BluetoothCube::new();

            let repaint_signal = frame.repaint_signal();
            let move_queue = self.move_queue.clone();
            cube.register_move_listener(move |moves, state| {
                move_queue
                    .lock()
                    .unwrap()
                    .push((moves.to_vec(), state.clone()));
                repaint_signal.request_repaint();
            });

            self.cube = Some(cube);
        }
    }

    pub fn close(&mut self) {
        if self.mode != BluetoothMode::Finished {
            self.disconnect();
        }
    }

    fn discover_devices(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.add(Label::new(
                "Searching for supported Bluetooth cubes. Click a \
                    cube's name to connect.",
            ));

            ui.add_space(16.0);

            ScrollArea::from_max_height(350.0)
                .id_source("bluetooth_device_list")
                .show(ui, |ui| {
                    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Content.into(),
                    };
                    ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Green.into(),
                    };
                    ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Green.into(),
                    };
                    let cube = self.cube.as_ref().unwrap();
                    if let Ok(available_devices) = cube.available_devices() {
                        let mut at_least_one = false;
                        for device in available_devices {
                            if ui
                                .add(
                                    Label::new(format!("⮊  {}", device.name))
                                        .text_style(FontSize::Section.into())
                                        .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                match cube.connect(device.address) {
                                    Ok(_) => self.mode = BluetoothMode::WaitForConnection,
                                    Err(error) => {
                                        self.mode = BluetoothMode::Error;
                                        self.error = Some(error.to_string());
                                    }
                                }
                            }
                            at_least_one = true;
                        }
                        if !at_least_one {
                            ui.add(Label::new("Searching...").text_color(Theme::Disabled));
                        }
                    }
                });
        });
    }

    fn waiting_for_connection(&mut self, ui: &mut Ui) -> Result<()> {
        ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.add(
                        Label::new("Connecting to cube...")
                            .text_style(FontSize::Section.into())
                            .text_color(Theme::Disabled),
                    );
                },
            );
        });

        let cube = self.cube.as_ref().unwrap();
        match cube.state()? {
            BluetoothCubeState::Connected => {
                let state = cube.cube_state()?;
                self.cube_state = state.clone();
                self.renderer.set_cube_state(state);
                self.mode = BluetoothMode::CheckState;
            }
            BluetoothCubeState::Desynced => {
                self.mode = BluetoothMode::Error;
                self.error = Some("Cube state desynced".into());
            }
            BluetoothCubeState::Error => {
                self.mode = BluetoothMode::Error;
                self.error = Some("Internal error".into());
            }
            _ => (),
        }

        Ok(())
    }

    fn check_state(
        &mut self,
        ctxt: &CtxRef,
        ui: &mut Ui,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
    ) -> Result<()> {
        ui.vertical(|ui| {
            ui.label("Does this state match your cube?");

            ui.horizontal(|ui| {
                ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                };
                ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                };
                ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                };
                if ui
                    .add(
                        Label::new("✔  Yes")
                            .text_style(FontSize::Section.into())
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    self.mode = BluetoothMode::Finished;
                }

                ui.add_space(20.0);

                ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Red.into(),
                };
                ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Red.into(),
                };
                if ui
                    .add(
                        Label::new("✖  No")
                            .text_style(FontSize::Section.into())
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    self.mode = BluetoothMode::ResetState;
                    self.renderer.set_cube_state(Cube3x3x3::new());
                }
            });

            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(250.0, 250.0), Sense::click_and_drag());
            *cube_rect = Some(rect.clone());
            framerate.request_max();

            if ui.rect_contains_pointer(rect) {
                let scroll_delta = ctxt.input().scroll_delta;
                self.renderer
                    .adjust_angle(scroll_delta.x / 3.0, scroll_delta.y / 3.0);
            }
            if response.dragged() {
                self.renderer.adjust_angle(
                    ui.input().pointer.delta().x / 3.0,
                    ui.input().pointer.delta().y / 3.0,
                );
            }
        });

        for (moves, state) in self.move_queue.lock().unwrap().deref_mut().drain(..) {
            for mv in moves {
                self.renderer.do_move(mv.move_());
            }
            self.cube_state = state.clone();
            self.renderer.verify_state(state);
        }

        let cube = self.cube.as_ref().unwrap();
        if !cube.synced()? {
            return Err(anyhow!("Cube state desynced"));
        }

        Ok(())
    }

    fn reset_state(
        &mut self,
        ctxt: &CtxRef,
        ui: &mut Ui,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
    ) -> Result<()> {
        ui.vertical(|ui| {
            ui.label("Solve your cube to reset its state.");

            ui.horizontal(|ui| {
                ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Disabled.into(),
                };
                ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                };
                ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Green.into(),
                };
                if ui
                    .add(
                        Label::new("👍  I'm ready")
                            .text_style(FontSize::Section.into())
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    let cube = self.cube.as_ref().unwrap();
                    if let Err(error) = cube.reset_cube_state() {
                        self.mode = BluetoothMode::Error;
                        self.error = Some(error.to_string());
                    } else {
                        self.mode = BluetoothMode::Finished;
                        self.cube_state = Cube3x3x3::new();
                    }
                }
            });

            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(250.0, 250.0), Sense::click_and_drag());
            *cube_rect = Some(rect.clone());
            framerate.request_max();

            if ui.rect_contains_pointer(rect) {
                let scroll_delta = ctxt.input().scroll_delta;
                self.renderer
                    .adjust_angle(scroll_delta.x / 3.0, scroll_delta.y / 3.0);
            }
            if response.dragged() {
                self.renderer.adjust_angle(
                    ui.input().pointer.delta().x / 3.0,
                    ui.input().pointer.delta().y / 3.0,
                );
            }
        });

        let cube = self.cube.as_ref().unwrap();
        if !cube.synced()? {
            return Err(anyhow!("Cube state desynced"));
        }

        Ok(())
    }

    fn show_error(&self, ui: &mut Ui) {
        if let Some(error) = &self.error {
            ui.add(Label::new(error).text_color(Theme::Red));
        }
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
        open: &mut bool,
    ) {
        ctxt.set_visuals(dialog_visuals());
        Window::new("Connect")
            .fixed_size(Vec2::new(250.0, 300.0))
            .collapsible(false)
            .open(open)
            .show(ctxt, |ui| {
                ui.set_min_size(Vec2::new(250.0, 300.0));
                ui.set_max_size(Vec2::new(250.0, 300.0));
                match self.mode {
                    BluetoothMode::DiscoverDevices => self.discover_devices(ui),
                    BluetoothMode::WaitForConnection => match self.waiting_for_connection(ui) {
                        Ok(_) => (),
                        Err(error) => {
                            self.mode = BluetoothMode::Error;
                            self.error = Some(error.to_string());
                        }
                    },
                    BluetoothMode::CheckState => {
                        match self.check_state(ctxt, ui, framerate, cube_rect) {
                            Ok(_) => (),
                            Err(error) => {
                                self.mode = BluetoothMode::Error;
                                self.error = Some(error.to_string());
                            }
                        }
                    }
                    BluetoothMode::ResetState => {
                        match self.reset_state(ctxt, ui, framerate, cube_rect) {
                            Ok(_) => (),
                            Err(error) => {
                                self.mode = BluetoothMode::Error;
                                self.error = Some(error.to_string());
                            }
                        }
                    }
                    BluetoothMode::Error => self.show_error(ui),
                    _ => (),
                }
            });

        framerate.request(Some(10));
    }

    pub fn paint_cube(
        &mut self,
        ctxt: &CtxRef,
        gl: &mut GlContext<'_, '_>,
        rect: &Rect,
    ) -> Result<()> {
        self.renderer.draw(ctxt, gl, rect)
    }
}
