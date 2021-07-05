use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::timer::scramble::TimerCube;
use crate::timer::state::TimerState;
use egui::{Pos2, Rect, Ui, Vec2};

const TARGET_CUBE_FRACTION: f32 = 0.75;

pub fn timer_ui(ui: &mut Ui, center: &Pos2, state: &TimerState) {
    // Render timer only in center of screen
    let timer_height = ui.fonts().row_height(FontSize::Timer.into());
    let galley = ui
        .fonts()
        .layout_single_line(FontSize::Timer.into(), state.current_time_string());
    let timer_width = galley.size.x;
    ui.painter().galley(
        Pos2::new(center.x - timer_width / 2.0, center.y - timer_height / 2.0),
        galley,
        state.current_time_color(),
    );
}

pub fn bluetooth_timer_ui(
    ui: &mut Ui,
    rect: &Rect,
    center: &Pos2,
    cube: &TimerCube,
    state: &TimerState,
    cube_rect: &mut Option<Rect>,
    framerate: &mut Framerate,
) {
    // In Bluetooth mode, render cube as well as timer
    let timer_height = ui.fonts().row_height(FontSize::Timer.into());
    let timer_padding = 32.0;
    let cube_height = (rect.height() - timer_height - timer_padding) * TARGET_CUBE_FRACTION;
    let total_height = timer_height + timer_padding + cube_height;

    // Allocate space for the cube rendering. This is 3D so it will be rendered
    // with OpenGL after egui is done painting.
    let y = center.y - total_height / 2.0;
    let computed_cube_rect = Rect::from_min_size(
        Pos2::new(center.x - cube_height / 2.0, y),
        Vec2::new(cube_height, cube_height),
    );
    if computed_cube_rect.width() > 0.0 && computed_cube_rect.height() > 0.0 {
        *cube_rect = Some(computed_cube_rect);
        if cube.animating() {
            framerate.request_max();
        }
    }

    // Draw timer
    let galley = ui
        .fonts()
        .layout_single_line(FontSize::Timer.into(), state.current_time_string());
    let timer_width = galley.size.x;
    ui.painter().galley(
        Pos2::new(
            center.x - timer_width / 2.0,
            y + cube_height + timer_padding,
        ),
        galley,
        state.current_time_color(),
    );
}
