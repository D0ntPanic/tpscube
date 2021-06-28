use crate::cube::CubeRenderer;
use crate::font::{FontSize, LabelFontSize};
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::{solve_time_short_string, solve_time_string, CustomWidgets};
use anyhow::Result;
use chrono::Local;
use egui::{
    containers::ScrollArea, popup_below_widget, widgets::Label, Align, Align2, CentralPanel,
    Color32, CtxRef, Key, Layout, Pos2, Rect, Sense, SidePanel, Stroke, TopBottomPanel, Ui, Vec2,
};
use instant::Instant;
use tpscube_core::{
    scramble_3x3x3, Average, BestSolve, Cube, Cube3x3x3, History, Move, MoveSequence, Penalty,
    Solve, SolveList, SolveType, TimedMove,
};

const MIN_SCRAMBLE_LINES: usize = 2;
const MAX_SCRAMBLE_LINES: usize = 5;

const TARGET_SCRAMBLE_FRACTION: f32 = 0.2;
const TARGET_TIMER_FRACTION: f32 = 0.2;
const TARGET_CUBE_FRACTION: f32 = 0.75;

const NEW_SCRAMBLE_PADDING: f32 = 4.0;

#[derive(Clone)]
enum TimerState {
    Inactive(u32),
    Preparing(Instant, u32),
    BluetoothPreparing(Instant, u32),
    Ready,
    BluetoothReady,
    Solving(Instant),
    BluetoothSolving(Instant, Vec<TimedMove>),
    SolveComplete(u32),
}

struct CachedSessionSolves {
    update_id: Option<u64>,
    solves: Vec<Solve>,
    last_ao5: Option<u32>,
    last_ao12: Option<u32>,
    session_avg: Option<u32>,
    best_solve: Option<BestSolve>,
    best_ao5: Option<Average>,
    best_ao12: Option<Average>,
}

pub struct TimerWidget {
    state: TimerState,
    current_scramble: Vec<Move>,
    current_scramble_displayed: bool,
    displayed_scramble: Vec<Move>,
    next_scramble: Option<Vec<Move>>,
    session_solves: CachedSessionSolves,
    cube: CubeRenderer,
    bluetooth_active: bool,
    scramble_move_index: Option<usize>,
    scramble_pending_move: Option<Move>,
    scramble_fix_moves: Vec<Move>,
}

enum ScrambleMoveResult {
    Good,
    Bad,
    Pending,
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

impl TimerWidget {
    pub fn new() -> Self {
        let current_scramble = scramble_3x3x3();
        let displayed_scramble = current_scramble.clone();
        let mut cube = CubeRenderer::new();
        cube.reset_cube_state();
        cube.do_moves(&current_scramble);

        Self {
            state: TimerState::Inactive(0),
            current_scramble,
            current_scramble_displayed: false,
            displayed_scramble,
            next_scramble: Some(scramble_3x3x3()),
            session_solves: CachedSessionSolves::new(None, Vec::new()),
            cube,
            bluetooth_active: false,
            scramble_move_index: None,
            scramble_pending_move: None,
            scramble_fix_moves: Vec::new(),
        }
    }

    fn session_time(ui: &mut Ui, name: &str, small: bool, time: Option<u32>) {
        ui.horizontal(|ui| {
            if small {
                ui.add(Label::new(format!("{}:", name)).small());
            } else {
                ui.label(format!("{}:", name));
            }
            ui.with_layout(Layout::right_to_left(), |ui| {
                if let Some(time) = time {
                    ui.label(solve_time_string(time));
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
            TimerState::Ready
            | TimerState::BluetoothReady
            | TimerState::BluetoothPreparing(_, _) => solve_time_short_string(0),
            TimerState::Solving(start) | TimerState::BluetoothSolving(start, _) => {
                solve_time_short_string((Instant::now() - start).as_millis() as u32)
            }
        }
    }

    fn current_time_color(&self) -> Color32 {
        match self.state {
            TimerState::Inactive(_)
            | TimerState::BluetoothPreparing(_, _)
            | TimerState::Solving(_)
            | TimerState::BluetoothSolving(_, _)
            | TimerState::SolveComplete(_) => Theme::Content.into(),
            TimerState::Preparing(_, _) => {
                if self.is_solving() {
                    Theme::BackgroundHighlight.into()
                } else {
                    Theme::Content.into()
                }
            }
            TimerState::Ready | TimerState::BluetoothReady => Theme::Green.into(),
        }
    }

    fn scramble_lines(scramble: &[Move], line_count: usize) -> Vec<Vec<Move>> {
        let per_line = (scramble.len() + line_count - 1) / line_count;
        let mut lines = Vec::new();
        for chunks in scramble.chunks(per_line) {
            lines.push(chunks.to_vec());
        }
        lines
    }

    fn fit_scramble(ui: &Ui, scramble: &[Move], width: f32) -> Vec<Vec<Move>> {
        for line_count in MIN_SCRAMBLE_LINES..MAX_SCRAMBLE_LINES {
            let lines = Self::scramble_lines(scramble, line_count);
            if !lines.iter().any(|line| {
                ui.fonts()
                    .layout_single_line(
                        FontSize::Scramble.into(),
                        line.iter()
                            .map(|mv| mv.to_string())
                            .collect::<Vec<String>>()
                            .join("  "),
                    )
                    .size
                    .x
                    > width
            }) {
                return lines;
            }
        }
        Self::scramble_lines(scramble, MAX_SCRAMBLE_LINES)
    }

    pub fn is_solving(&self) -> bool {
        match self.state {
            TimerState::Inactive(_) | TimerState::SolveComplete(_) => false,
            TimerState::Preparing(start, _) => (Instant::now() - start).as_millis() > 10,
            _ => true,
        }
    }

    fn new_scramble(&mut self) {
        if let Some(scramble) = &self.next_scramble {
            self.current_scramble = scramble.clone();
        } else {
            self.current_scramble = scramble_3x3x3();
        }
        self.current_scramble_displayed = false;
        self.displayed_scramble = self.current_scramble.clone();
        self.next_scramble = None;

        if self.bluetooth_active {
            self.display_scramble_from_current_state();
        } else {
            self.cube.reset_cube_state();
            self.cube.do_moves(&self.current_scramble);
            self.cube.reset_angle();
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
        self.new_scramble();
    }

    fn finish_bluetooth_solve(
        &mut self,
        history: &mut History,
        moves: Vec<TimedMove>,
        name: Option<String>,
    ) {
        // Sanity check move data and modify move timing to be relative to the start
        // instead of relative to the prior move
        let mut cube = Cube3x3x3::new();
        cube.do_moves(&self.current_scramble);
        let mut final_moves = Vec::new();
        let mut time = 0;
        for mv in &moves {
            cube.do_move(mv.move_());
            time += mv.time();
            final_moves.push(TimedMove::new(mv.move_(), time));
        }
        let moves = if cube.is_solved() {
            Some(final_moves)
        } else {
            None
        };

        history.new_solve(Solve {
            id: Solve::new_id(),
            solve_type: SolveType::Standard3x3x3,
            session: history.current_session().into(),
            scramble: self.current_scramble.clone(),
            created: Local::now(),
            time,
            penalty: Penalty::None,
            device: name,
            moves,
        });
        let _ = history.local_commit();
        self.state = TimerState::SolveComplete(time);
        self.new_scramble();
    }

    fn abort_solve(&mut self, time: u32, history: &mut History) {
        if time > 2000 {
            // If some solve progress was made, add a DNF. Otherwise,
            // treat it as an accidental start.
            history.new_solve(Solve {
                id: Solve::new_id(),
                solve_type: SolveType::Standard3x3x3,
                session: history.current_session().into(),
                scramble: self.current_scramble.clone(),
                created: Local::now(),
                time,
                penalty: Penalty::DNF,
                device: None,
                moves: None,
            });
            let _ = history.local_commit();
            self.new_scramble();
        }
        self.state = TimerState::SolveComplete(0);
    }

    fn update_solve_cache(&mut self, history: &History) {
        if let Some(session) = history.sessions().get(history.current_session()) {
            // Check for updates
            if let Some(update_id) = self.session_solves.update_id {
                if update_id == session.update_id() {
                    // Already cached and up to date
                    return;
                }
            }

            // Cache solve information
            self.session_solves =
                CachedSessionSolves::new(Some(session.update_id()), session.to_vec(history));
        } else {
            // New session, invalidate cache
            self.session_solves = CachedSessionSolves::new(None, Vec::new());
        }
    }

    fn add_solve(ui: &mut Ui, idx: usize, solve: &Solve, history: &mut History) {
        // Change window theme so that popup menu stands out
        let old_visuals = ui.ctx().style().visuals.clone();
        ui.ctx().set_visuals(crate::style::popup_visuals());

        ui.style_mut().spacing.item_spacing.x = 0.0;
        ui.horizontal(|ui| {
            ui.add(Label::new(format!("{}.", idx + 1)).text_color(Theme::Disabled));
            ui.with_layout(Layout::right_to_left(), |ui| {
                let popup_id = ui.make_persistent_id(format!("timer-{}", solve.id));
                let response = ui.add(Label::new("  ☰").small().sense(Sense::click()));
                if response.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                popup_below_widget(ui, popup_id, &response, |ui| {
                    ui.set_min_width(180.0);
                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::None => true,
                                _ => false,
                            },
                            "No penalty",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::None);
                        let _ = history.local_commit();
                    }

                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::Time(2000) => true,
                                _ => false,
                            },
                            "2 second penalty",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::Time(2000));
                        let _ = history.local_commit();
                    }

                    if ui
                        .selectable_label(
                            match solve.penalty {
                                Penalty::DNF => true,
                                _ => false,
                            },
                            "DNF",
                        )
                        .clicked()
                    {
                        history.penalty(solve.id.clone(), Penalty::DNF);
                        let _ = history.local_commit();
                    }

                    ui.separator();

                    if ui.selectable_label(false, "Delete solve").clicked() {
                        history.delete_solve(solve.id.clone());
                        let _ = history.local_commit();
                    }
                });

                // Draw penalty if there is one, but always reserve space for it
                let penalty_galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Small.into(), " (+2) ".into());
                let (response, painter) = ui.allocate_painter(penalty_galley.size, Sense::hover());
                if let Penalty::Time(penalty) = solve.penalty {
                    painter.text(
                        response.rect.left_bottom(),
                        Align2::LEFT_BOTTOM,
                        format!(" (+{})", penalty / 1000),
                        FontSize::Small.into(),
                        Theme::Red.into(),
                    );
                }

                if let Some(time) = solve.final_time() {
                    ui.label(solve_time_string(time));
                } else {
                    ui.add(Label::new("DNF").text_color(Theme::Red));
                }
            });
        });

        // Restore old theme
        ui.ctx().set_visuals(old_visuals);
    }

    fn display_scramble_from_current_state(&mut self) {
        if self.bluetooth_active {
            let state = self.cube.cube_state();

            if state.is_solved() {
                self.displayed_scramble = self.current_scramble.clone();
            } else {
                // Use the solver to determine a minimal set of moves to arrive
                // at the correct state from the current state. This will be a
                // different sequence of moves from the original scramble but
                // it will arrive in the same state.
                let to_solved = state.solve_fast().unwrap();
                let mut new_state = Cube3x3x3::new();
                new_state.do_moves(&to_solved);
                new_state.do_moves(&self.current_scramble);
                self.displayed_scramble = new_state.solve().unwrap().inverse();
            }

            // Start from first move of new sequence
            self.scramble_move_index = Some(0);
            self.scramble_pending_move = None;
            self.scramble_fix_moves.clear();
        }
    }

    fn bluetooth_started(&mut self, state: Cube3x3x3) {
        self.bluetooth_active = true;
        self.cube.set_cube_state(state);
        self.cube.reset_angle();

        self.display_scramble_from_current_state();
    }

    fn bluetooth_lost(&mut self) {
        self.bluetooth_active = false;
        self.scramble_move_index = None;
        self.scramble_pending_move = None;
        self.scramble_fix_moves.clear();

        self.displayed_scramble = self.current_scramble.clone();
        self.cube.reset_cube_state();
        self.cube.do_moves(&self.current_scramble);
        self.cube.reset_angle();
    }

    fn apply_bluetooth_move_for_expected_move(
        &mut self,
        mv: Move,
        expected: Move,
    ) -> ScrambleMoveResult {
        if let Some(pending) = self.scramble_pending_move {
            // There is a pending move, check for correct completion
            self.scramble_pending_move = None;
            if mv == pending {
                // Redoing pending move will complete it
                ScrambleMoveResult::Good
            } else if mv == pending.inverse() {
                // Undoing pending move, stay at current state but clear
                // pending move.
                ScrambleMoveResult::Pending
            } else {
                ScrambleMoveResult::Bad
            }
        } else if expected.rotation() == 2 {
            // This is a 180 degree rotation that will appear as two moves from
            // the Bluetooth cube. Check for half completion.
            if mv.face() == expected.face() {
                // Correct face, set this as a pending move. If the same move
                // is performed again, this step is complete.
                self.scramble_pending_move = Some(mv);
                ScrambleMoveResult::Pending
            } else {
                ScrambleMoveResult::Bad
            }
        } else if mv == expected {
            // 90 degree rotation, move must match exactly
            ScrambleMoveResult::Good
        } else {
            ScrambleMoveResult::Bad
        }
    }

    fn bad_bluetooth_move(&mut self, mv: Move) {
        if self.scramble_fix_moves.len() > 0 {
            let last_fix_move = self.scramble_fix_moves.last().unwrap();
            if mv.face() == last_fix_move.face() {
                match Move::from_face_and_rotation(
                    mv.face(),
                    last_fix_move.rotation() - mv.rotation(),
                ) {
                    Some(mv) => *self.scramble_fix_moves.last_mut().unwrap() = mv,
                    None => {
                        self.scramble_fix_moves.pop();
                    }
                }
                return;
            }
        }
        self.scramble_fix_moves.push(mv.inverse());
    }

    fn apply_bluetooth_moves_for_scramble(&mut self, moves: &[TimedMove]) {
        for mv in moves {
            if self.scramble_fix_moves.len() > 0 {
                // There are moves needed to fix a bad scramble. Verify them.
                match self.apply_bluetooth_move_for_expected_move(
                    mv.move_(),
                    *self.scramble_fix_moves.last().unwrap(),
                ) {
                    ScrambleMoveResult::Good => {
                        self.scramble_fix_moves.pop();
                    }
                    ScrambleMoveResult::Bad => self.bad_bluetooth_move(mv.move_()),
                    ScrambleMoveResult::Pending => (),
                }
            } else if let Some(index) = self.scramble_move_index {
                if index >= self.displayed_scramble.len() {
                    // Extra moves when already done with scramble but timer hasn't
                    // started yet, go into fix mode to undo them.
                    self.bad_bluetooth_move(mv.move_());
                } else {
                    // Verify scramble step
                    let expected = self.displayed_scramble[index];
                    match self.apply_bluetooth_move_for_expected_move(mv.move_(), expected) {
                        ScrambleMoveResult::Good => self.scramble_move_index = Some(index + 1),
                        ScrambleMoveResult::Bad => self.bad_bluetooth_move(mv.move_()),
                        ScrambleMoveResult::Pending => (),
                    }
                }
            }
        }

        // If the fix path gets long, redo the scramble to get a correct result from
        // whatever the current state is.
        if self.scramble_fix_moves.len() > 3 {
            self.display_scramble_from_current_state();
        }
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        _frame: &mut epi::Frame<'_>,
        history: &mut History,
        bluetooth_state: Option<Cube3x3x3>,
        mut bluetooth_moves: Vec<TimedMove>,
        bluetooth_name: Option<String>,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
    ) {
        // Generate a scramble when the current one is onscreen. The slight delay will
        // not be noticed as much when performing a new scramble.
        if self.current_scramble_displayed && self.next_scramble.is_none() {
            self.next_scramble = Some(scramble_3x3x3());
        }

        // Check for Bluetooth state changes, and update rendered cube with Bluetooth moves
        if let Some(state) = bluetooth_state {
            if self.bluetooth_active {
                for mv in &bluetooth_moves {
                    self.cube.do_move(mv.move_());
                }
            } else {
                self.bluetooth_started(state);
            }
        } else {
            if self.bluetooth_active {
                self.bluetooth_lost();
            }
        }

        ctxt.set_visuals(side_visuals());
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        if aspect >= 1.0 {
            // Landscape mode. Session details to the left.
            SidePanel::left("left_timer")
                .default_width(175.0)
                .resizable(false)
                .show(ctxt, |ui| {
                    ui.section("Session");

                    self.update_solve_cache(history);

                    ui.vertical(|ui| {
                        Self::session_time(ui, "Last ao5", false, self.session_solves.last_ao5);
                        Self::session_time(ui, "Last ao12", false, self.session_solves.last_ao12);
                        Self::session_time(
                            ui,
                            "Session avg",
                            false,
                            self.session_solves.session_avg,
                        );
                        Self::session_time(
                            ui,
                            "Best solve",
                            false,
                            self.session_solves
                                .best_solve
                                .as_ref()
                                .map(|best| best.time),
                        );
                        Self::session_time(
                            ui,
                            "Best ao5",
                            false,
                            self.session_solves.best_ao5.as_ref().map(|best| best.time),
                        );
                        Self::session_time(
                            ui,
                            "Best ao12",
                            false,
                            self.session_solves.best_ao12.as_ref().map(|best| best.time),
                        );

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
                            for (idx, solve) in self.session_solves.solves.iter().enumerate().rev()
                            {
                                Self::add_solve(ui, idx, solve, history);
                                has_solves = true;
                            }
                            if !has_solves {
                                ui.add(
                                    Label::new("No solves in this session")
                                        .text_color(Theme::Disabled),
                                );
                            }
                        });
                });
        } else {
            // Portrait mode. Session details at the top.
            TopBottomPanel::top("top_timer").show(ctxt, |ui| {
                // Session header with embedded new session button.
                ui.horizontal(|ui| {
                    ui.add(
                        Label::new("Session")
                            .font_size(FontSize::Section)
                            .text_color(Theme::Blue),
                    );
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        if ui
                            .add(Label::new("↺  New session").sense(Sense::click()))
                            .clicked()
                        {
                            let _ = history.new_session();
                        }
                    })
                });
                ui.section_separator();

                self.update_solve_cache(history);

                // If the screen is too small, can only show last averages
                let best_cutoff = if crate::is_mobile() == Some(true) {
                    320.0
                } else {
                    290.0
                };
                let show_best = ui.max_rect().width() > best_cutoff;

                ui.horizontal(|ui| {
                    // Show last averages
                    ui.allocate_ui(
                        Vec2::new(
                            if show_best {
                                (ui.max_rect().width() - 24.0) / 2.0
                            } else {
                                ui.max_rect().width()
                            },
                            ui.max_rect().height(),
                        ),
                        |ui| {
                            ui.vertical(|ui| {
                                Self::session_time(
                                    ui,
                                    "Last ao5",
                                    true,
                                    self.session_solves.last_ao5,
                                );
                                Self::session_time(
                                    ui,
                                    "Last ao12",
                                    true,
                                    self.session_solves.last_ao12,
                                );
                                Self::session_time(
                                    ui,
                                    "Session avg",
                                    true,
                                    self.session_solves.session_avg,
                                );
                            });
                        },
                    );

                    if show_best {
                        // Show separator between last averages and best averages
                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Disabled.into(),
                            };
                            ui.separator();
                        });

                        // Show best averages
                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 24.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    Self::session_time(
                                        ui,
                                        "Best solve",
                                        true,
                                        self.session_solves
                                            .best_solve
                                            .as_ref()
                                            .map(|best| best.time),
                                    );
                                    Self::session_time(
                                        ui,
                                        "Best ao5",
                                        true,
                                        self.session_solves.best_ao5.as_ref().map(|best| best.time),
                                    );
                                    Self::session_time(
                                        ui,
                                        "Best ao12",
                                        true,
                                        self.session_solves
                                            .best_ao12
                                            .as_ref()
                                            .map(|best| best.time),
                                    );
                                });
                            },
                        );
                    }
                });

                ui.add_space(4.0);
            });
        }

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.vertical(|ui| {
                // The rest of the central area is the timer
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    let mut rect = ui.max_rect();
                    let mut center = rect.center();
                    let is_solving = self.is_solving();

                    if !is_solving {
                        // Draw new scramble button at top
                        let scramble_galley = ui
                            .fonts()
                            .layout_single_line(FontSize::Small.into(), "↺  New scramble".into());
                        let new_scramble_rect = Rect::from_center_size(
                            Pos2::new(
                                rect.center().x,
                                rect.top() + NEW_SCRAMBLE_PADDING + scramble_galley.size.y / 2.0,
                            ),
                            scramble_galley.size,
                        );
                        let interact = ui.allocate_rect(new_scramble_rect, Sense::click());
                        ui.painter().galley(
                            new_scramble_rect.left_top(),
                            scramble_galley,
                            if interact.hovered() {
                                Theme::Red.into()
                            } else {
                                Theme::Disabled.into()
                            },
                        );

                        // Check for new scramble clicks
                        if interact.clicked() {
                            if let TimerState::Inactive(_) = &self.state {
                                self.new_scramble();
                            }
                        }

                        // Adjust remaining rectangle to remove new scramble button area
                        let top_left = Pos2::new(
                            rect.left(),
                            new_scramble_rect.bottom() + NEW_SCRAMBLE_PADDING,
                        );
                        rect = Rect::from_min_size(
                            top_left,
                            Vec2::new(rect.width(), rect.bottom() - top_left.y),
                        );
                        center = rect.center();
                    }

                    // The entire timer area is interactable, touch events should start/stop the
                    // timer anywhere in the timer area.
                    let id = ui.make_persistent_id("timer_input");
                    let interact = ui.interact(rect, id, Sense::click_and_drag());
                    ui.memory().request_focus(id);

                    // Check for user input to interact with the timer
                    let touching = crate::is_mobile() == Some(true)
                        && (interact.is_pointer_button_down_on() || interact.dragged());
                    match self.state.clone() {
                        TimerState::Inactive(time) => {
                            if ctxt.input().keys_down.contains(&Key::Space) || touching {
                                self.state = TimerState::Preparing(Instant::now(), time);
                            }

                            if self.bluetooth_active {
                                self.apply_bluetooth_moves_for_scramble(&bluetooth_moves);
                                if let Some(move_index) = self.scramble_move_index {
                                    if move_index >= self.displayed_scramble.len()
                                        && self.scramble_fix_moves.len() == 0
                                    {
                                        // Scramble complete, get ready to transition to solving
                                        self.state =
                                            TimerState::BluetoothPreparing(Instant::now(), time);
                                    }
                                }
                            }
                        }
                        TimerState::Preparing(start, time) => {
                            if ctxt.input().keys_down.len() == 0 && !touching {
                                self.state = TimerState::Inactive(time);
                            } else if (Instant::now() - start).as_millis() > 300 {
                                self.state = TimerState::Ready;
                            }
                        }
                        TimerState::BluetoothPreparing(start, time) => {
                            if self.bluetooth_active {
                                if bluetooth_moves.len() != 0 {
                                    // For the first second after finishing a Bluetooth scramble,
                                    // cause any extra moves to transition to the fix bad
                                    // scramble state. This means that extra accidental turns
                                    // at the end of a scramble will not cause the timer to
                                    // start before the user is ready.
                                    self.apply_bluetooth_moves_for_scramble(&bluetooth_moves);
                                    if let Some(move_index) = self.scramble_move_index {
                                        if move_index < self.displayed_scramble.len()
                                            || self.scramble_fix_moves.len() != 0
                                        {
                                            self.state = TimerState::Inactive(time);
                                        } else if (Instant::now() - start).as_millis() >= 1000 {
                                            self.state = TimerState::BluetoothReady;
                                        }
                                    } else {
                                        self.state = TimerState::Inactive(time);
                                    }
                                } else if (Instant::now() - start).as_millis() >= 1000 {
                                    self.state = TimerState::BluetoothReady;
                                }
                            } else {
                                self.state = TimerState::Inactive(time);
                            }
                        }
                        TimerState::Ready => {
                            if ctxt.input().keys_down.len() == 0 && !touching {
                                self.state = TimerState::Solving(Instant::now());
                            }
                        }
                        TimerState::BluetoothReady => {
                            if bluetooth_moves.len() != 0 {
                                // Rewrite first move timing data to be at start
                                let first_move = TimedMove::new(bluetooth_moves[0].move_(), 0);
                                bluetooth_moves[0] = first_move;

                                // Start solving and keep track of moves
                                self.state =
                                    TimerState::BluetoothSolving(Instant::now(), bluetooth_moves);
                            }
                        }
                        TimerState::Solving(start) => {
                            if ctxt.input().keys_down.contains(&Key::Escape) {
                                self.abort_solve(
                                    (Instant::now() - start).as_millis() as u32,
                                    history,
                                );
                                ctxt.request_repaint();
                            } else if ctxt.input().keys_down.len() != 0 || touching {
                                self.finish_solve(
                                    (Instant::now() - start).as_millis() as u32,
                                    history,
                                );
                                ctxt.request_repaint();
                            }
                        }
                        TimerState::BluetoothSolving(start, moves) => {
                            if ctxt.input().keys_down.contains(&Key::Escape) {
                                self.abort_solve(
                                    (Instant::now() - start).as_millis() as u32,
                                    history,
                                );
                                ctxt.request_repaint();
                            } else if ctxt.input().keys_down.len() != 0 || touching {
                                self.finish_solve(
                                    (Instant::now() - start).as_millis() as u32,
                                    history,
                                );
                                ctxt.request_repaint();
                            } else if bluetooth_moves.len() != 0 {
                                let mut moves = moves.clone();
                                moves.extend(bluetooth_moves);
                                if self.cube.is_solved() {
                                    self.finish_bluetooth_solve(history, moves, bluetooth_name);
                                    ctxt.request_repaint();
                                } else {
                                    self.state = TimerState::BluetoothSolving(start, moves);
                                }
                            }
                        }
                        TimerState::SolveComplete(time) => {
                            if ctxt.input().keys_down.len() == 0 && !touching {
                                self.state = TimerState::Inactive(time);
                                ctxt.request_repaint();
                            }
                        }
                    }

                    if is_solving {
                        if self.bluetooth_active {
                            // In Bluetooth mode, render cube as well as timer
                            let timer_height = ui.fonts().row_height(FontSize::Timer.into());
                            let timer_padding = 32.0;
                            let cube_height = (rect.height() - timer_height - timer_padding)
                                * TARGET_CUBE_FRACTION;
                            let total_height = timer_height + timer_padding + cube_height;

                            // Allocate space for the cube rendering. This is 3D so it will be rendered
                            // with OpenGL after egui is done painting.
                            let y = center.y - total_height / 2.0;
                            let computed_cube_rect = Rect::from_min_size(
                                Pos2::new(center.x - cube_height / 2.0, y),
                                Vec2::new(cube_height, cube_height),
                            );
                            if computed_cube_rect.width() > 0.0 && computed_cube_rect.height() > 0.0
                            {
                                *cube_rect = Some(computed_cube_rect);
                                if self.cube.animating() {
                                    framerate.request_max();
                                }
                            }

                            // Draw timer
                            let galley = ui.fonts().layout_single_line(
                                FontSize::Timer.into(),
                                self.current_time_string(),
                            );
                            let timer_width = galley.size.x;
                            ui.painter().galley(
                                Pos2::new(
                                    center.x - timer_width / 2.0,
                                    y + cube_height + timer_padding,
                                ),
                                galley,
                                self.current_time_color(),
                            );
                        } else {
                            // Render timer only in center of screen
                            let timer_height = ui.fonts().row_height(FontSize::Timer.into());
                            let galley = ui.fonts().layout_single_line(
                                FontSize::Timer.into(),
                                self.current_time_string(),
                            );
                            let timer_width = galley.size.x;
                            ui.painter().galley(
                                Pos2::new(
                                    center.x - timer_width / 2.0,
                                    center.y - timer_height / 2.0,
                                ),
                                galley,
                                self.current_time_color(),
                            );
                        }
                    } else {
                        // Compute sizes of components in the main view
                        let target_scramble_height = rect.height() * TARGET_SCRAMBLE_FRACTION;
                        let target_timer_height = rect.height() * TARGET_TIMER_FRACTION;

                        let scramble_padding = 8.0;

                        let (fix, scramble) =
                            if self.bluetooth_active && self.scramble_fix_moves.len() > 0 {
                                (
                                    true,
                                    vec![
                                        vec![],
                                        self.scramble_fix_moves
                                            .iter()
                                            .rev()
                                            .cloned()
                                            .collect::<Vec<Move>>(),
                                    ],
                                )
                            } else {
                                (
                                    false,
                                    Self::fit_scramble(ui, &self.displayed_scramble, rect.width()),
                                )
                            };

                        let scramble_line_height = ui.fonts().row_height(FontSize::Scramble.into());
                        let min_scramble_height = scramble_line_height * scramble.len() as f32;
                        let scramble_height = min_scramble_height.max(target_scramble_height);

                        let min_timer_height = ui.fonts().row_height(FontSize::Timer.into());
                        let timer_overlap = min_timer_height * 0.4;
                        let timer_height = min_timer_height.max(target_timer_height);
                        let timer_padding = if aspect >= 1.0 {
                            16.0 + min_timer_height * 0.2
                        } else {
                            16.0
                        };

                        let cube_height = rect.height()
                            - (scramble_padding + scramble_height + timer_height + timer_padding
                                - timer_overlap);

                        // Render scramble
                        let mut y = rect.top()
                            + scramble_padding
                            + (scramble_height - min_scramble_height) / 2.0;
                        let mut move_idx = 0;
                        for (line_idx, line) in scramble.iter().enumerate() {
                            // Layout individual moves in the scramble
                            let mut tokens = Vec::new();
                            if fix && line_idx == 0 {
                                tokens.push(ui.fonts().layout_single_line(
                                    FontSize::Scramble.into(),
                                    "Scramble incorrect, fix with".into(),
                                ));
                            } else {
                                for (idx, mv) in line.iter().enumerate() {
                                    tokens.push(ui.fonts().layout_single_line(
                                        FontSize::Scramble.into(),
                                        if idx == 0 {
                                            mv.to_string()
                                        } else {
                                            format!("  {}", mv.to_string())
                                        },
                                    ));
                                }
                            }

                            // Determine line width and center on screen
                            let line_width =
                                tokens.iter().fold(0.0, |sum, token| sum + token.size.x);
                            let mut x = center.x - line_width / 2.0;

                            // Render individual moves
                            for token in tokens {
                                let width = token.size.x;
                                ui.painter().galley(
                                    Pos2::new(x, y),
                                    token,
                                    if !fix
                                        && (self.scramble_move_index.is_none()
                                            || Some(move_idx) == self.scramble_move_index)
                                    {
                                        Theme::Blue.into()
                                    } else if fix && (line_idx == 0 || move_idx == 0) {
                                        Theme::Red.into()
                                    } else {
                                        Theme::VeryLight.into()
                                    },
                                );

                                x += width;
                                if !fix || line_idx != 0 {
                                    move_idx += 1;
                                }
                            }

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
                            if self.cube.animating() {
                                framerate.request_max();
                            }
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
            });
        });

        // Run at 10 FPS when solving (to update counting timer), or only when
        // updates occur otherwise
        match self.state {
            TimerState::Preparing(_, _)
            | TimerState::BluetoothPreparing(_, _)
            | TimerState::Solving(_)
            | TimerState::BluetoothSolving(_, _) => framerate.request(Some(10)),
            _ => (),
        }
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
