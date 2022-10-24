mod analysis;
mod scramble;
mod session;
mod solve;
mod state;

use crate::app::SolveDetails;
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::settings::Settings;
use crate::style::{content_visuals, side_visuals};
use anyhow::Result;
use chrono::Local;
use egui::{Align, CentralPanel, CtxRef, Event, Key, Layout, Rect, Response, Sense, Ui, Vec2};
use instant::Instant;
use scramble::TimerCube;
use session::TimerSession;
use solve::{bluetooth_timer_ui, timer_ui};
use state::{LastSolve, TimerState};
use tpscube_core::{
    Analysis, Cube, Cube3x3x3, CubeWithSolution, History, InitialCubeState, PartialAnalysis,
    Penalty, Solve, SolveType, TimedMove,
};

pub struct TimerWidget {
    state: TimerState,
    session: TimerSession,
    cube: TimerCube,
}

pub enum BluetoothEvent {
    Move(TimedMove),
    HandsOnTimer,
    TimerStartCancel,
    TimerReady,
    TimerStarted,
    TimerFinished(u32),
}

impl TimerWidget {
    pub fn new() -> Self {
        Self {
            state: TimerState::Inactive(0, None),
            cube: TimerCube::new(),
            session: TimerSession::new(),
        }
    }

    pub fn is_solving(&self) -> bool {
        self.state.is_solving()
    }

    fn finish_solve(&mut self, time: u32, history: &mut History, solve_type: SolveType) {
        let id = Solve::new_id();
        history.new_solve(Solve {
            id: id.clone(),
            solve_type,
            session: history.current_session().into(),
            scramble: self.cube.scramble().to_vec(),
            created: Local::now(),
            time,
            penalty: Penalty::None,
            device: None,
            moves: None,
        });
        let _ = history.local_commit();
        self.state = TimerState::SolveComplete(
            time,
            LastSolve {
                id,
                solve_type,
                analysis: None,
                scramble: self.cube.scramble().to_vec(),
                penalty: Penalty::None,
            },
        );
        self.cube.new_scramble();
    }

    fn finish_bluetooth_solve(
        &mut self,
        history: &mut History,
        moves: Vec<TimedMove>,
        name: Option<String>,
        solve_type: SolveType,
    ) {
        // Sanity check move data and modify move timing to be relative to the start
        // instead of relative to the prior move
        let mut cube = Cube3x3x3::new();
        cube.do_moves(self.cube.scramble());
        let initial_state = cube.clone();
        let mut final_moves = Vec::new();
        let mut time = 0;
        for mv in &moves {
            cube.do_move(mv.move_());
            time += mv.time();
            final_moves.push(TimedMove::new(mv.move_(), time));
        }
        let (moves, analysis) = if cube.is_solved() {
            let analysis = Analysis::analyze(&CubeWithSolution {
                initial_state,
                solution: final_moves.clone(),
            });
            (Some(final_moves), Some(analysis))
        } else {
            (None, None)
        };

        let id = Solve::new_id();
        history.new_solve(Solve {
            id: id.clone(),
            solve_type,
            session: history.current_session().into(),
            scramble: self.cube.scramble().to_vec(),
            created: Local::now(),
            time,
            penalty: Penalty::None,
            device: name,
            moves,
        });
        let _ = history.local_commit();
        self.state = TimerState::SolveComplete(
            time,
            LastSolve {
                id,
                solve_type,
                analysis,
                scramble: self.cube.scramble().to_vec(),
                penalty: Penalty::None,
            },
        );
        self.cube.new_scramble();
    }

    fn abort_solve(&mut self, time: u32, history: &mut History, solve_type: SolveType) {
        let (penalty, min_time) = if solve_type.is_last_layer_training() {
            (Penalty::ExecutionDNF, 500)
        } else {
            (Penalty::DNF, 2000)
        };

        if time >= min_time {
            // If some solve progress was made, add a DNF. Otherwise,
            // treat it as an accidental start.
            let id = Solve::new_id();
            history.new_solve(Solve {
                id: Solve::new_id(),
                solve_type,
                session: history.current_session().into(),
                scramble: self.cube.scramble().to_vec(),
                created: Local::now(),
                time,
                penalty,
                device: None,
                moves: None,
            });
            let _ = history.local_commit();
            self.state = TimerState::SolveComplete(
                time,
                LastSolve {
                    id,
                    solve_type,
                    analysis: None,
                    scramble: self.cube.scramble().to_vec(),
                    penalty,
                },
            );
            self.cube.new_scramble();
        } else {
            self.state = TimerState::Inactive(0, None);
        }
    }

    fn check_for_interaction_and_state_transition(
        &mut self,
        ctxt: &CtxRef,
        ui: &mut Ui,
        rect: &Rect,
        history: &mut History,
        bluetooth_events: Vec<BluetoothEvent>,
        bluetooth_name: Option<String>,
        accept_keyboard: bool,
        solve_type: SolveType,
    ) -> Response {
        let id = ui.make_persistent_id("timer_input");

        // Don't allow interaction at the very bottom of the screen. This is to avoid false
        // starts when closing the app on an iPhone.
        let interact_rect = Rect::from_min_size(
            rect.left_top(),
            Vec2::new(rect.width(), rect.height() - 96.0),
        );

        let interact = ui.interact(interact_rect, id, Sense::click_and_drag());
        ui.memory().request_focus(id);

        // Check for Bluetooth timer events
        for event in &bluetooth_events {
            match event {
                BluetoothEvent::HandsOnTimer => {
                    match self.state.clone() {
                        TimerState::Inactive(time, last_solve) => {
                            self.state = TimerState::ExternalTimerPreparing(time, last_solve);
                        }
                        _ => self.state = TimerState::ExternalTimerPreparing(0, None),
                    }
                    ctxt.request_repaint();
                }
                BluetoothEvent::TimerStartCancel => {
                    match self.state.clone() {
                        TimerState::ExternalTimerPreparing(time, last_solve) => {
                            self.state = TimerState::Inactive(time, last_solve);
                        }
                        _ => {
                            self.state = TimerState::Inactive(0, None);
                        }
                    }
                    ctxt.request_repaint();
                }
                BluetoothEvent::TimerReady => {
                    self.state = TimerState::ExternalTimerReady;
                    ctxt.request_repaint();
                }
                BluetoothEvent::TimerStarted => {
                    self.state = TimerState::ExternalTimerSolving(Instant::now());
                    ctxt.request_repaint();
                }
                BluetoothEvent::TimerFinished(time) => {
                    self.finish_solve(*time, history, solve_type);
                    ctxt.request_repaint();
                }
                _ => (),
            }
        }

        // Check for user input to interact with the timer
        let touching = crate::is_mobile() == Some(true)
            && (interact.is_pointer_button_down_on() || interact.dragged());
        match self.state.clone() {
            TimerState::Inactive(time, last_solve) => {
                if accept_keyboard && (ctxt.input().keys_down.contains(&Key::Space) || touching) {
                    self.state = TimerState::Preparing(Instant::now(), time, last_solve);
                } else if self.cube.is_bluetooth_active() {
                    if self
                        .cube
                        .update_bluetooth_scramble_and_check_finish(&bluetooth_events)
                    {
                        if solve_type == SolveType::Blind3x3x3 {
                            // When solving in blind mode, start timer immediately when the
                            // scramble is complete so that memorization time is accounted for.
                            self.state = TimerState::BluetoothSolving(
                                Instant::now(),
                                Vec::new(),
                                PartialAnalysis::Unsuccessful,
                            );
                        } else {
                            self.state =
                                TimerState::BluetoothPreparing(Instant::now(), time, last_solve);
                        }
                    }
                } else if accept_keyboard {
                    for event in &ctxt.input().events {
                        if let Event::Text(text) = event {
                            if text.len() == 1 {
                                let ch = text.chars().next().unwrap();
                                if let Some(digit) = ch.to_digit(10) {
                                    self.state.update_for_numerical_input(digit);
                                }
                            }
                        }
                    }
                }
            }
            TimerState::Preparing(start, time, last_solve) => {
                if ctxt.input().keys_down.len() == 0 && !touching {
                    self.state = TimerState::Inactive(time, last_solve);
                } else if (Instant::now() - start).as_millis() > 300 {
                    self.state = TimerState::Ready;
                }
            }
            TimerState::BluetoothPreparing(start, time, last_solve) => {
                if self.cube.is_bluetooth_active() {
                    if bluetooth_events.len() != 0 {
                        // For the first second after finishing a Bluetooth scramble,
                        // cause any extra moves to transition to the fix bad
                        // scramble state. This means that extra accidental turns
                        // at the end of a scramble will not cause the timer to
                        // start before the user is ready.
                        if !self
                            .cube
                            .update_bluetooth_scramble_and_check_finish(&bluetooth_events)
                        {
                            self.state = TimerState::Inactive(time, last_solve);
                        }
                    } else if (Instant::now() - start).as_millis() >= 1000 {
                        self.state = TimerState::BluetoothReady;
                    }
                } else {
                    self.state = TimerState::Inactive(time, last_solve);
                }
            }
            TimerState::ExternalTimerPreparing(time, last_solve) => {
                if bluetooth_name.is_none() {
                    self.state = TimerState::Inactive(time, last_solve);
                }
            }
            TimerState::Ready => {
                if ctxt.input().keys_down.len() == 0 && !touching {
                    self.state = TimerState::Solving(Instant::now());
                }
            }
            TimerState::BluetoothReady => {
                let mut bluetooth_moves = Vec::new();
                for event in bluetooth_events {
                    if let BluetoothEvent::Move(mv) = event {
                        bluetooth_moves.push(mv);
                    }
                }

                if bluetooth_moves.len() != 0 {
                    // Rewrite first move timing data to be at start
                    let first_move = TimedMove::new(bluetooth_moves[0].move_(), 0);
                    bluetooth_moves[0] = first_move;

                    // Start solving and keep track of moves
                    self.state = TimerState::BluetoothSolving(
                        Instant::now(),
                        bluetooth_moves,
                        PartialAnalysis::Unsuccessful,
                    );
                }
            }
            TimerState::ExternalTimerReady => {
                if bluetooth_name.is_none() {
                    self.state = TimerState::Inactive(0, None);
                }
            }
            TimerState::Solving(start) => {
                if ctxt.input().keys_down.contains(&Key::Escape) {
                    self.abort_solve(
                        (Instant::now() - start).as_millis() as u32,
                        history,
                        solve_type,
                    );
                    ctxt.request_repaint();
                } else if ctxt.input().keys_down.len() != 0 || touching {
                    self.finish_solve(
                        (Instant::now() - start).as_millis() as u32,
                        history,
                        solve_type,
                    );
                    ctxt.request_repaint();
                }
            }
            TimerState::BluetoothSolving(start, moves, _) => {
                let mut bluetooth_moves = Vec::new();
                for event in bluetooth_events {
                    if let BluetoothEvent::Move(mv) = event {
                        bluetooth_moves.push(mv);
                    }
                }

                if ctxt.input().keys_down.contains(&Key::Escape) {
                    self.abort_solve(
                        (Instant::now() - start).as_millis() as u32,
                        history,
                        solve_type,
                    );
                    ctxt.request_repaint();
                } else if ctxt.input().keys_down.len() != 0 || touching {
                    self.finish_solve(
                        (Instant::now() - start).as_millis() as u32,
                        history,
                        solve_type,
                    );
                    ctxt.request_repaint();
                } else if bluetooth_moves.len() != 0 {
                    let mut moves = moves.clone();
                    moves.extend(bluetooth_moves);
                    if self.cube.is_solved() {
                        self.finish_bluetooth_solve(history, moves, bluetooth_name, solve_type);
                        ctxt.request_repaint();
                    } else {
                        let mut cube = Cube3x3x3::new();
                        cube.do_moves(self.cube.scramble());
                        let initial_state = cube.clone();
                        let mut final_moves = Vec::new();
                        let mut time = 0;
                        for mv in &moves {
                            cube.do_move(mv.move_());
                            time += mv.time();
                            final_moves.push(TimedMove::new(mv.move_(), time));
                        }
                        let analysis = PartialAnalysis::analyze(&CubeWithSolution {
                            initial_state,
                            solution: final_moves.clone(),
                        });
                        self.state = TimerState::BluetoothSolving(start, moves, analysis);
                    }
                }
            }
            TimerState::ExternalTimerSolving(_) => {
                if bluetooth_name.is_none() {
                    self.state = TimerState::Inactive(0, None);
                }
                ctxt.request_repaint();
            }
            TimerState::ManualTimeEntry(digits) => {
                if ctxt.input().key_down(Key::Escape) {
                    self.state = TimerState::Inactive(0, None);
                } else if ctxt.input().key_down(Key::Backspace) {
                    self.state =
                        TimerState::ManualTimeEntryDelay(digits / 10, Some(Key::Backspace));
                } else if ctxt.input().key_down(Key::Enter) {
                    let time = TimerState::digits_to_time(digits);
                    self.finish_solve(time, history, solve_type);
                } else {
                    for event in &ctxt.input().events {
                        if let Event::Text(text) = event {
                            if text.len() == 1 {
                                let ch = text.chars().next().unwrap();
                                if let Some(digit) = ch.to_digit(10) {
                                    self.state.update_for_numerical_input(digit);
                                }
                            }
                        }
                    }
                }
            }
            TimerState::ManualTimeEntryDelay(digits, wait_for) => {
                // This REALLY shouldn't be necessary but you get multiple Event::Text inputs
                // for every key. WTF. This has the side effect of possible eaten inputs
                // but its better than guaranteed repeated inputs.
                if let Some(key) = wait_for {
                    if !ctxt.input().key_down(key) {
                        self.state = TimerState::ManualTimeEntry(digits);
                    }
                } else {
                    self.state = TimerState::ManualTimeEntry(digits);
                }
            }
            TimerState::SolveComplete(time, last_solve) => {
                if ctxt.input().keys_down.len() == 0 && !touching {
                    self.state = TimerState::Inactive(time, Some(last_solve));
                    ctxt.request_repaint();
                }
            }
        }

        interact
    }

    fn check_for_expired_session(&mut self, history: &mut History, solve_type: SolveType) {
        self.session.check_solve_type(history, solve_type);

        if let Some(last_solve_time) = self.session.last_solve_time() {
            if Settings::auto_sessions_enabled(history) {
                let since = Local::now() - last_solve_time;
                if since.num_seconds() > Settings::auto_session_time(history) {
                    self.session.new_session(history);
                }
            }
        }
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        bluetooth_state: Option<Cube3x3x3>,
        bluetooth_events: Vec<BluetoothEvent>,
        bluetooth_name: Option<String>,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
        details: &mut Option<SolveDetails>,
        accept_keyboard: bool,
        solve_type: &mut SolveType,
    ) {
        self.cube.check_for_new_scramble();
        self.cube
            .update_bluetooth_state(&bluetooth_state, &bluetooth_events);

        if bluetooth_state.is_some() {
            // If Bluetooth cube is connected, force 3x3x3 solve type
            if !solve_type.is_3x3x3() && !solve_type.is_last_layer_training() {
                *solve_type = SolveType::Standard3x3x3;
            }
        }

        self.cube.check_solve_type(*solve_type, history);
        self.check_for_expired_session(history, *solve_type);

        ctxt.set_visuals(side_visuals());
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        if aspect >= 1.0 {
            // Landscape mode. Session details to the left.
            self.session
                .landscape_sidebar(ctxt, history, details, &mut self.cube);
        } else {
            // Portrait mode. Session details at the top.
            self.session
                .portrait_top_bar(ctxt, history, details, &mut self.cube, &self.state);
        }

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.vertical(|ui| {
                // The rest of the central area is the timer
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    let mut rect = ui.max_rect();
                    let mut center = rect.center();
                    let is_solving = self.state.is_solving();

                    if !is_solving {
                        // Draw new scramble button at top
                        self.cube.new_scramble_button(
                            if let TimerState::Inactive(_, _) = &self.state {
                                true
                            } else {
                                false
                            },
                            ui,
                            &mut rect,
                            &mut center,
                        );

                        if solve_type.is_last_layer_training() {
                            self.cube.training_penalty_buttons(
                                ui,
                                history,
                                &mut self.state,
                                &mut rect,
                                &mut center,
                            );
                        }
                    }

                    // The entire timer area is interactable, touch events should start/stop the
                    // timer anywhere in the timer area.
                    let interact = self.check_for_interaction_and_state_transition(
                        ctxt,
                        ui,
                        &rect,
                        history,
                        bluetooth_events,
                        bluetooth_name,
                        accept_keyboard,
                        *solve_type,
                    );

                    if is_solving {
                        if self.cube.is_bluetooth_active() {
                            bluetooth_timer_ui(
                                ui,
                                &rect,
                                &center,
                                &self.cube,
                                &self.state,
                                cube_rect,
                                framerate,
                            );
                        } else {
                            timer_ui(ui, &center, &self.state);
                        }
                    } else {
                        self.cube.scramble_ui(
                            ui,
                            &rect,
                            &center,
                            aspect,
                            &self.state,
                            cube_rect,
                            framerate,
                        );
                    }

                    self.cube
                        .rotate_cube_with_input(ctxt, ui, cube_rect, interact);
                });
            });
        });

        // Run at 10 FPS when solving (to update counting timer), or only when
        // updates occur otherwise
        match self.state {
            TimerState::Preparing(_, _, _)
            | TimerState::BluetoothPreparing(_, _, _)
            | TimerState::Solving(_)
            | TimerState::BluetoothSolving(_, _, _) => framerate.request(Some(10)),
            _ => (),
        }
    }

    pub fn paint_cube(
        &mut self,
        ctxt: &CtxRef,
        gl: &mut GlContext<'_, '_>,
        rect: &Rect,
    ) -> Result<()> {
        self.cube.paint_cube(ctxt, gl, rect)
    }
}
