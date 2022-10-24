use crate::cube::CubeRenderer;
use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::theme::Theme;
use crate::timer::analysis::TimerPostAnalysis;
use crate::timer::state::TimerState;
use crate::timer::BluetoothEvent;
use crate::widgets::fit_scramble;
use anyhow::Result;
use egui::{CtxRef, Pos2, Rect, Response, Sense, Ui, Vec2};
use tpscube_core::{
    scramble_2x2x2, scramble_3x3x3, scramble_4x4x4, scramble_last_layer, Cube, Cube2x2x2,
    Cube3x3x3, Cube4x4x4, History, InitialCubeState, LastLayerRandomization, Move, MoveSequence,
    Penalty, SolveType,
};

const TARGET_SCRAMBLE_FRACTION: f32 = 0.2;
const TARGET_ANALYSIS_SCRAMBLE_FRACTION: f32 = 0.15;
const TARGET_TIMER_FRACTION: f32 = 0.2;

const NEW_SCRAMBLE_PADDING: f32 = 4.0;

const TRAINING_PENALTY_SPACING: f32 = 32.0;
const TRAINING_PENALTY_PADDING: f32 = 16.0;

const ANALYSIS_MIN_PADDING: f32 = 24.0;
const ANALYSIS_MAX_PADDING: f32 = 64.0;
const MAX_ANALYSIS_WIDTH: f32 = 360.0;

pub struct TimerCube {
    current_scramble: Vec<Move>,
    current_scramble_displayed: bool,
    displayed_scramble: Vec<Move>,
    next_scramble: Option<Vec<Move>>,
    renderer: CubeRenderer,
    bluetooth_active: bool,
    scramble_move_index: Option<usize>,
    scramble_pending_move: Option<Move>,
    scramble_fix_moves: Vec<Move>,
    solve_type: SolveType,
    last_layer_training: LastLayerTrainingSettings,
}

enum ScrambleMoveResult {
    Good,
    Bad,
    BadWithPending(Move),
    Pending,
}

#[derive(Clone, Debug)]
pub struct LastLayerTrainingSettings {
    pub algorithms: LastLayerAlgorithmSelection,
    pub realistic_weights: bool,
    pub learning_multiplier: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LastLayerAlgorithmSelection {
    Known,
    Learning,
    KnownAndLearning,
    All,
}

impl LastLayerAlgorithmSelection {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "Known" => Some(LastLayerAlgorithmSelection::Known),
            "Learning" => Some(LastLayerAlgorithmSelection::Learning),
            "Known + Learning" => Some(LastLayerAlgorithmSelection::KnownAndLearning),
            "All" => Some(LastLayerAlgorithmSelection::All),
            _ => None,
        }
    }
}

impl ToString for LastLayerAlgorithmSelection {
    fn to_string(&self) -> String {
        match self {
            LastLayerAlgorithmSelection::Known => "Known".to_string(),
            LastLayerAlgorithmSelection::Learning => "Learning".to_string(),
            LastLayerAlgorithmSelection::KnownAndLearning => "Known + Learning".to_string(),
            LastLayerAlgorithmSelection::All => "All".to_string(),
        }
    }
}

impl TimerCube {
    pub fn new() -> Self {
        let current_scramble = scramble_3x3x3();
        let displayed_scramble = current_scramble.clone();
        let mut renderer = CubeRenderer::new(Box::new(Cube3x3x3::new()));
        renderer.reset_cube_state();
        renderer.do_moves(&current_scramble);

        Self {
            current_scramble,
            current_scramble_displayed: false,
            displayed_scramble,
            next_scramble: Some(scramble_3x3x3()),
            renderer,
            bluetooth_active: false,
            scramble_move_index: None,
            scramble_pending_move: None,
            scramble_fix_moves: Vec::new(),
            solve_type: SolveType::Standard3x3x3,
            last_layer_training: LastLayerTrainingSettings {
                algorithms: LastLayerAlgorithmSelection::All,
                realistic_weights: true,
                learning_multiplier: 1,
            },
        }
    }

    pub fn scramble(&self) -> &[Move] {
        &self.current_scramble
    }

    pub fn is_bluetooth_active(&self) -> bool {
        self.bluetooth_active
    }

    fn generate_scramble(&self) -> Vec<Move> {
        match self.solve_type {
            SolveType::Standard2x2x2 => scramble_2x2x2(),
            SolveType::Standard3x3x3 | SolveType::OneHanded3x3x3 | SolveType::Blind3x3x3 => {
                scramble_3x3x3()
            }
            SolveType::Standard4x4x4 | SolveType::Blind4x4x4 => scramble_4x4x4(),
            SolveType::OLLTraining => {
                scramble_last_layer(LastLayerRandomization::RandomStateUnsolved)
            }
            SolveType::PLLTraining => {
                scramble_last_layer(LastLayerRandomization::OrientedRandomStateUnsolved)
            }
        }
    }

    pub fn new_scramble(&mut self) {
        if let Some(scramble) = &self.next_scramble {
            self.current_scramble = scramble.clone();
        } else {
            self.current_scramble = self.generate_scramble();
        }
        self.current_scramble_displayed = false;
        self.displayed_scramble = self.current_scramble.clone();
        self.next_scramble = None;

        if self.bluetooth_active {
            self.display_scramble_from_current_state();
        } else {
            self.renderer.reset_cube_state();
            self.renderer.do_moves(&self.current_scramble);
            self.renderer.reset_angle();
        }
    }

    pub fn display_scramble_from_current_state(&mut self) {
        if self.bluetooth_active {
            let state = self.renderer.cube_state();

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

    pub fn bluetooth_started(&mut self, state: &Cube3x3x3) {
        self.bluetooth_active = true;
        self.renderer.set_cube_state(Box::new(state.clone()));
        self.renderer.reset_angle();

        self.display_scramble_from_current_state();
    }

    pub fn bluetooth_lost(&mut self) {
        self.bluetooth_active = false;
        self.scramble_move_index = None;
        self.scramble_pending_move = None;
        self.scramble_fix_moves.clear();

        self.displayed_scramble = self.current_scramble.clone();
        self.renderer.reset_cube_state();
        self.renderer.do_moves(&self.current_scramble);
        self.renderer.reset_angle();
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
                ScrambleMoveResult::BadWithPending(pending)
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

    pub fn apply_bluetooth_moves_for_scramble(&mut self, events: &[BluetoothEvent]) {
        for event in events {
            if let BluetoothEvent::Move(mv) = event {
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
                        ScrambleMoveResult::BadWithPending(pending) => {
                            self.bad_bluetooth_move(pending);
                            self.bad_bluetooth_move(mv.move_());
                        }
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
                            ScrambleMoveResult::BadWithPending(pending) => {
                                self.bad_bluetooth_move(pending);
                                self.bad_bluetooth_move(mv.move_());
                            }
                            ScrambleMoveResult::Pending => (),
                        }
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

    pub fn check_for_new_scramble(&mut self) {
        // Generate a scramble when the current one is onscreen. The slight delay will
        // not be noticed as much when performing a new scramble.
        if self.current_scramble_displayed && self.next_scramble.is_none() {
            self.next_scramble = Some(self.generate_scramble());
        }
    }

    pub fn update_bluetooth_state(
        &mut self,
        bluetooth_state: &Option<Cube3x3x3>,
        bluetooth_events: &[BluetoothEvent],
    ) {
        if let Some(state) = bluetooth_state {
            if self.bluetooth_active {
                for event in bluetooth_events {
                    if let BluetoothEvent::Move(mv) = event {
                        self.renderer.do_move(mv.move_());
                    }
                }
            } else {
                self.bluetooth_started(state);
            }
        } else {
            if self.bluetooth_active {
                self.bluetooth_lost();
            }
        }
    }

    pub fn update_bluetooth_scramble_and_check_finish(
        &mut self,
        bluetooth_events: &[BluetoothEvent],
    ) -> bool {
        if self.bluetooth_active {
            self.apply_bluetooth_moves_for_scramble(bluetooth_events);
            if let Some(move_index) = self.scramble_move_index {
                if move_index >= self.displayed_scramble.len() && self.scramble_fix_moves.len() == 0
                {
                    // Scramble complete, get ready to transition to solving
                    return true;
                }
            }
        }
        false
    }

    pub fn is_solved(&self) -> bool {
        self.renderer.is_solved()
    }

    pub fn new_scramble_button(
        &mut self,
        active: bool,
        ui: &mut Ui,
        rect: &mut Rect,
        center: &mut Pos2,
    ) {
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
            if active {
                if interact.hovered() {
                    Theme::Red.into()
                } else {
                    Theme::Disabled.into()
                }
            } else {
                Theme::Light.into()
            },
        );

        // Check for new scramble clicks
        if interact.clicked() && active {
            self.new_scramble();
        }

        // Adjust remaining rectangle to remove new scramble button area
        let top_left = Pos2::new(
            rect.left(),
            new_scramble_rect.bottom() + NEW_SCRAMBLE_PADDING,
        );
        *rect = Rect::from_min_size(
            top_left,
            Vec2::new(rect.width(), rect.bottom() - top_left.y),
        );
        *center = rect.center();
    }

    pub fn training_penalty_buttons(
        &mut self,
        ui: &mut Ui,
        history: &mut History,
        state: &mut TimerState,
        rect: &mut Rect,
        center: &mut Pos2,
    ) {
        if let TimerState::Inactive(_, Some(last_solve)) = state {
            if last_solve.solve_type != self.solve_type
                || !last_solve.solve_type.is_last_layer_training()
            {
                return;
            }

            let ok_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "✔  Solve OK".into());
            let misrecognize_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "👁  Misrecognized".into());
            let misexecute_galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "✖  Misexecuted".into());

            let total_width = ok_galley.size.x
                + misrecognize_galley.size.x
                + misexecute_galley.size.x
                + TRAINING_PENALTY_SPACING * 2.0;
            let mut x = rect.center().x - total_width / 2.0;
            let bottom_padding = TRAINING_PENALTY_PADDING + 32.0;

            let ok_rect;
            let misrecognize_rect;
            let misexecute_rect;
            if x < TRAINING_PENALTY_SPACING {
                // Too wide to fit, place on two lines
                ok_rect = Rect::from_min_size(
                    Pos2::new(
                        rect.center().x - ok_galley.size.x / 2.0,
                        rect.bottom() - bottom_padding - ok_galley.size.y * 2.5,
                    ),
                    ok_galley.size,
                );

                let total_width = misrecognize_galley.size.x
                    + misexecute_galley.size.x
                    + TRAINING_PENALTY_SPACING;
                x = rect.center().x - total_width / 2.0;
                misrecognize_rect = Rect::from_min_size(
                    Pos2::new(
                        x,
                        rect.bottom() - bottom_padding - misrecognize_galley.size.y,
                    ),
                    misrecognize_galley.size,
                );
                x += misrecognize_galley.size.x + TRAINING_PENALTY_SPACING;
                misexecute_rect = Rect::from_min_size(
                    Pos2::new(x, rect.bottom() - bottom_padding - misexecute_galley.size.y),
                    misexecute_galley.size,
                );
            } else {
                ok_rect = Rect::from_min_size(
                    Pos2::new(x, rect.bottom() - bottom_padding - ok_galley.size.y),
                    ok_galley.size,
                );
                x += ok_galley.size.x + TRAINING_PENALTY_SPACING;
                misrecognize_rect = Rect::from_min_size(
                    Pos2::new(
                        x,
                        rect.bottom() - bottom_padding - misrecognize_galley.size.y,
                    ),
                    misrecognize_galley.size,
                );
                x += misrecognize_galley.size.x + TRAINING_PENALTY_SPACING;
                misexecute_rect = Rect::from_min_size(
                    Pos2::new(x, rect.bottom() - bottom_padding - misexecute_galley.size.y),
                    misexecute_galley.size,
                );
            }

            let ok_interact = ui.allocate_rect(ok_rect, Sense::click());
            ui.painter().galley(
                ok_rect.left_top(),
                ok_galley,
                if ok_interact.hovered() || last_solve.penalty == Penalty::None {
                    Theme::Green.into()
                } else {
                    Theme::Light.into()
                },
            );

            let misrecognize_interact = ui.allocate_rect(misrecognize_rect, Sense::click());
            ui.painter().galley(
                misrecognize_rect.left_top(),
                misrecognize_galley,
                if misrecognize_interact.hovered() || last_solve.penalty == Penalty::RecognitionDNF
                {
                    Theme::Red.into()
                } else {
                    Theme::Light.into()
                },
            );

            let misexecute_interact = ui.allocate_rect(misexecute_rect, Sense::click());
            ui.painter().galley(
                misexecute_rect.left_top(),
                misexecute_galley,
                if misexecute_interact.hovered() || last_solve.penalty == Penalty::ExecutionDNF {
                    Theme::Red.into()
                } else {
                    Theme::Light.into()
                },
            );

            // Check for clicks
            if ok_interact.clicked() {
                history.penalty(last_solve.id.clone(), Penalty::None);
                let _ = history.local_commit();
                last_solve.penalty = Penalty::None;
            }

            if misrecognize_interact.clicked() {
                history.penalty(last_solve.id.clone(), Penalty::RecognitionDNF);
                let _ = history.local_commit();
                last_solve.penalty = Penalty::RecognitionDNF;
            }

            if misexecute_interact.clicked() {
                history.penalty(last_solve.id.clone(), Penalty::ExecutionDNF);
                let _ = history.local_commit();
                last_solve.penalty = Penalty::ExecutionDNF;
            }

            // Adjust remaining rectangle to remove new scramble button area
            let bottom_left = Pos2::new(rect.left(), ok_rect.top() - TRAINING_PENALTY_PADDING);
            *rect = Rect::from_min_size(
                rect.left_top(),
                Vec2::new(rect.width(), bottom_left.y - rect.top()),
            );
            *center = rect.center();
        }
    }

    pub fn scramble_ui(
        &mut self,
        ui: &mut Ui,
        rect: &Rect,
        center: &Pos2,
        aspect: f32,
        state: &TimerState,
        cube_rect: &mut Option<Rect>,
        framerate: &mut Framerate,
    ) {
        let analysis = if let Some(analysis) = state.analysis() {
            if aspect >= 1.0 {
                TimerPostAnalysis::new(analysis.step_summary())
            } else {
                TimerPostAnalysis::new(Vec::new())
            }
        } else {
            TimerPostAnalysis::new(Vec::new())
        };

        // Compute sizes of components in the main view
        let target_scramble_height = rect.height()
            * if analysis.present() {
                TARGET_ANALYSIS_SCRAMBLE_FRACTION
            } else {
                TARGET_SCRAMBLE_FRACTION
            };
        let target_timer_height = rect.height() * TARGET_TIMER_FRACTION;

        let scramble_padding = 8.0;

        let scramble_font = if self.displayed_scramble.len() > 25 {
            FontSize::Section
        } else {
            FontSize::Scramble
        };

        let (fix, scramble) = if self.bluetooth_active && self.scramble_fix_moves.len() > 0 {
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
                fit_scramble(ui, scramble_font, &self.displayed_scramble, rect.width()),
            )
        };

        let scramble_line_height = ui.fonts().row_height(scramble_font.into());
        let min_scramble_height = scramble_line_height * scramble.len() as f32;
        let scramble_height = min_scramble_height.max(target_scramble_height);

        let analysis_height = analysis.height(ui);

        let min_timer_height = ui.fonts().row_height(FontSize::Timer.into());
        let timer_overlap = min_timer_height * if analysis.present() { 0.2 } else { 0.4 };
        let timer_height = min_timer_height
            .max(target_timer_height)
            .max(analysis_height);
        let timer_padding = if aspect >= 1.0 {
            16.0 + min_timer_height * 0.2
        } else {
            16.0
        };

        let show_cube = !self.solve_type.is_last_layer_training();
        let cube_height = rect.height()
            - (scramble_padding + scramble_height + timer_height + timer_padding - timer_overlap);

        let margin = if show_cube {
            0.0
        } else {
            (cube_height - min_timer_height * 0.25) / 2.0
        };

        // Render scramble
        let mut y =
            rect.top() + margin + scramble_padding + (scramble_height - min_scramble_height) / 2.0;
        let mut move_idx = 0;
        for (line_idx, line) in scramble.iter().enumerate() {
            // Layout individual moves in the scramble
            let mut tokens = Vec::new();
            if fix && line_idx == 0 {
                tokens.push(ui.fonts().layout_single_line(
                    scramble_font.into(),
                    "Scramble incorrect, fix with".into(),
                ));
            } else {
                for (idx, mv) in line.iter().enumerate() {
                    tokens.push(ui.fonts().layout_single_line(
                        scramble_font.into(),
                        if idx == 0 {
                            mv.to_string()
                        } else {
                            format!("  {}", mv.to_string())
                        },
                    ));
                }
            }

            // Determine line width and center on screen
            let line_width = tokens.iter().fold(0.0, |sum, token| sum + token.size.x);
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
                        Theme::Light.into()
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
        if show_cube {
            let computed_cube_rect = Rect::from_min_size(
                Pos2::new(center.x - cube_height / 2.0, y),
                Vec2::new(cube_height, cube_height),
            );
            if computed_cube_rect.width() > 0.0 && computed_cube_rect.height() > 0.0 {
                *cube_rect = Some(computed_cube_rect);
                if self.animating() {
                    framerate.request_max();
                }
            }
        } else {
            *cube_rect = None;
        }

        // Layout timer
        let timer_galley = ui
            .fonts()
            .layout_single_line(FontSize::Timer.into(), state.current_time_string());
        let timer_width = timer_galley.size.x;

        // Determine target width of analysis region
        let analysis_width = rect.width() - timer_width - ANALYSIS_MIN_PADDING * 2.0;
        let (analysis_width, analysis_padding) = if analysis_width > MAX_ANALYSIS_WIDTH {
            (
                MAX_ANALYSIS_WIDTH,
                (ANALYSIS_MIN_PADDING + (analysis_width - MAX_ANALYSIS_WIDTH) / 2.0)
                    .min(ANALYSIS_MAX_PADDING),
            )
        } else {
            (analysis_width, ANALYSIS_MIN_PADDING)
        };

        // Render timer
        let timer_x = if analysis.present() {
            center.x - (timer_width + analysis_width + analysis_padding) / 2.0
        } else {
            center.x - timer_width / 2.0
        };
        let timer_width = timer_galley.size.x;
        let move_count_height = ui.fonts().row_height(FontSize::Normal.into());
        let timer_y = rect.bottom()
            - margin
            - (timer_height / 2.0 + timer_padding)
            - (min_timer_height
                + if analysis.present() {
                    move_count_height
                } else {
                    0.0
                })
                / 2.0;
        ui.painter().galley(
            Pos2::new(timer_x, timer_y),
            timer_galley,
            state.current_time_color(),
        );

        // Render details below timer if analysis is present
        if analysis.present() {
            let tps: f32 = analysis.move_count() as f32 / (state.current_time() as f32 / 1000.0);
            let move_count_galley = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                format!("{} moves  {:.2} TPS", analysis.move_count(), tps),
            );
            let move_count_width = move_count_galley.size.x;
            ui.painter().galley(
                Pos2::new(
                    timer_x + timer_width / 2.0 - move_count_width / 2.0,
                    timer_y + min_timer_height + 16.0,
                ),
                move_count_galley,
                Theme::Disabled.into(),
            );
        }

        // Render analysis if present
        if analysis.present() {
            analysis.render(
                ui,
                Rect::from_min_size(
                    Pos2::new(
                        timer_x + timer_width + analysis_padding,
                        rect.bottom()
                            - (timer_height / 2.0 + timer_padding / 3.0)
                            - (analysis_height / 2.0),
                    ),
                    Vec2::new(analysis_width, analysis_height),
                ),
            );
        }
    }

    pub fn animating(&self) -> bool {
        self.renderer.animating()
    }

    pub fn paint_cube(
        &mut self,
        ctxt: &CtxRef,
        gl: &mut GlContext<'_, '_>,
        rect: &Rect,
    ) -> Result<()> {
        self.renderer.draw(ctxt, gl, rect)
    }

    pub fn rotate_cube_with_input(
        &mut self,
        ctxt: &CtxRef,
        ui: &Ui,
        cube_rect: &Option<Rect>,
        interact: Response,
    ) {
        if cube_rect.is_some() && ui.rect_contains_pointer(cube_rect.unwrap()) {
            let scroll_delta = ctxt.input().scroll_delta;
            self.renderer
                .adjust_angle(scroll_delta.x / 3.0, scroll_delta.y / 3.0);
        }
        if crate::is_mobile() != Some(true) && interact.dragged() {
            self.renderer.adjust_angle(
                ui.input().pointer.delta().x / 3.0,
                ui.input().pointer.delta().y / 3.0,
            );
        }
    }

    pub fn check_solve_type(&mut self, solve_type: SolveType, history: &mut History) {
        if self.solve_type == solve_type {
            return;
        }

        self.solve_type = solve_type;

        if solve_type.is_last_layer_training() {
            self.last_layer_training.algorithms = LastLayerAlgorithmSelection::from_str(
                &history
                    .setting_as_string("last_layer_training_algorithms")
                    .unwrap_or("All".into()),
            )
            .unwrap_or(LastLayerAlgorithmSelection::All);
            self.last_layer_training.realistic_weights = history
                .setting_as_bool("last_layer_training_realistic_weights")
                .unwrap_or(true);
            self.last_layer_training.learning_multiplier = history
                .setting_as_i64("last_layer_training_learning_multiplier")
                .unwrap_or(1)
                .clamp(1, 32) as usize;
        }

        self.renderer = match solve_type {
            SolveType::Standard2x2x2 => CubeRenderer::new(Box::new(Cube2x2x2::new())),
            SolveType::Standard3x3x3
            | SolveType::OneHanded3x3x3
            | SolveType::Blind3x3x3
            | SolveType::OLLTraining
            | SolveType::PLLTraining => CubeRenderer::new(Box::new(Cube3x3x3::new())),
            SolveType::Standard4x4x4 | SolveType::Blind4x4x4 => {
                CubeRenderer::new(Box::new(Cube4x4x4::new()))
            }
        };
        self.next_scramble = None;
        self.new_scramble();
    }

    pub fn solve_type(&self) -> SolveType {
        self.solve_type
    }

    pub fn last_layer_training_algorithms(&self) -> LastLayerAlgorithmSelection {
        self.last_layer_training.algorithms
    }

    pub fn last_layer_training_realistic_weights(&self) -> bool {
        self.last_layer_training.realistic_weights
    }

    pub fn last_layer_training_learning_multiplier(&self) -> usize {
        self.last_layer_training.learning_multiplier
    }

    pub fn set_last_layer_training_algorithms(
        &mut self,
        algorithms: LastLayerAlgorithmSelection,
        history: &mut History,
    ) {
        if self.last_layer_training.algorithms == algorithms {
            return;
        }

        self.last_layer_training.algorithms = algorithms;
        let _ =
            history.set_string_setting("last_layer_training_algorithms", &algorithms.to_string());

        self.next_scramble = None;
        self.new_scramble();
    }

    pub fn set_last_layer_training_realistic_weights(
        &mut self,
        realistic_weights: bool,
        history: &mut History,
    ) {
        if self.last_layer_training.realistic_weights == realistic_weights {
            return;
        }

        self.last_layer_training.realistic_weights = realistic_weights;
        let _ =
            history.set_bool_setting("last_layer_training_realistic_weights", realistic_weights);

        self.next_scramble = None;
        self.new_scramble();
    }

    pub fn set_last_layer_training_learning_multiplier(
        &mut self,
        learning_multplier: usize,
        history: &mut History,
    ) {
        if self.last_layer_training.learning_multiplier == learning_multplier {
            return;
        }

        self.last_layer_training.learning_multiplier = learning_multplier;
        let _ = history.set_i64_setting(
            "last_layer_training_learning_multiplier",
            learning_multplier as i64,
        );

        self.next_scramble = None;
        self.new_scramble();
    }
}
