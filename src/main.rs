mod algorithms;
mod app;
mod center_generated;
mod corner_generated;
mod cube;
mod details;
mod edge_generated;
mod font;
mod framerate;
mod future;
mod gl;
mod graph;
mod history;
mod mode;
mod settings;
mod style;
mod theme;
mod timer;
mod widgets;

#[cfg(not(target_arch = "wasm32"))]
mod bluetooth;

// This code is from backend.rs in egui_glium, but modified to allow for rendering
// of 3D elements.

use egui_glium::{window_settings::WindowSettings, *};
use gl::GlContext;
use glium::glutin;
#[cfg(target_os = "windows")]
use glium::glutin::platform::windows::WindowBuilderExtWindows;
use std::time::Instant;

struct RequestRepaintEvent;

struct GliumRepaintSignal(
    std::sync::Mutex<glutin::event_loop::EventLoopProxy<RequestRepaintEvent>>,
);

impl epi::RepaintSignal for GliumRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(RequestRepaintEvent).ok();
    }
}

#[cfg(target_os = "windows")]
fn window_builder_drag_and_drop(
    window_builder: glutin::window::WindowBuilder,
    enable: bool,
) -> glutin::window::WindowBuilder {
    window_builder.with_drag_and_drop(enable)
}

#[cfg(not(target_os = "windows"))]
fn window_builder_drag_and_drop(
    window_builder: glutin::window::WindowBuilder,
    _enable: bool,
) -> glutin::window::WindowBuilder {
    // drag and drop can only be disabled on windows
    window_builder
}

fn create_display(
    app: &dyn app::App,
    native_options: &epi::NativeOptions,
    window_settings: Option<WindowSettings>,
    window_icon: Option<glutin::window::Icon>,
    event_loop: &glutin::event_loop::EventLoop<RequestRepaintEvent>,
) -> glium::Display {
    let mut window_builder = glutin::window::WindowBuilder::new()
        .with_always_on_top(native_options.always_on_top)
        .with_decorations(native_options.decorated)
        .with_resizable(native_options.resizable)
        .with_title(app.name())
        .with_transparent(native_options.transparent)
        .with_window_icon(window_icon);

    window_builder =
        window_builder_drag_and_drop(window_builder, native_options.drag_and_drop_support);

    let initial_size_points = native_options.initial_window_size;

    if let Some(window_settings) = &window_settings {
        window_builder = window_settings.initialize_size(window_builder);
    } else if let Some(initial_size_points) = initial_size_points {
        window_builder = window_builder.with_inner_size(glutin::dpi::LogicalSize {
            width: initial_size_points.x as f64,
            height: initial_size_points.y as f64,
        });
    }

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_srgb(true)
        .with_stencil_buffer(8)
        .with_multisampling(2)
        .with_vsync(true);

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    if let Some(window_settings) = &window_settings {
        window_settings.restore_positions(&display);
    }

    display
}

fn integration_info(
    display: &glium::Display,
    previous_frame_time: Option<f32>,
) -> epi::IntegrationInfo {
    epi::IntegrationInfo {
        web_info: None,
        prefer_dark_mode: Some(true),
        cpu_usage: previous_frame_time,
        seconds_since_midnight: seconds_since_midnight(),
        native_pixels_per_point: Some(native_pixels_per_point(&display)),
    }
}

fn load_icon(icon_data: epi::IconData) -> Option<glutin::window::Icon> {
    glutin::window::Icon::from_rgba(icon_data.rgba, icon_data.width, icon_data.height).ok()
}

pub fn is_mobile() -> Option<bool> {
    Some(false)
}

// ----------------------------------------------------------------------------

/// Run an egui app
pub fn run(
    mut app: Box<dyn app::App>,
    nativve_options: epi::NativeOptions,
    video_subsystem: sdl2::VideoSubsystem,
) -> ! {
    let mut screensaver_enabled = false;
    let window_settings = None;
    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let icon = nativve_options.icon_data.clone().and_then(load_icon);
    let display = create_display(&*app, &nativve_options, window_settings, icon, &event_loop);

    let repaint_signal = std::sync::Arc::new(GliumRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    let mut egui = EguiGlium::new(&display);
    *egui.ctx().memory() = Default::default();

    {
        let (ctx, _painter) = egui.ctx_and_painter_mut();
        app.setup(&ctx);
    }

    let mut previous_frame_time = None;

    let mut is_focused = true;

    if app.warm_up_enabled() {
        let saved_memory = egui.ctx().memory().clone();
        egui.ctx().memory().set_everything_is_visible(true);

        egui.begin_frame(&display);
        let (ctx, painter) = egui.ctx_and_painter_mut();
        let mut app_output = epi::backend::AppOutput::default();
        let mut frame = epi::backend::FrameBuilder {
            info: integration_info(&display, None),
            tex_allocator: painter,
            #[cfg(feature = "http")]
            http: http.clone(),
            output: &mut app_output,
            repaint_signal: repaint_signal.clone(),
        }
        .build();

        app.update(&ctx, &mut frame);

        let _ = egui.end_frame(&display);

        *egui.ctx().memory() = saved_memory; // We don't want to remember that windows were huge.
        egui.ctx().clear_animations();

        // TODO: handle app_output
        // eprintln!("Warmed up in {} ms", warm_up_start.elapsed().as_millis())
    }

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            if !is_focused {
                // On Mac, a minimized Window uses up all CPU: https://github.com/emilk/egui/issues/325
                // We can't know if we are minimized: https://github.com/rust-windowing/winit/issues/208
                // But we know if we are focused (in foreground). When minimized, we are not focused.
                // However, a user may want an egui with an animation in the background,
                // so we still need to repaint quite fast.
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            let frame_start = std::time::Instant::now();

            egui.begin_frame(&display);
            let (ctx, painter) = egui.ctx_and_painter_mut();
            let mut app_output = epi::backend::AppOutput::default();
            let mut frame = epi::backend::FrameBuilder {
                info: integration_info(&display, previous_frame_time),
                tex_allocator: painter,
                output: &mut app_output,
                repaint_signal: repaint_signal.clone(),
            }
            .build();
            app.update(ctx, &mut frame);
            let (needs_repaint, shapes) = egui.end_frame(&display);

            let frame_time = (Instant::now() - frame_start).as_secs_f64() as f32;
            previous_frame_time = Some(frame_time);

            {
                use glium::Surface as _;
                let mut target = display.draw();
                let clear_color = app.clear_color();
                target.clear_color(
                    clear_color[0],
                    clear_color[1],
                    clear_color[2],
                    clear_color[3],
                );
                egui.paint(&display, &mut target, shapes);

                let (ctx, _painter) = egui.ctx_and_painter_mut();
                let mut gl = GlContext {
                    display: &display,
                    target: &mut target,
                };
                app.update_gl(ctx, &mut gl);

                target.finish().unwrap();
            }

            let desired_screensaver_enabled = app.screensaver_enabled();
            if screensaver_enabled != desired_screensaver_enabled {
                screensaver_enabled = desired_screensaver_enabled;
                if screensaver_enabled {
                    video_subsystem.enable_screen_saver();
                } else {
                    video_subsystem.disable_screen_saver();
                }
            }

            {
                let epi::backend::AppOutput { quit, window_size } = app_output;

                if let Some(window_size) = window_size {
                    display.gl_window().window().set_inner_size(
                        glutin::dpi::PhysicalSize {
                            width: (egui.ctx().pixels_per_point() * window_size.x).round(),
                            height: (egui.ctx().pixels_per_point() * window_size.y).round(),
                        }
                        .to_logical::<f32>(native_pixels_per_point(&display) as f64),
                    );
                }

                *control_flow = if quit {
                    glutin::event_loop::ControlFlow::Exit
                } else if needs_repaint {
                    display.gl_window().window().request_redraw();
                    glutin::event_loop::ControlFlow::Poll
                } else {
                    glutin::event_loop::ControlFlow::Wait
                };
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                if egui.is_quit_event(&event) {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Focused(new_focused) = event {
                    is_focused = new_focused;
                }

                egui.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                app.on_exit();
            }

            glutin::event::Event::UserEvent(RequestRepaintEvent) => {
                display.gl_window().window().request_redraw();
            }

            _ => (),
        }
    });
}

#[tokio::main]
async fn main() {
    // Initialize SDL2 just for stopping the screensaver. There are no other crates for this and
    // its a giant pile of platform dependent code.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let app: Box<dyn app::App> = match app::Application::new() {
        Ok(app) => Box::new(app),
        Err(error) => Box::new(app::ErrorApplication::new(error.to_string())),
    };
    let native_options = epi::NativeOptions::default();

    run(app, native_options, video_subsystem);
}
