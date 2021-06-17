use crate::cube::CubeRenderer;
use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::{solve_time_short_string, solve_time_string, CustomWidgets};
use anyhow::Result;
use chrono::Local;
use egui::{
    containers::ScrollArea, widgets::Label, CentralPanel, Color32, CtxRef, Key, Layout, Pos2, Rect,
    Sense, SidePanel, Stroke, Ui, Vec2,
};
use instant::Instant;
use std::cmp::Ord;
use tpscube_core::{
    scramble_3x3x3, Cube, Cube3x3x3, History, Move, Penalty, Solve, SolveList, SolveType,
};

const MIN_SCRAMBLE_LINES: usize = 2;
const MAX_SCRAMBLE_LINES: usize = 5;

const TARGET_SCRAMBLE_FRACTION: f32 = 0.2;
const TARGET_TIMER_FRACTION: f32 = 0.2;

pub enum TimerState {
    Inactive(u32),
    Preparing(Instant, u32),
    Ready,
    Solving(Instant),
    SolveComplete(u32),
}

pub struct CachedSessionSolves {
    update_id: Option<u64>,
    solves: Vec<Solve>,
    last_ao5: Option<u32>,
    last_ao12: Option<u32>,
    session_avg: Option<u32>,
    best_solve: Option<u32>,
    best_ao5: Option<u32>,
    best_ao12: Option<u32>,
}

pub struct Timer {
    state: TimerState,
    current_scramble: Vec<Move>,
    current_scramble_displayed: bool,
    next_scramble: Option<Vec<Move>>,
    session_solves: CachedSessionSolves,
    cube: CubeRenderer,
}

impl CachedSessionSolves {
    fn new(update_id: Option<u64>, solves: Vec<Solve>) -> Self {
        let last_ao5 = solves.as_slice().last_average(5);
        let last_ao12 = solves.as_slice().last_average(12);
        let session_avg = solves.as_slice().average();
        let best_solve = solves.as_slice().best();
        let best_ao5 = solves.as_slice().best_average(5);
        let best_ao12 = solves.as_slice().best_average(12);

        Self {
            update_id,
            solves,
            last_ao5,
            last_ao12,
            session_avg,
            best_solve,
            best_ao5,
            best_ao12,
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        let current_scramble = scramble_3x3x3();
        let mut cube_state = Cube3x3x3::new();
        cube_state.do_moves(&current_scramble);
        let mut cube = CubeRenderer::new();
        cube.set_cube_state(cube_state);

        Self {
            state: TimerState::Inactive(0),
            current_scramble,
            current_scramble_displayed: false,
            next_scramble: Some(scramble_3x3x3()),
            session_solves: CachedSessionSolves::new(None, Vec::new()),
            cube,
        }
    }

    fn session_time(ui: &mut Ui, name: &str, time: Option<u32>) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", name));
            ui.with_layout(Layout::right_to_left(), |ui| {
                if let Some(time) = time {
                    ui.solve_time(time);
                } else {
                    ui.label("-");
                }
            })
        });
    }

    fn current_time_string(&self) -> String {
        match self.state {
            TimerState::Inactive(time) | TimerState::SolveComplete(time) => solve_time_string(time),
            TimerState::Preparing(_, time) => {
                if self.is_solving() {
                    solve_time_short_string(0)
                } else {
                    solve_time_string(time)
                }
            }
            TimerState::Ready => solve_time_short_string(0),
            TimerState::Solving(start) => {
                solve_time_short_string((Instant::now() - start).as_millis() as u32)
            }
        }
    }

    fn current_time_color(&self) -> Color32 {
        match self.state {
            TimerState::Inactive(_) | TimerState::Solving(_) | TimerState::SolveComplete(_) => {
                Theme::Content.into()
            }
            TimerState::Preparing(_, _) => {
                if self.is_solving() {
                    Theme::BackgroundHighlight.into()
                } else {
                    Theme::Content.into()
                }
            }
            TimerState::Ready => Theme::Green.into(),
        }
    }

    fn scramble_lines(scramble: &[Move], line_count: usize) -> Vec<String> {
        let per_line = (scramble.len() + line_count - 1) / line_count;
        let mut lines = Vec::new();
        for chunks in scramble.chunks(per_line) {
            let moves: Vec<String> = chunks.iter().map(|mv| mv.to_string()).collect();
            lines.push(moves.join("  "));
        }
        lines
    }

    fn fit_scramble(ui: &Ui, scramble: &[Move], width: f32) -> Vec<String> {
        for line_count in MIN_SCRAMBLE_LINES..MAX_SCRAMBLE_LINES {
            let lines = Self::scramble_lines(scramble, line_count);
            if !lines.iter().any(|line| {
                ui.fonts()
                    .layout_single_line(FontSize::Scramble.into(), line.into())
                    .size
                    .x
                    > width
            }) {
                return lines;
            }
        }
        Self::scramble_lines(scramble, MAX_SCRAMBLE_LINES)
    }

    fn is_solving(&self) -> bool {
        match self.state {
            TimerState::Inactive(_) | TimerState::SolveComplete(_) => false,
            TimerState::Preparing(start, _) => (Instant::now() - start).as_millis() > 10,
            _ => true,
        }
    }

    fn finish_solve(&mut self, time: u32, history: &mut History) {
        history.new_solve(Solve {
            id: Solve::new_id(),
            solve_type: SolveType::Standard3x3x3,
            session: history.current_session().into(),
            scramble: self.current_scramble.clone(),
            created: Local::now(),
            time,
            penalty: Penalty::None,
            device: None,
            moves: None,
        });
        let _ = history.local_commit();

        self.state = TimerState::SolveComplete(time);
        if let Some(scramble) = &self.next_scramble {
            self.current_scramble = scramble.clone();
        } else {
            self.current_scramble = scramble_3x3x3();
        }
        self.current_scramble_displayed = false;
        self.next_scramble = None;

        let mut cube_state = Cube3x3x3::new();
        cube_state.do_moves(&self.current_scramble);
        self.cube.set_cube_state(cube_state);
        self.cube.reset_angle();
    }

    fn update_solve_cache(&mut self, history: &History) {
        if let Some(session) = history.sessions().get(history.current_session()) {
            // Check for updates
            if let Some(update_id) = self.session_solves.update_id {
                if update_id == session.update_id {
                    // Already cached and up to date
                    return;
                }
            }

            // Cache solve information
            let mut solves = Vec::new();
            for solve in &session.solves {
                if let Some(solve) = history.solves().get(solve) {
                    solves.push(solve.clone());
                }
            }

            // Sort solves by time, then by ID. There cannot be any equal solves so it
            // is fine to use the faster unstable sort here.
            solves.sort_unstable_by(|a, b| a.cmp(&b));
            self.session_solves = CachedSessionSolves::new(Some(session.update_id), solves);
        } else {
            // New session, invalidate cache
            self.session_solves = CachedSessionSolves::new(None, Vec::new());
        }
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        framerate: &Framerate,
        cube_rect: &mut Option<Rect>,
    ) {
        // Generate a scramble when the current one is onscreen. The slight delay will
        // not be noticed as much when performing a new scramble.
        if self.current_scramble_displayed && self.next_scramble.is_none() {
            self.next_scramble = Some(scramble_3x3x3());
        }

        ctxt.set_visuals(side_visuals());
        SidePanel::left("timer", 160.0).show(ctxt, |ui| {
            ui.section("Session");

            self.update_solve_cache(history);

            ui.vertical(|ui| {
                Self::session_time(ui, "Last ao5", self.session_solves.last_ao5);
                Self::session_time(ui, "Last ao12", self.session_solves.last_ao12);
                Self::session_time(ui, "Session avg", self.session_solves.session_avg);
                Self::session_time(ui, "Best solve", self.session_solves.best_solve);
                Self::session_time(ui, "Best ao5", self.session_solves.best_ao5);
                Self::session_time(ui, "Best ao12", self.session_solves.best_ao12);

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Red.into(),
                    };
                    ui.style_mut().visuals.widgets.active.fg_stroke = Stroke {
                        width: 1.0,
                        color: Theme::Red.into(),
                    };
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        if ui
                            .add(Label::new("↺  New session").sense(Sense::click()))
                            .clicked()
                        {
                            let _ = history.new_session();
                        }
                    })
                });
            });

            ui.add_space(8.0);
            ui.section("Solves");

            ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
            ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
            ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
            ScrollArea::auto_sized()
                .id_source("timer_solve_list")
                .show(ui, |ui| {
                    let mut has_solves = false;
                    for (idx, solve) in self.session_solves.solves.iter().enumerate().rev() {
                        ui.solve("timer", idx, solve, history);
                        has_solves = true;
                    }
                    if !has_solves {
                        let color: Color32 = Theme::Disabled.into();
                        ui.add(Label::new("No solves in this session").text_color(color));
                    }
                });
        });

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            let rect = ui.max_rect();
            let center = rect.center();

            let id = ui.make_persistent_id("timer_input");
            let interact = ui.interact(rect, id, Sense::click_and_drag());
            ui.memory().request_focus(id);

            // Check for user input to interact with the timer
            let touching = crate::is_mobile() == Some(true)
                && (interact.is_pointer_button_down_on() || interact.dragged());
            match self.state {
                TimerState::Inactive(time) => {
                    if ctxt.input().keys_down.contains(&Key::Space) || touching {
                        self.state = TimerState::Preparing(Instant::now(), time);
                    }
                }
                TimerState::Preparing(start, time) => {
                    if ctxt.input().keys_down.len() == 0 && !touching {
                        self.state = TimerState::Inactive(time);
                    } else if (Instant::now() - start).as_millis() > 300 {
                        self.state = TimerState::Ready;
                    }
                }
                TimerState::Ready => {
                    if ctxt.input().keys_down.len() == 0 && !touching {
                        self.state = TimerState::Solving(Instant::now());
                    }
                }
                TimerState::Solving(start) => {
                    if ctxt.input().keys_down.len() != 0 || touching {
                        self.finish_solve((Instant::now() - start).as_millis() as u32, history);
                    }
                }
                TimerState::SolveComplete(time) => {
                    if ctxt.input().keys_down.len() == 0 && !touching {
                        self.state = TimerState::Inactive(time)
                    }
                }
            }

            if self.is_solving() {
                // Render timer only in center of screen
                let timer_height = ui.fonts().row_height(FontSize::Timer.into());
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Timer.into(), self.current_time_string());
                let timer_width = galley.size.x;
                ui.painter().galley(
                    Pos2::new(center.x - timer_width / 2.0, center.y - timer_height / 2.0),
                    galley,
                    self.current_time_color(),
                );
            } else {
                // Compute sizes of components in the main view
                let target_scramble_height = rect.height() * TARGET_SCRAMBLE_FRACTION;
                let target_timer_height = rect.height() * TARGET_TIMER_FRACTION;

                let scramble_padding = 8.0;
                let timer_padding = 40.0;

                let scramble = Self::fit_scramble(ui, &self.current_scramble, rect.width());
                let scramble_line_height = ui.fonts().row_height(FontSize::Scramble.into());
                let min_scramble_height = scramble_line_height * scramble.len() as f32;
                let scramble_height = min_scramble_height.max(target_scramble_height);

                let min_timer_height = ui.fonts().row_height(FontSize::Timer.into());
                let timer_overlap = min_timer_height * 0.4;
                let timer_height = min_timer_height.max(target_timer_height);

                let cube_height = rect.height()
                    - (scramble_padding + scramble_height + timer_height + timer_padding
                        - timer_overlap);

                // Render scramble
                let mut y =
                    rect.top() + scramble_padding + (scramble_height - min_scramble_height) / 2.0;
                for line in scramble {
                    let galley = ui
                        .fonts()
                        .layout_single_line(FontSize::Scramble.into(), line);
                    let line_width = galley.size.x;
                    ui.painter().galley(
                        Pos2::new(center.x - line_width / 2.0, y),
                        galley,
                        Theme::Blue.into(),
                    );
                    y += scramble_line_height;
                }
                self.current_scramble_displayed = true;

                // Allocate space for the cube rendering. This is 3D so it will be rendered
                // with OpenGL after egui is done painting.
                let computed_cube_rect = Rect::from_min_size(
                    Pos2::new(center.x - cube_height / 2.0, y),
                    Vec2::new(cube_height, cube_height),
                );
                if computed_cube_rect.width() > 0.0 && computed_cube_rect.height() > 0.0 {
                    *cube_rect = Some(computed_cube_rect);
                }

                // Render timer
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Timer.into(), self.current_time_string());
                let timer_width = galley.size.x;
                ui.painter().galley(
                    Pos2::new(
                        center.x - timer_width / 2.0,
                        rect.bottom() - timer_height - timer_padding,
                    ),
                    galley,
                    self.current_time_color(),
                );
            }

            if cube_rect.is_some() && ui.rect_contains_pointer(cube_rect.unwrap()) {
                let scroll_delta = ctxt.input().scroll_delta;
                self.cube
                    .adjust_angle(scroll_delta.x / 3.0, scroll_delta.y / 3.0);
            }
            if crate::is_mobile() != Some(true) && interact.dragged() {
                self.cube.adjust_angle(
                    ui.input().pointer.delta().x / 3.0,
                    ui.input().pointer.delta().y / 3.0,
                );
            }
        });

        // Run at 10 FPS when solving (to update counting timer), or only when
        // updates occur otherwise
        framerate.set_target(match self.state {
            TimerState::Preparing(_, _) | TimerState::Solving(_) => Some(10),
            _ => None,
        });
    }

    pub fn paint_cube(
        &mut self,
        ctxt: &CtxRef,
        gl: &mut GlContext<'_, '_>,
        rect: &Rect,
    ) -> Result<()> {
        self.cube.draw(ctxt, gl, rect)
    }
}
