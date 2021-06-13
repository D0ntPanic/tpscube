mod app;
mod font;
mod framerate;
mod style;
mod theme;
mod timer;
mod widgets;

pub fn main() {
    let app: Box<dyn eframe::epi::App> = match app::Application::new() {
        Ok(app) => Box::new(app),
        Err(error) => Box::new(app::ErrorApplication::new(error.to_string())),
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(app, native_options);
}
