pub mod app;
mod font;
mod framerate;
mod style;
mod theme;
mod timer;
mod widgets;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app: Box<dyn eframe::epi::App> = match app::Application::new() {
        Ok(app) => Box::new(app),
        Err(error) => Box::new(app::ErrorApplication::new(error.to_string())),
    };
    eframe::start_web(canvas_id, app)
}
