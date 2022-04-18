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
    scramble_2x2x2, scramble_3x3x3, Cube, Cube2x2x2, Cube3x3x3, InitialCubeState, Move,
    MoveSequence, SolveType,
};

const TARGET_SCRAMBLE_FRACTION: f32 = 0.2;
const TARGET_ANALYSIS_SCRAMBLE_FRACTION: f32 = 0.15;
const TARGET_TIMER_FRACTION: f32 = 0.2;

const NEW_SCRAMBLE_PADDING: f32 = 4.0;

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
}

enum ScrambleMoveResult {
    Good,
    Bad,
    BadWithPending(Move),
    Pending,
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
                fit_scramble(
                    ui,
                    FontSize::Scramble,
                    &self.displayed_scramble,
                    rect.width(),
                ),
            )
        };

        let scramble_line_height = ui.fonts().row_height(FontSize::Scramble.into());
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

        let cube_height = rect.height()
            - (scramble_padding + scramble_height + timer_height + timer_padding - timer_overlap);

        // Render scramble
        let mut y = rect.top() + scramble_padding + (scramble_height - min_scramble_height) / 2.0;
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

        // Render move count below timer is analysis is present
        if analysis.present() {
            let move_count_galley = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                format!("{} moves", analysis.move_count()),
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

    pub fn check_solve_type(&mut self, solve_type: SolveType) {
        if self.solve_type == solve_type {
            return;
        }

        self.solve_type = solve_type;

        self.renderer = match solve_type {
            SolveType::Standard2x2x2 => CubeRenderer::new(Box::new(Cube2x2x2::new())),
            SolveType::Standard3x3x3 | SolveType::OneHanded3x3x3 | SolveType::Blind3x3x3 => {
                CubeRenderer::new(Box::new(Cube3x3x3::new()))
            }
        };
        self.next_scramble = None;
        self.new_scramble();
    }
}
