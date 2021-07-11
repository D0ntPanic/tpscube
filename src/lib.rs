pub mod app;
mod center_generated;
mod corner_generated;
mod cube;
mod edge_generated;
mod font;
mod framerate;
mod future;
mod gl;
mod graph;
mod history;
mod settings;
mod style;
mod theme;
mod timer;
mod widgets;

#[cfg(not(target_arch = "wasm32"))]
mod bluetooth;

// This code is from backend.rs and lib.rs in egui_web, but modified to allow for
// rendering of 3D elements.

#[cfg(target_arch = "wasm32")]
use egui_web::{webgl1, NeedRepaint, Painter, WebInput};
#[cfg(target_arch = "wasm32")]
use gl::GlContext;
#[cfg(target_arch = "wasm32")]
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, Mutex};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
static AGENT_ID: &str = "egui_text_agent";

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
struct WebBackend {
    egui_ctx: egui::CtxRef,
    canvas: web_sys::HtmlCanvasElement,
    gl: web_sys::WebGlRenderingContext,
    painter: webgl1::WebGlPainter,
    previous_frame_time: Option<f32>,
    frame_start: Option<f64>,
}

#[cfg(target_arch = "wasm32")]
impl WebBackend {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        use wasm_bindgen::JsCast;

        let ctx = egui::CtxRef::default();

        let canvas = egui_web::canvas_element_or_die(canvas_id);

        let gl = canvas
            .get_context("webgl")?
            .ok_or_else(|| JsValue::from("Failed to get WebGl context"))?
            .dyn_into::<web_sys::WebGlRenderingContext>()?;

        let painter = if let Ok(webgl1_painter) = webgl1::WebGlPainter::new(canvas_id) {
            webgl1_painter
        } else {
            panic!("WebGL required");
        };

        Ok(Self {
            egui_ctx: ctx,
            canvas,
            gl,
            painter,
            previous_frame_time: None,
            frame_start: None,
        })
    }

    /// id of the canvas html element containing the rendering
    pub fn canvas_id(&self) -> &str {
        self.painter.canvas_id()
    }

    pub fn begin_frame(&mut self, raw_input: egui::RawInput) {
        self.frame_start = Some(egui_web::now_sec());
        self.egui_ctx.begin_frame(raw_input)
    }

    pub fn end_frame(&mut self) -> Result<(egui::Output, Vec<egui::ClippedMesh>), JsValue> {
        let frame_start = self
            .frame_start
            .take()
            .expect("unmatched calls to begin_frame/end_frame");

        let (output, shapes) = self.egui_ctx.end_frame();
        let clipped_meshes = self.egui_ctx.tessellate(shapes);

        let now = egui_web::now_sec();
        self.previous_frame_time = Some((now - frame_start) as f32);

        Ok((output, clipped_meshes))
    }

    pub fn paint(
        &mut self,
        clear_color: egui::Rgba,
        clipped_meshes: Vec<egui::ClippedMesh>,
    ) -> Result<(), JsValue> {
        self.painter.upload_egui_texture(&self.egui_ctx.texture());
        self.painter.clear(clear_color);
        self.painter
            .paint_meshes(clipped_meshes, self.egui_ctx.pixels_per_point())
    }
}

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
struct AppRunner {
    web_backend: WebBackend,
    pub(crate) input: WebInput,
    app: Box<dyn app::App>,
    pub(crate) needs_repaint: std::sync::Arc<NeedRepaint>,
    storage: egui_web::LocalStorage,
    last_save_time: f64,
    screen_reader: egui_web::screen_reader::ScreenReader,
    pub(crate) last_text_cursor_pos: Option<egui::Pos2>,
}

#[cfg(target_arch = "wasm32")]
impl AppRunner {
    pub fn new(web_backend: WebBackend, app: Box<dyn app::App>) -> Result<Self, JsValue> {
        egui_web::load_memory(&web_backend.egui_ctx);

        let storage = egui_web::LocalStorage::default();

        let mut runner = Self {
            web_backend,
            input: Default::default(),
            app,
            needs_repaint: Default::default(),
            storage,
            last_save_time: egui_web::now_sec(),
            screen_reader: Default::default(),
            last_text_cursor_pos: None,
        };

        runner.app.setup(&runner.web_backend.egui_ctx);
        Ok(runner)
    }

    pub fn egui_ctx(&self) -> &egui::CtxRef {
        &self.web_backend.egui_ctx
    }

    pub fn auto_save(&mut self) {
        let now = egui_web::now_sec();
        let time_since_last_save = now - self.last_save_time;

        if time_since_last_save > self.app.auto_save_interval().as_secs_f64() {
            egui_web::save_memory(&self.web_backend.egui_ctx);
            self.app.save(&mut self.storage);
            self.last_save_time = now;
        }
    }

    pub fn canvas_id(&self) -> &str {
        self.web_backend.canvas_id()
    }

    pub fn warm_up(&mut self) -> Result<(), JsValue> {
        if self.app.warm_up_enabled() {
            let saved_memory = self.web_backend.egui_ctx.memory().clone();
            self.web_backend
                .egui_ctx
                .memory()
                .set_everything_is_visible(true);
            self.logic()?;
            *self.web_backend.egui_ctx.memory() = saved_memory; // We don't want to remember that windows were huge.
            self.web_backend.egui_ctx.clear_animations();
        }
        Ok(())
    }

    fn integration_info(&self) -> epi::IntegrationInfo {
        epi::IntegrationInfo {
            web_info: Some(epi::WebInfo {
                web_location_hash: egui_web::location_hash().unwrap_or_default(),
            }),
            prefer_dark_mode: Some(true),
            cpu_usage: self.web_backend.previous_frame_time,
            seconds_since_midnight: Some(egui_web::seconds_since_midnight()),
            native_pixels_per_point: Some(egui_web::native_pixels_per_point()),
        }
    }

    pub fn logic(&mut self) -> Result<(egui::Output, Vec<egui::ClippedMesh>), JsValue> {
        egui_web::resize_canvas_to_screen_size(
            self.web_backend.canvas_id(),
            self.app.max_size_points(),
        );
        let canvas_size = egui_web::canvas_size_in_points(self.web_backend.canvas_id());
        let raw_input = self.input.new_frame(canvas_size);

        self.web_backend.begin_frame(raw_input);

        let mut app_output = epi::backend::AppOutput::default();
        let mut frame = epi::backend::FrameBuilder {
            info: self.integration_info(),
            tex_allocator: self.web_backend.painter.as_tex_allocator(),
            output: &mut app_output,
            repaint_signal: self.needs_repaint.clone(),
        }
        .build();

        self.app.update(&self.web_backend.egui_ctx, &mut frame);
        let (egui_output, clipped_meshes) = self.web_backend.end_frame()?;

        if self.web_backend.egui_ctx.memory().options.screen_reader {
            self.screen_reader.speak(&egui_output.events_description());
        }
        handle_output(&egui_output, self);

        {
            let epi::backend::AppOutput {
                quit: _,        // Can't quit a web page
                window_size: _, // Can't resize a web page
            } = app_output;
        }

        Ok((egui_output, clipped_meshes))
    }

    pub fn paint(&mut self, clipped_meshes: Vec<egui::ClippedMesh>) -> Result<(), JsValue> {
        self.web_backend
            .paint(self.app.clear_color(), clipped_meshes)?;

        let mut gl = GlContext {
            canvas: &self.web_backend.canvas,
            ctxt: &self.web_backend.gl,
        };
        self.app.update_gl(&self.web_backend.egui_ctx, &mut gl);

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_output(output: &egui::Output, runner: &mut AppRunner) {
    let egui::Output {
        cursor_icon,
        open_url,
        copied_text,
        needs_repaint: _, // handled elsewhere
        events: _,        // we ignore these (TODO: accessibility screen reader)
        text_cursor_pos,
    } = output;

    egui_web::set_cursor_icon(*cursor_icon);
    if let Some(open) = open_url {
        egui_web::open_url(&open.url, open.new_tab);
    }

    if !copied_text.is_empty() {
        set_clipboard_text(copied_text);
    }

    if &runner.last_text_cursor_pos != text_cursor_pos {
        move_text_cursor(text_cursor_pos, runner.canvas_id());
        runner.last_text_cursor_pos = *text_cursor_pos;
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_clipboard_text(s: &str) {
    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let promise = clipboard.write_text(s);
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        let future = async move {
            if let Err(err) = future.await {
                web_sys::console::error_1(&format!("Copy/cut action denied: {:?}", err).into());
            }
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct AppRunnerRef(Arc<Mutex<AppRunner>>);

#[cfg(target_arch = "wasm32")]
fn paint_and_schedule(runner_ref: AppRunnerRef) -> Result<(), JsValue> {
    fn paint_if_needed(runner_ref: &AppRunnerRef) -> Result<(), JsValue> {
        let mut runner_lock = runner_ref.0.lock().unwrap();
        if runner_lock.needs_repaint.fetch_and_clear() {
            let (output, clipped_meshes) = runner_lock.logic()?;
            runner_lock.paint(clipped_meshes)?;
            if output.needs_repaint {
                runner_lock.needs_repaint.set_true();
            }
            runner_lock.auto_save();
        }

        Ok(())
    }

    fn request_animation_frame(runner_ref: AppRunnerRef) -> Result<(), JsValue> {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().unwrap();
        let closure = Closure::once(move || paint_and_schedule(runner_ref));
        window.request_animation_frame(closure.as_ref().unchecked_ref())?;
        closure.forget(); // We must forget it, or else the callback is canceled on drop
        Ok(())
    }

    paint_if_needed(&runner_ref)?;
    request_animation_frame(runner_ref)
}

#[cfg(target_arch = "wasm32")]
fn text_agent() -> web_sys::HtmlInputElement {
    use wasm_bindgen::JsCast;
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(AGENT_ID)
        .unwrap()
        .dyn_into()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn install_document_events(runner_ref: &AppRunnerRef) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    {
        // keydown
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.is_composing() || event.key_code() == 229 {
                // https://www.fxsitecompat.dev/en-CA/docs/2018/keydown-and-keyup-events-are-now-fired-during-ime-composition/
                return;
            }

            let mut runner_lock = runner_ref.0.lock().unwrap();
            let modifiers = modifiers_from_event(&event);
            runner_lock.input.raw.modifiers = modifiers;

            let key = event.key();

            if let Some(key) = egui_web::translate_key(&key) {
                runner_lock.input.raw.events.push(egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                });
            }
            if !modifiers.ctrl
                && !modifiers.command
                && !should_ignore_key(&key)
                // When text agent is shown, it sends text event instead.
                && text_agent().hidden()
            {
                runner_lock.input.raw.events.push(egui::Event::Text(key));
            }
            runner_lock.needs_repaint.set_true();

            let egui_wants_keyboard = runner_lock.egui_ctx().wants_keyboard_input();

            let prevent_default = if matches!(event.key().as_str(), "Tab") {
                // Always prevent moving cursor to url bar.
                // egui wants to use tab to move to the next text field.
                true
            } else if egui_wants_keyboard {
                matches!(
                    event.key().as_str(),
                    "Backspace" // so we don't go back to previous page when deleting text
                | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "ArrowUp" // cmd-left is "back" on Mac (https://github.com/emilk/egui/issues/58)
                )
            } else {
                // We never want to prevent:
                // * F5 / cmd-R (refresh)
                // * cmd-shift-C (debug tools)
                // * cmd/ctrl-c/v/x (or we stop copy/past/cut events)
                false
            };

            // console_log(format!(
            //     "On key-down {:?}, egui_wants_keyboard: {}, prevent_default: {}",
            //     event.key().as_str(),
            //     egui_wants_keyboard,
            //     prevent_default
            // ));

            if prevent_default {
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        // keyup
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            let modifiers = modifiers_from_event(&event);
            runner_lock.input.raw.modifiers = modifiers;
            if let Some(key) = egui_web::translate_key(&event.key()) {
                runner_lock.input.raw.events.push(egui::Event::Key {
                    key,
                    pressed: false,
                    modifiers,
                });
            }
            runner_lock.needs_repaint.set_true();
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    for event_name in &["load", "pagehide", "pageshow", "resize"] {
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move || {
            runner_ref.0.lock().unwrap().needs_repaint.set_true();
        }) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

/// Repaint at least every `ms` milliseconds.
#[cfg(target_arch = "wasm32")]
fn repaint_every_ms(runner_ref: &AppRunnerRef, milliseconds: i32) -> Result<(), JsValue> {
    assert!(milliseconds >= 0);
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let runner_ref = runner_ref.clone();
    let closure = Closure::wrap(Box::new(move || {
        runner_ref.0.lock().unwrap().needs_repaint.set_true();
    }) as Box<dyn FnMut()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        milliseconds,
    )?;
    closure.forget();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn modifiers_from_event(event: &web_sys::KeyboardEvent) -> egui::Modifiers {
    egui::Modifiers {
        alt: event.alt_key(),
        ctrl: event.ctrl_key(),
        shift: event.shift_key(),

        // Ideally we should know if we are running or mac or not,
        // but this works good enough for now.
        mac_cmd: event.meta_key(),

        // Ideally we should know if we are running or mac or not,
        // but this works good enough for now.
        command: event.ctrl_key() || event.meta_key(),
    }
}

///
/// Text event handler,
#[cfg(target_arch = "wasm32")]
fn install_text_agent(runner_ref: &AppRunnerRef) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().expect("document should have a body");
    let input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    let input = std::rc::Rc::new(input);
    input.set_id(AGENT_ID);
    let is_composing = Rc::new(Cell::new(false));
    {
        let style = input.style();
        // Transparent
        style.set_property("opacity", "0").unwrap();
        // Hide under canvas
        style.set_property("z-index", "-1").unwrap();
    }
    // Set size as small as possible, in case user may click on it.
    input.set_size(1);
    input.set_autofocus(true);
    input.set_hidden(true);
    {
        // When IME is off
        let input_clone = input.clone();
        let runner_ref = runner_ref.clone();
        let is_composing = is_composing.clone();
        let on_input = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
            let text = input_clone.value();
            if !text.is_empty() && !is_composing.get() {
                input_clone.set_value("");
                let mut runner_lock = runner_ref.0.lock().unwrap();
                runner_lock.input.raw.events.push(egui::Event::Text(text));
                runner_lock.needs_repaint.set_true();
            }
        }) as Box<dyn FnMut(_)>);
        input.add_event_listener_with_callback("input", on_input.as_ref().unchecked_ref())?;
        on_input.forget();
    }
    {
        // When IME is on, handle composition event
        let input_clone = input.clone();
        let runner_ref = runner_ref.clone();
        let on_compositionend = Closure::wrap(Box::new(move |event: web_sys::CompositionEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            let opt_event = match event.type_().as_ref() {
                "compositionstart" => {
                    is_composing.set(true);
                    input_clone.set_value("");
                    Some(egui::Event::CompositionStart)
                }
                "compositionend" => {
                    is_composing.set(false);
                    input_clone.set_value("");
                    event.data().map(egui::Event::CompositionEnd)
                }
                "compositionupdate" => event.data().map(egui::Event::CompositionUpdate),
                s => {
                    egui_web::console_error(format!("Unknown composition event type: {:?}", s));
                    None
                }
            };
            if let Some(event) = opt_event {
                runner_lock.input.raw.events.push(event);
                runner_lock.needs_repaint.set_true();
            }
        }) as Box<dyn FnMut(_)>);
        let f = on_compositionend.as_ref().unchecked_ref();
        input.add_event_listener_with_callback("compositionstart", f)?;
        input.add_event_listener_with_callback("compositionupdate", f)?;
        input.add_event_listener_with_callback("compositionend", f)?;
        on_compositionend.forget();
    }
    {
        // When input lost focus, focus on it again.
        // It is useful when user click somewhere outside canvas.
        let on_focusout = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            // Delay 10 ms, and focus again.
            let func = js_sys::Function::new_no_args(&format!(
                "document.getElementById('{}').focus()",
                AGENT_ID
            ));
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&func, 10)
                .unwrap();
        }) as Box<dyn FnMut(_)>);
        input.add_event_listener_with_callback("focusout", on_focusout.as_ref().unchecked_ref())?;
        on_focusout.forget();
    }
    body.append_child(&input)?;
    Ok(())
}

/// Web sends all keys as strings, so it is up to us to figure out if it is
/// a real text input or the name of a key.
#[cfg(target_arch = "wasm32")]
fn should_ignore_key(key: &str) -> bool {
    let is_function_key = key.starts_with('F') && key.len() > 1;
    is_function_key
        || matches!(
            key,
            "Alt"
                | "ArrowDown"
                | "ArrowLeft"
                | "ArrowRight"
                | "ArrowUp"
                | "Backspace"
                | "CapsLock"
                | "ContextMenu"
                | "Control"
                | "Delete"
                | "End"
                | "Enter"
                | "Esc"
                | "Escape"
                | "Help"
                | "Home"
                | "Insert"
                | "Meta"
                | "NumLock"
                | "PageDown"
                | "PageUp"
                | "Pause"
                | "ScrollLock"
                | "Shift"
                | "Tab"
        )
}

#[cfg(target_arch = "wasm32")]
fn pos_from_touch(canvas_origin: egui::Pos2, touch: &web_sys::Touch) -> egui::Pos2 {
    egui::Pos2 {
        x: touch.page_x() as f32 - canvas_origin.x as f32,
        y: touch.page_y() as f32 - canvas_origin.y as f32,
    }
}

#[cfg(target_arch = "wasm32")]
fn canvas_origin(canvas_id: &str) -> egui::Pos2 {
    let rect = egui_web::canvas_element(canvas_id)
        .unwrap()
        .get_bounding_client_rect();
    egui::Pos2::new(rect.left() as f32, rect.top() as f32)
}

#[cfg(target_arch = "wasm32")]
fn push_touches(runner: &mut AppRunner, phase: egui::TouchPhase, event: &web_sys::TouchEvent) {
    let canvas_origin = canvas_origin(runner.canvas_id());
    for touch_idx in 0..event.changed_touches().length() {
        if let Some(touch) = event.changed_touches().item(touch_idx) {
            runner.input.raw.events.push(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId::from(touch.identifier()),
                phase,
                pos: pos_from_touch(canvas_origin, &touch),
                force: touch.force(),
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn install_canvas_events(runner_ref: &AppRunnerRef) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    let canvas = egui_web::canvas_element(runner_ref.0.lock().unwrap().canvas_id()).unwrap();

    {
        // By default, right-clicks open a context menu.
        // We don't want to do that (right clicks is handled by egui):
        let event_name = "contextmenu";
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "mousedown";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            if !runner_lock.input.is_touch {
                if let Some(button) = egui_web::button_from_mouse_event(&event) {
                    let pos = egui_web::pos_from_mouse_event(runner_lock.canvas_id(), &event);
                    let modifiers = runner_lock.input.raw.modifiers;
                    runner_lock
                        .input
                        .raw
                        .events
                        .push(egui::Event::PointerButton {
                            pos,
                            button,
                            pressed: true,
                            modifiers,
                        });
                    runner_lock.needs_repaint.set_true();
                    event.stop_propagation();
                    event.prevent_default();
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "mousemove";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            if !runner_lock.input.is_touch {
                let pos = egui_web::pos_from_mouse_event(runner_lock.canvas_id(), &event);
                runner_lock
                    .input
                    .raw
                    .events
                    .push(egui::Event::PointerMoved(pos));
                runner_lock.needs_repaint.set_true();
                event.stop_propagation();
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "mouseup";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            if !runner_lock.input.is_touch {
                if let Some(button) = egui_web::button_from_mouse_event(&event) {
                    let pos = egui_web::pos_from_mouse_event(runner_lock.canvas_id(), &event);
                    let modifiers = runner_lock.input.raw.modifiers;
                    runner_lock
                        .input
                        .raw
                        .events
                        .push(egui::Event::PointerButton {
                            pos,
                            button,
                            pressed: false,
                            modifiers,
                        });
                    runner_lock.needs_repaint.set_true();
                    event.stop_propagation();
                    event.prevent_default();
                }
                manipulate_agent(runner_lock.canvas_id(), runner_lock.input.latest_touch_pos);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "mouseleave";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            if !runner_lock.input.is_touch {
                runner_lock.input.raw.events.push(egui::Event::PointerGone);
                runner_lock.needs_repaint.set_true();
                event.stop_propagation();
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "touchstart";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            let mut latest_touch_pos_id = runner_lock.input.latest_touch_pos_id;
            let pos = egui_web::pos_from_touch_event(
                runner_lock.canvas_id(),
                &event,
                &mut latest_touch_pos_id,
            );
            runner_lock.input.latest_touch_pos_id = latest_touch_pos_id;
            runner_lock.input.latest_touch_pos = Some(pos);
            runner_lock.input.is_touch = true;
            let modifiers = runner_lock.input.raw.modifiers;
            runner_lock
                .input
                .raw
                .events
                .push(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers,
                });

            push_touches(&mut *runner_lock, egui::TouchPhase::Start, &event);
            runner_lock.needs_repaint.set_true();
            event.stop_propagation();
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "touchmove";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            let mut latest_touch_pos_id = runner_lock.input.latest_touch_pos_id;
            let pos = egui_web::pos_from_touch_event(
                runner_lock.canvas_id(),
                &event,
                &mut latest_touch_pos_id,
            );
            runner_lock.input.latest_touch_pos_id = latest_touch_pos_id;
            runner_lock.input.latest_touch_pos = Some(pos);
            runner_lock.input.is_touch = true;
            runner_lock
                .input
                .raw
                .events
                .push(egui::Event::PointerMoved(pos));

            push_touches(&mut *runner_lock, egui::TouchPhase::Move, &event);
            runner_lock.needs_repaint.set_true();
            event.stop_propagation();
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "touchend";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            runner_lock.input.is_touch = true;
            if let Some(pos) = runner_lock.input.latest_touch_pos {
                let modifiers = runner_lock.input.raw.modifiers;
                // First release mouse to click:
                runner_lock
                    .input
                    .raw
                    .events
                    .push(egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers,
                    });
                // Then remove hover effect:
                runner_lock.input.raw.events.push(egui::Event::PointerGone);

                push_touches(&mut *runner_lock, egui::TouchPhase::End, &event);
                runner_lock.needs_repaint.set_true();
                event.stop_propagation();
                event.prevent_default();

                // Finally, focus or blur on agent to toggle keyboard
                manipulate_agent(runner_lock.canvas_id(), runner_lock.input.latest_touch_pos);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "touchcancel";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();
            runner_lock.input.is_touch = true;
            push_touches(&mut *runner_lock, egui::TouchPhase::Cancel, &event);
            event.stop_propagation();
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let event_name = "wheel";
        let runner_ref = runner_ref.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            let mut runner_lock = runner_ref.0.lock().unwrap();

            let scroll_multiplier = match event.delta_mode() {
                web_sys::WheelEvent::DOM_DELTA_PAGE => {
                    egui_web::canvas_size_in_points(runner_ref.0.lock().unwrap().canvas_id()).y
                }
                web_sys::WheelEvent::DOM_DELTA_LINE => {
                    8.0 // magic value!
                }
                _ => 1.0,
            };

            let delta = -scroll_multiplier
                * egui::Vec2::new(event.delta_x() as f32, event.delta_y() as f32);

            // Report a zoom event in case CTRL (on Windows or Linux) or CMD (on Mac) is pressed.
            // This if-statement is equivalent to how `Modifiers.command` is determined in
            // `modifiers_from_event()`, but we cannot directly use that fn for a `WheelEvent`.
            if event.ctrl_key() || event.meta_key() {
                runner_lock.input.raw.zoom_delta *= (delta.y / 200.0).exp();
            } else {
                runner_lock.input.raw.scroll_delta += delta;
            }

            runner_lock.needs_repaint.set_true();
            event.stop_propagation();
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn cursor_web_name(cursor: egui::CursorIcon) -> &'static str {
    match cursor {
        egui::CursorIcon::Alias => "alias",
        egui::CursorIcon::AllScroll => "all-scroll",
        egui::CursorIcon::Cell => "cell",
        egui::CursorIcon::ContextMenu => "context-menu",
        egui::CursorIcon::Copy => "copy",
        egui::CursorIcon::Crosshair => "crosshair",
        egui::CursorIcon::Default => "default",
        egui::CursorIcon::Grab => "grab",
        egui::CursorIcon::Grabbing => "grabbing",
        egui::CursorIcon::Help => "help",
        egui::CursorIcon::Move => "move",
        egui::CursorIcon::NoDrop => "no-drop",
        egui::CursorIcon::None => "none",
        egui::CursorIcon::NotAllowed => "not-allowed",
        egui::CursorIcon::PointingHand => "pointer",
        egui::CursorIcon::Progress => "progress",
        egui::CursorIcon::ResizeHorizontal => "ew-resize",
        egui::CursorIcon::ResizeNeSw => "nesw-resize",
        egui::CursorIcon::ResizeNwSe => "nwse-resize",
        egui::CursorIcon::ResizeVertical => "ns-resize",
        egui::CursorIcon::Text => "text",
        egui::CursorIcon::VerticalText => "vertical-text",
        egui::CursorIcon::Wait => "wait",
        egui::CursorIcon::ZoomIn => "zoom-in",
        egui::CursorIcon::ZoomOut => "zoom-out",
    }
}

#[cfg(target_arch = "wasm32")]
fn manipulate_agent(canvas_id: &str, latest_cursor: Option<egui::Pos2>) -> Option<()> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlInputElement;
    let window = web_sys::window()?;
    let document = window.document()?;
    let input: HtmlInputElement = document.get_element_by_id(AGENT_ID)?.dyn_into().unwrap();
    let cutsor_txt = document.body()?.style().get_property_value("cursor").ok()?;
    let style = egui_web::canvas_element(canvas_id)?.style();
    if cutsor_txt == cursor_web_name(egui::CursorIcon::Text) {
        input.set_hidden(false);
        input.focus().ok()?;
        // Panning canvas so that text edit is shown at 30%
        // Only on touch screens, when keyboard popups
        if let Some(p) = latest_cursor {
            let inner_height = window.inner_height().ok()?.as_f64()? as f32;
            let current_rel = p.y / inner_height;

            if current_rel > 0.5 {
                // probably below the keyboard

                let target_rel = 0.3;

                let delta = target_rel - current_rel;
                let new_pos_percent = (delta * 100.0).round().to_string() + "%";

                style.set_property("position", "absolute").ok()?;
                style.set_property("top", &new_pos_percent).ok()?;
            }
        }
    } else {
        input.blur().ok()?;
        input.set_hidden(true);
        style.set_property("position", "absolute").ok()?;
        style.set_property("top", "0%").ok()?; // move back to normal position
    }
    Some(())
}

#[cfg(target_arch = "wasm32")]
const MOBILE_DEVICE: [&str; 6] = ["Android", "iPhone", "iPad", "iPod", "webOS", "BlackBerry"];
/// If context is running under mobile device?
#[cfg(target_arch = "wasm32")]
pub fn is_mobile() -> Option<bool> {
    let user_agent = web_sys::window()?.navigator().user_agent().ok()?;
    let is_mobile = MOBILE_DEVICE.iter().any(|&name| user_agent.contains(name));
    Some(is_mobile)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_mobile() -> Option<bool> {
    Some(false)
}

// Move text agent to text cursor's position, on desktop/laptop,
// candidate window moves following text element (agent),
// so it appears that the IME candidate window moves with text cursor.
// On mobile devices, there is no need to do that.
#[cfg(target_arch = "wasm32")]
fn move_text_cursor(cursor: &Option<egui::Pos2>, canvas_id: &str) -> Option<()> {
    let style = text_agent().style();
    // Note: movint agent on mobile devices will lead to unpredictable scroll.
    if is_mobile() == Some(false) {
        cursor.as_ref().and_then(|&egui::Pos2 { x, y }| {
            let canvas = egui_web::canvas_element(canvas_id)?;
            let y = y + (canvas.scroll_top() + canvas.offset_top()) as f32;
            let x = x + (canvas.scroll_left() + canvas.offset_left()) as f32;
            // Canvas is translated 50% horizontally in html.
            let x = x - canvas.offset_width() as f32 / 2.0;
            style.set_property("position", "absolute").ok()?;
            style.set_property("top", &(y.to_string() + "px")).ok()?;
            style.set_property("left", &(x.to_string() + "px")).ok()
        })
    } else {
        style.set_property("position", "absolute").ok()?;
        style.set_property("top", "0px").ok()?;
        style.set_property("left", "0px").ok()
    }
}

/// Install event listeners to register different input events
/// and start running the given app.
#[cfg(target_arch = "wasm32")]
fn start_web(canvas_id: &str, app: Box<dyn app::App>) -> Result<AppRunnerRef, JsValue> {
    let backend = WebBackend::new(canvas_id)?;
    let mut runner = AppRunner::new(backend, app)?;
    runner.warm_up()?;
    start_runner(runner)
}

/// Install event listeners to register different input events
/// and starts running the given `AppRunner`.
#[cfg(target_arch = "wasm32")]
fn start_runner(app_runner: AppRunner) -> Result<AppRunnerRef, JsValue> {
    let runner_ref = AppRunnerRef(Arc::new(Mutex::new(app_runner)));
    install_canvas_events(&runner_ref)?;
    install_document_events(&runner_ref)?;
    install_text_agent(&runner_ref)?;
    repaint_every_ms(&runner_ref, 1000)?; // just in case. TODO: make it a parameter
    paint_and_schedule(runner_ref.clone())?;
    Ok(runner_ref)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    let app: Box<dyn app::App> = match app::Application::new() {
        Ok(app) => Box::new(app),
        Err(error) => Box::new(app::ErrorApplication::new(error.to_string())),
    };
    start_web(canvas_id, app)?;
    Ok(())
}
