use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::{solve_time_short_string, solve_time_string, CustomWidgets};
use chrono::Local;
use eframe::{
    egui::{
        widgets::Label, CentralPanel, Color32, CtxRef, Key, Layout, Pos2, Rect, Sense, SidePanel,
        Ui, Vec2,
    },
    epi,
};
use instant::Instant;
use std::cmp::Ord;
use tpscube_core::{scramble_3x3x3, History, Move, Penalty, Solve, SolveType};

const MIN_SCRAMBLE_LINES: usize = 2;
const MAX_SCRAMBLE_LINES: usize = 5;

pub enum TimerState {
    Inactive(u32),
    Preparing(Instant, u32),
    Ready,
    Solving(Instant),
}

pub struct CachedSessionSolves {
    update_id: u64,
    solves: Vec<Solve>,
}

pub struct Timer {
    state: TimerState,
    current_scramble: Vec<Move>,
    current_scramble_displayed: bool,
    next_scramble: Option<Vec<Move>>,
    cached_solves: Option<CachedSessionSolves>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            state: TimerState::Inactive(0),
            current_scramble: scramble_3x3x3(),
            current_scramble_displayed: false,
            next_scramble: Some(scramble_3x3x3()),
            cached_solves: None,
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
            TimerState::Inactive(time) => solve_time_string(time),
            TimerState::Preparing(_, _) | TimerState::Ready => solve_time_short_string(0),
            TimerState::Solving(start) => {
                solve_time_short_string((Instant::now() - start).as_millis() as u32)
            }
        }
    }

    fn current_time_color(&self) -> Color32 {
        match self.state {
            TimerState::Inactive(_) | TimerState::Solving(_) => Theme::Content.into(),
            TimerState::Preparing(_, _) => Theme::BackgroundHighlight.into(),
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
            TimerState::Inactive(_) => false,
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

        self.state = TimerState::Inactive(time);
        if let Some(scramble) = &self.next_scramble {
            self.current_scramble = scramble.clone();
        } else {
            self.current_scramble = scramble_3x3x3();
        }
        self.current_scramble_displayed = false;
        self.next_scramble = None;
    }

    fn get_current_session_solves(&mut self, history: &History) -> &Option<CachedSessionSolves> {
        if let Some(session) = history.sessions().get(history.current_session()) {
            // Check for updates
            if let Some(cache) = &self.cached_solves {
                if cache.update_id == session.update_id {
                    // Already cached
                    return &self.cached_solves;
                }
            }

            // Cache solve information
            let mut solves = Vec::new();
            for solve in &session.solves {
                if let Some(solve) = history.solves().get(solve) {
                    solves.push(solve.clone());
                }
            }
            solves.sort_unstable_by(|a, b| a.cmp(&b));
            self.cached_solves = Some(CachedSessionSolves {
                update_id: session.update_id,
                solves,
            });
        } else {
            // New session, invalidate cache
            self.cached_solves = None;
        }
        &self.cached_solves
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        framerate: &Framerate,
    ) {
        // Generate a scramble when the current one is onscreen. The slight delay will
        // not be noticed as much when performing a new scramble.
        if self.current_scramble_displayed && self.next_scramble.is_none() {
            self.next_scramble = Some(scramble_3x3x3());
        }

        ctxt.set_visuals(side_visuals());
        SidePanel::left("timer", 160.0).show(ctxt, |ui| {
            ui.section("Session");

            ui.vertical(|ui| {
                Self::session_time(ui, "Last ao5", Some(43874));
                Self::session_time(ui, "Last ao12", Some(48235));
                Self::session_time(ui, "Session avg", Some(47126));
                Self::session_time(ui, "Best solve", Some(38742));
                Self::session_time(ui, "Best ao5", Some(42983));
                Self::session_time(ui, "Best ao12", Some(48239));
            });

            ui.add_space(8.0);
            ui.section("Solves");

            let mut has_solves = false;
            if let Some(cache) = self.get_current_session_solves(history) {
                for (idx, solve) in cache.solves.iter().enumerate().rev() {
                    ui.solve("timer", idx, solve, history);
                    has_solves = true;
                }
            }
            if !has_solves {
                let color: Color32 = Theme::Disabled.into();
                ui.add(Label::new("No solves in this session").text_color(color));
            }
        });

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            let rect = ui.max_rect();
            let center = rect.center();

            let id = ui.make_persistent_id("timer_input");
            let interact = ui.interact(rect, id, Sense::click());
            ui.memory().request_focus(id);

            // Check for user input to interact with the timer
            match self.state {
                TimerState::Inactive(time) => {
                    if ctxt.input().keys_down.contains(&Key::Space)
                        || interact.is_pointer_button_down_on()
                    {
                        self.state = TimerState::Preparing(Instant::now(), time);
                    }
                }
                TimerState::Preparing(start, time) => {
                    if ctxt.input().keys_down.len() == 0 && !interact.is_pointer_button_down_on() {
                        self.state = TimerState::Inactive(time);
                    } else if (Instant::now() - start).as_millis() > 500 {
                        self.state = TimerState::Ready;
                    }
                }
                TimerState::Ready => {
                    if ctxt.input().keys_down.len() == 0 && !interact.is_pointer_button_down_on() {
                        self.state = TimerState::Solving(Instant::now());
                    }
                }
                TimerState::Solving(start) => {
                    if ctxt.input().keys_down.len() != 0 || interact.is_pointer_button_down_on() {
                        self.finish_solve((Instant::now() - start).as_millis() as u32, history);
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
                let scramble_padding = 8.0;
                let timer_padding = 40.0;

                let scramble = Self::fit_scramble(ui, &self.current_scramble, rect.width());
                let scramble_line_height = ui.fonts().row_height(FontSize::Scramble.into());
                let scramble_height = scramble_line_height * scramble.len() as f32;

                let timer_height = ui.fonts().row_height(FontSize::Timer.into());
                let timer_overlap = timer_height * 0.4;

                let cube_height = rect.height()
                    - (scramble_padding + scramble_height + timer_height + timer_padding
                        - timer_overlap);

                // Render scramble
                let mut y = rect.top() + scramble_padding;
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

                // Allocate space for the cube rendering
                let cube_rect = Rect::from_min_size(
                    Pos2::new(center.x - cube_height / 2.0, y),
                    Vec2::new(cube_height, cube_height),
                );
                let galley = ui.fonts().layout_single_line(
                    FontSize::Small.into(),
                    "[Cube rendering goes here]".into(),
                );
                ui.painter().galley(
                    Pos2::new(
                        cube_rect.center().x - galley.size.x / 2.0,
                        cube_rect.center().y - galley.size.y / 2.0,
                    ),
                    galley,
                    Theme::Disabled.into(),
                );

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
        });

        // Run at 10 FPS when solving (to update counting timer), or only when
        // updates occur otherwise
        framerate.set_target(match self.state {
            TimerState::Solving(_) => Some(10),
            _ => None,
        });
    }
}
