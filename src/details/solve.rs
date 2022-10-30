use crate::cube::CubeRenderer;
use crate::details::bar::SolveBar;
use crate::font::FontSize;
use crate::framerate::Framerate;
use crate::gl::GlContext;
use crate::style::dialog_visuals;
use crate::theme::Theme;
use crate::widgets::{
    color_for_recognition_step_index, color_for_step_index, date_string, fit_scramble,
    solve_time_string, CustomWidgets,
};
use anyhow::Result;
use egui::{
    Align, Color32, CtxRef, Direction, Event, Id, Key, Label, Layout, Pos2, Rect, Sense, Stroke,
    Ui, Vec2, Window,
};
use instant::Instant;
use tpscube_core::{
    Analysis, AnalysisStepSummary, AnalysisSummary, Cube, Cube2x2x2, Cube3x3x3, Cube4x4x4,
    CubeWithSolution, InitialCubeState, Penalty, Solve, SolveType,
};

const TARGET_MIN_WIDTH: f32 = 280.0;
const TARGET_MAX_WIDTH: f32 = 400.0;
const STEP_COLUMN_PADDING: f32 = 8.0;

pub struct SolveDetailsWindow {
    solve: Solve,
    unsolved_state: Box<dyn Cube>,
    analysis: Analysis,
    summary: Vec<AnalysisStepSummary>,
    renderer: CubeRenderer,
    replay_time: f32,
    replay_move_idx: usize,
    playing: bool,
    last_frame: Instant,
    mode: SolveDetailsMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SolveDetailsMode {
    Replay,
    Analysis,
}

impl SolveDetailsWindow {
    pub fn new(solve: Solve) -> Self {
        match solve.solve_type {
            SolveType::Standard2x2x2 => {
                let mut unsolved_state = Box::new(Cube2x2x2::new());
                unsolved_state.do_moves(&solve.scramble);
                let renderer = CubeRenderer::new(unsolved_state.dyn_clone());

                Self {
                    solve,
                    unsolved_state,
                    analysis: Analysis::default(),
                    summary: Vec::new(),
                    renderer,
                    replay_time: 0.0,
                    replay_move_idx: 0,
                    playing: false,
                    last_frame: Instant::now(),
                    mode: SolveDetailsMode::Replay,
                }
            }
            SolveType::Standard3x3x3
            | SolveType::OneHanded3x3x3
            | SolveType::Blind3x3x3
            | SolveType::OLLTraining
            | SolveType::PLLTraining => {
                let mut unsolved_state = Cube3x3x3::new();
                unsolved_state.do_moves(&solve.scramble);
                let renderer = CubeRenderer::new(Box::new(unsolved_state.clone()));

                let analysis = if let Some(solution) = &solve.moves {
                    Analysis::analyze(&CubeWithSolution {
                        initial_state: unsolved_state.clone(),
                        solution: solution.clone(),
                    })
                } else {
                    Analysis::default()
                };
                let summary = analysis.detailed_step_summary();

                Self {
                    solve,
                    unsolved_state: Box::new(unsolved_state),
                    analysis,
                    summary,
                    renderer,
                    replay_time: 0.0,
                    replay_move_idx: 0,
                    playing: false,
                    last_frame: Instant::now(),
                    mode: SolveDetailsMode::Replay,
                }
            }
            SolveType::Standard4x4x4 | SolveType::Blind4x4x4 => {
                let mut unsolved_state = Cube4x4x4::new();
                unsolved_state.do_moves(&solve.scramble);
                let renderer = CubeRenderer::new(Box::new(unsolved_state.clone()));

                Self {
                    solve,
                    unsolved_state: Box::new(unsolved_state),
                    analysis: Analysis::default(),
                    summary: Vec::new(),
                    renderer,
                    replay_time: 0.0,
                    replay_move_idx: 0,
                    playing: false,
                    last_frame: Instant::now(),
                    mode: SolveDetailsMode::Replay,
                }
            }
            SolveType::Megaminx => Self {
                solve,
                unsolved_state: Box::new(Cube3x3x3::new()),
                analysis: Analysis::default(),
                summary: Vec::new(),
                renderer: CubeRenderer::new(Box::new(Cube3x3x3::new())),
                replay_time: 0.0,
                replay_move_idx: 0,
                playing: false,
                last_frame: Instant::now(),
                mode: SolveDetailsMode::Replay,
            },
        }
    }

    fn go_to_move_idx(&mut self, move_idx: usize) {
        if let Some(solution) = &self.solve.moves {
            if move_idx > self.replay_move_idx {
                // Target is after current point, apply moves
                while self.replay_move_idx < move_idx {
                    if self.replay_move_idx >= solution.len() {
                        break;
                    }
                    self.renderer
                        .do_move(solution[self.replay_move_idx].move_());
                    self.replay_move_idx += 1;
                }
            } else if move_idx < self.replay_move_idx {
                // Target is before current point, apply inverse moves
                while self.replay_move_idx > move_idx && self.replay_move_idx > 0 {
                    self.replay_move_idx -= 1;
                    self.renderer
                        .do_move(solution[self.replay_move_idx].move_().inverse());
                }
            }
        }
    }

    fn go_to_time(&mut self, time: f32) {
        if let Some(solution) = &self.solve.moves {
            if solution.len() > 0 {
                // Find the point in the replay where this time is
                self.replay_time = time;
                let mut move_idx = solution.len();
                for (i, mv) in solution.iter().enumerate() {
                    if mv.time() as f32 / 1000.0 > time {
                        move_idx = i;
                        break;
                    }
                }

                if move_idx >= solution.len() {
                    // End of replay, set time to exact end
                    self.replay_time = solution.last().unwrap().time() as f32 / 1000.0;
                }

                // Submit moves to the animator to arrive at the target point
                self.go_to_move_idx(move_idx);
            }
        }
    }

    fn advance_replay_frame(&mut self) {
        let now = Instant::now();
        let time_passed = now - self.last_frame;
        self.last_frame = now;

        // Advance replay time according to how much time has passed since the
        // previously rendered frame
        self.replay_time += time_passed.as_secs_f32();

        // See when the solve should end so that we don't go past the end
        let final_time = self.solve.moves.as_ref().unwrap().last().unwrap().time() as f32 / 1000.0;

        if self.replay_time >= final_time
            || self.replay_move_idx >= self.solve.moves.as_ref().unwrap().len()
        {
            // Replay has ended, set replay to solved state and exactly at the end
            self.replay_time = final_time;
            self.go_to_move_idx(self.solve.moves.as_ref().unwrap().len());
            self.playing = false;
        } else {
            // Replay is still in progress, apply moves as they occur in the replay
            while self.replay_time
                > self.solve.moves.as_ref().unwrap()[self.replay_move_idx].time() as f32 / 1000.0
            {
                self.replay_time =
                    self.solve.moves.as_ref().unwrap()[self.replay_move_idx].time() as f32 / 1000.0;
                self.go_to_move_idx(self.replay_move_idx + 1);
            }
        }
    }

    fn replay_controls(&mut self, ctxt: &CtxRef, ui: &mut Ui) {
        let mut space_down = false;
        let mut left_down = false;
        let mut right_down = false;
        let mut home_down = false;
        let mut end_down = false;
        for event in &ctxt.input().events {
            match event {
                Event::Key { key, pressed, .. } => {
                    if *pressed {
                        match key {
                            Key::Space => space_down = true,
                            Key::ArrowLeft => left_down = true,
                            Key::ArrowRight => right_down = true,
                            Key::Home => home_down = true,
                            Key::End => end_down = true,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        ui.horizontal(|ui| {
            ui.visuals_mut().widgets.noninteractive.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Light.into(),
            };
            ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Content.into(),
            };
            ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Blue.into(),
            };
            ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                width: 1.0,
                color: Theme::Blue.into(),
            };

            if self.replay_move_idx > 0 {
                if ui
                    .add(Label::new("⏮").sense(Sense::click()))
                    .on_hover_text("Go to start")
                    .clicked()
                    || home_down
                {
                    self.replay_time = 0.0;
                    self.go_to_move_idx(0);
                    self.playing = false;
                }
            } else {
                ui.add(Label::new("⏮"));
            }

            if self.replay_move_idx > 0 {
                if ui
                    .add(Label::new("⬅").sense(Sense::click()))
                    .on_hover_text("Go to previous move")
                    .clicked()
                    || left_down
                {
                    self.go_to_move_idx(self.replay_move_idx - 1);
                    if self.replay_move_idx > 0 {
                        self.replay_time = self.solve.moves.as_ref().unwrap()
                            [self.replay_move_idx - 1]
                            .time() as f32
                            / 1000.0;
                    } else {
                        self.replay_time = 0.0;
                    }
                    self.playing = false;
                }
            } else {
                ui.add(Label::new("⬅"));
            }

            if self.playing {
                if ui
                    .add(Label::new("⏸").sense(Sense::click()))
                    .on_hover_text("Pause")
                    .clicked()
                    || space_down
                {
                    self.playing = false;
                }
            } else {
                if ui
                    .add(Label::new("▶").sense(Sense::click()))
                    .on_hover_text("Play")
                    .clicked()
                    || space_down
                {
                    if self.replay_move_idx >= self.solve.moves.as_ref().unwrap().len() {
                        self.renderer
                            .set_cube_state(self.unsolved_state.dyn_clone());
                        self.replay_move_idx = 0;
                        self.replay_time = 0.0;
                    }
                    self.playing = true;
                    self.last_frame = Instant::now();
                }
            }

            if self.replay_move_idx < self.solve.moves.as_ref().unwrap().len() {
                if ui
                    .add(Label::new("➡").sense(Sense::click()))
                    .on_hover_text("Go to next move")
                    .clicked()
                    || right_down
                {
                    self.replay_time = self.solve.moves.as_ref().unwrap()[self.replay_move_idx]
                        .time() as f32
                        / 1000.0;
                    self.go_to_move_idx(self.replay_move_idx + 1);
                    self.playing = false;
                }
            } else {
                ui.add(Label::new("➡"));
            }

            if end_down {
                self.replay_time =
                    self.solve.moves.as_ref().unwrap().last().unwrap().time() as f32 / 1000.0;
                self.go_to_move_idx(self.solve.moves.as_ref().unwrap().len());
                self.playing = false;
            }

            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.visuals_mut().widgets.noninteractive.fg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Content.into(),
                };

                // Get maximum size of time for replay
                let total_ms = self.solve.moves.as_ref().unwrap().last().unwrap().time();
                let galley = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), solve_time_string(total_ms));
                let (_, rect) = ui.allocate_space(galley.size);

                // Show current time in replay. Don't use a label here so we can
                // allocate a consistent amount of space for the time, regardless
                // of where in the replay we are.
                let galley = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    solve_time_string((self.replay_time * 1000.0) as u32),
                );
                ui.painter().galley(
                    Pos2::new(
                        rect.right() - galley.size.x,
                        rect.center().y - galley.size.y / 2.0,
                    ),
                    galley,
                    Theme::Content.into(),
                );

                // Show scrub bar, which also acts as a breakdown of the various
                // phases of the solve.
                self.solve_bar(ctxt, ui);
            });
        });
    }

    fn solve_bar(&mut self, ctxt: &CtxRef, ui: &mut Ui) {
        let (id, rect) = ui.allocate_space(Vec2::new(ui.available_width(), 16.0));
        let response = ui.interact(rect, id, Sense::click_and_drag());
        let bar = SolveBar::new(
            &self.solve,
            &self.summary,
            self.solve.moves.as_ref().unwrap().last().unwrap().time(),
            Some(self.replay_time),
        );
        if let Some(navigate_time) = bar.interactive(ctxt, ui, rect, response) {
            self.go_to_time(navigate_time);
        }
    }

    fn scramble_and_replay(
        &mut self,
        ctxt: &CtxRef,
        ui: &mut Ui,
        target_width: f32,
        cube_size: f32,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
    ) {
        ui.vertical_centered(|ui| {
            // Fit scramble to desired window size
            let scramble_lines =
                fit_scramble(ui, FontSize::Section, &self.solve.scramble, target_width);

            // Add scramble at top
            for line in scramble_lines {
                let line: Vec<String> = line.iter().map(|mv| mv.to_string()).collect();
                let line = line.join(" ");
                if ui
                    .add(
                        Label::new(line)
                            .text_style(FontSize::Section.into())
                            .text_color(Theme::Blue)
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    ui.output().copied_text = self
                        .solve
                        .scramble
                        .iter()
                        .map(|mv| mv.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                };
            }

            // Add final time below scramble
            if let Some(time) = self.solve.final_time() {
                ui.add(Label::new(solve_time_string(time)).text_style(FontSize::Scramble.into()));
            } else {
                match self.solve.penalty {
                    Penalty::RecognitionDNF => ui.add(
                        Label::new("Misrecognized")
                            .text_style(FontSize::Scramble.into())
                            .text_color(Theme::Red),
                    ),
                    Penalty::ExecutionDNF => ui.add(
                        Label::new("Misexecuted")
                            .text_style(FontSize::Scramble.into())
                            .text_color(Theme::Red),
                    ),
                    _ => ui.add(
                        Label::new("DNF")
                            .text_style(FontSize::Scramble.into())
                            .text_color(Theme::Red),
                    ),
                };
            }

            let show_cube = self.solve.solve_type != SolveType::Megaminx;

            // Allocate space for the cube rendering, this will be rendered using
            // OpenGL later.
            if show_cube {
                let (id, rect) = ui.allocate_space(Vec2::new(cube_size, cube_size));
                let response = ui.interact(rect, id, Sense::click_and_drag());
                *cube_rect = Some(rect);
                if self.renderer.animating() {
                    framerate.request_max();
                }

                // Process rotation input for the cube rendering
                if ui.rect_contains_pointer(rect) {
                    let scroll_delta = ctxt.input().scroll_delta;
                    self.renderer
                        .adjust_angle(scroll_delta.x / 3.0, scroll_delta.y / 3.0);
                }
                if response.dragged() {
                    self.renderer.adjust_angle(
                        ui.input().pointer.delta().x / 3.0,
                        ui.input().pointer.delta().y / 3.0,
                    );
                }
            }

            if self.solve.moves.is_some() && self.solve.moves.as_ref().unwrap().len() > 0 {
                // Solve has a solution associated with it, show replay controls
                self.replay_controls(ctxt, ui);
            }
        });
    }

    fn solve_breakdown(&mut self, ui: &mut Ui, target_width: f32) {
        ui.vertical(|ui| {
            // Lay out column headers
            let step_name_header = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), "Step".into());
            let recognition_header = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), "Recognize".into());
            let execution_header = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), "Execute".into());
            let move_count_header = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), "Moves".into());
            let tps_header = ui
                .fonts()
                .layout_single_line(FontSize::Small.into(), "eTPS / TPS".into());

            // Go through all steps and lay out each to determine column widths
            let mut step_names = Vec::new();
            let mut step_algs = Vec::new();
            let mut recognitions = Vec::new();
            let mut executions = Vec::new();
            let mut move_counts = Vec::new();
            let mut tpses = Vec::new();
            let mut step_name_width: f32 = step_name_header.size.x;
            let mut recognition_width: f32 = if target_width > 320.0 {
                recognition_header.size.x
            } else {
                0.0
            };
            let mut execution_width: f32 = execution_header.size.x;
            let mut move_count_width: f32 = move_count_header.size.x;
            let mut tps_width: f32 = tps_header.size.x;
            let mut max_step_time = 0;
            let mut total_recognition_time = 0;
            let mut total_execution_time = 0;
            let mut total_move_count = 0;
            for step in &self.summary {
                // Lay out name and algorithm column
                let step_name = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), step.name.clone());
                let step_alg = if let Some(alg) = &step.algorithm {
                    Some(
                        ui.fonts()
                            .layout_single_line(FontSize::Normal.into(), format!("  {}", alg)),
                    )
                } else {
                    None
                };
                step_name_width = step_name_width.max(
                    step_name.size.x
                        + if let Some(alg) = &step_alg {
                            alg.size.x
                        } else {
                            0.0
                        },
                );
                step_names.push(step_name);
                step_algs.push(step_alg);

                // Lay out recognition time column
                let recognition = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    if step.recognition_time == 0 {
                        "".into()
                    } else {
                        solve_time_string(step.recognition_time)
                    },
                );
                recognition_width = recognition_width.max(recognition.size.x);
                recognitions.push(recognition);

                // Lay out execution time column
                let execution = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    solve_time_string(step.execution_time),
                );
                execution_width = execution_width.max(execution.size.x);
                executions.push(execution);

                // Lay out move count column
                let move_count = ui
                    .fonts()
                    .layout_single_line(FontSize::Normal.into(), format!("{}", step.move_count));
                move_count_width = move_count_width.max(move_count.size.x);
                move_counts.push(move_count);

                // Lay out tps column
                let etps_value: i32 = if step.execution_time != 0 {
                    let time = (step.execution_time + 5) / 10;
                    if time == 0 {
                        0
                    } else {
                        step.move_count as i32 * 1000 / time as i32
                    }
                } else {
                    -1
                };
                let tps_value: i32 = if step.execution_time != 0 {
                    let time = (step.execution_time + step.recognition_time + 5) / 10;
                    if time == 0 {
                        0
                    } else {
                        step.move_count as i32 * 1000 / time as i32
                    }
                } else {
                    -1
                };
                let tps = ui.fonts().layout_single_line(
                    FontSize::Normal.into(),
                    format!(
                        "{}.{} / {}.{}",
                        etps_value / 10,
                        etps_value % 10,
                        tps_value / 10,
                        tps_value % 10,
                    ),
                );
                tps_width = tps_width.max(tps.size.x);
                tpses.push(tps);

                max_step_time = max_step_time.max(step.recognition_time + step.execution_time);

                total_recognition_time += step.recognition_time;
                total_execution_time += step.execution_time;
                total_move_count += step.move_count;
            }

            // Lay out total times and move counts
            let total_recognition_time_ui = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                solve_time_string(total_recognition_time),
            );
            let total_execution_time_ui = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                solve_time_string(total_execution_time),
            );
            let total_move_count_ui = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), format!("{}", total_move_count));
            let total_etps_value = if total_execution_time != 0 {
                let time = (total_execution_time + 5) / 10;
                if time == 0 {
                    0
                } else {
                    total_move_count as i32 * 1000 / time as i32
                }
            } else {
                -1
            };
            let total_tps_value = if total_execution_time != 0 {
                let time = (total_execution_time + total_recognition_time + 5) / 10;
                if time == 0 {
                    0
                } else {
                    total_move_count as i32 * 1000 / time as i32
                }
            } else {
                -1
            };
            let total_tps_ui = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                // format!("{}.{}", total_tps_value / 10, total_tps_value % 10),
                format!(
                    "{}.{} / {}.{}",
                    total_etps_value / 10,
                    total_etps_value % 10,
                    total_tps_value / 10,
                    total_tps_value % 10,
                ),
            );
            recognition_width = recognition_width.max(total_recognition_time_ui.size.x);
            execution_width = execution_width.max(total_execution_time_ui.size.x);
            move_count_width = move_count_width.max(total_move_count_ui.size.x);
            tps_width = tps_width.max(total_tps_ui.size.x);

            // Add padding to columns and compute size of graph column
            step_name_width += STEP_COLUMN_PADDING;
            recognition_width += STEP_COLUMN_PADDING;
            execution_width += STEP_COLUMN_PADDING;
            move_count_width += STEP_COLUMN_PADDING;
            tps_width += STEP_COLUMN_PADDING;
            let graph_width = target_width
                - step_name_width
                - recognition_width
                - execution_width
                - move_count_width
                - tps_width;

            // Get column offsets
            let step_offset = 0.0;
            let graph_offset = step_name_width;
            let tps_offset = target_width;
            let move_count_offset = tps_offset - tps_width;
            let execution_offset = move_count_offset - move_count_width;
            let recognition_offset = execution_offset - execution_width;

            // Show column headers
            let (_, rect) = ui.allocate_space(Vec2::new(
                target_width,
                ui.fonts().row_height(FontSize::Small.into()),
            ));
            ui.painter().galley(
                Pos2::new(rect.left() + step_offset, rect.top()),
                step_name_header,
                Theme::Disabled.into(),
            );
            ui.painter().galley(
                Pos2::new(
                    rect.left() + recognition_offset - recognition_header.size.x,
                    rect.top(),
                ),
                recognition_header,
                Theme::Disabled.into(),
            );
            ui.painter().galley(
                Pos2::new(
                    rect.left() + execution_offset - execution_header.size.x,
                    rect.top(),
                ),
                execution_header,
                Theme::Disabled.into(),
            );
            ui.painter().galley(
                Pos2::new(
                    rect.left() + move_count_offset - move_count_header.size.x,
                    rect.top(),
                ),
                move_count_header,
                Theme::Disabled.into(),
            );
            ui.painter().galley(
                Pos2::new(rect.left() + tps_offset - tps_header.size.x, rect.top()),
                tps_header,
                Theme::Disabled.into(),
            );

            // Show step data
            let mut cur_move_index = 0;
            for (i, step) in self.summary.clone().iter().enumerate() {
                let (id, rect) = ui.allocate_space(Vec2::new(
                    target_width,
                    ui.fonts().row_height(FontSize::Small.into()),
                ));
                let response = ui.interact(rect, id, Sense::click());

                if response.clicked() {
                    // If a step row is clicked, navigate to the beginning of the
                    // step in the replay
                    if cur_move_index < self.solve.moves.as_ref().unwrap().len() {
                        self.replay_time = self.solve.moves.as_ref().unwrap()[cur_move_index].time()
                            as f32
                            / 1000.0;
                        self.go_to_move_idx(cur_move_index);
                    }
                }

                let color: Color32 = if response.hovered() {
                    Theme::Blue
                } else {
                    Theme::Content
                }
                .into();

                // Draw step name
                ui.painter().galley(
                    Pos2::new(rect.left() + step_offset, rect.top()),
                    step_names[i].clone(),
                    color,
                );

                // Draw step algorithm if there is one
                if let Some(alg) = &step_algs[i] {
                    ui.painter().galley(
                        Pos2::new(rect.left() + step_offset + step_names[i].size.x, rect.top()),
                        alg.clone(),
                        Theme::Disabled.into(),
                    );
                }

                // Draw recognition time
                ui.painter().galley(
                    Pos2::new(
                        rect.left() + recognition_offset - recognitions[i].size.x,
                        rect.top(),
                    ),
                    recognitions[i].clone(),
                    color,
                );

                // Draw execution time
                ui.painter().galley(
                    Pos2::new(
                        rect.left() + execution_offset - executions[i].size.x,
                        rect.top(),
                    ),
                    executions[i].clone(),
                    color,
                );

                // Draw move count
                ui.painter().galley(
                    Pos2::new(
                        rect.left() + move_count_offset - move_counts[i].size.x,
                        rect.top(),
                    ),
                    move_counts[i].clone(),
                    color,
                );

                // Draw tps
                ui.painter().galley(
                    Pos2::new(rect.left() + tps_offset - tpses[i].size.x, rect.top()),
                    tpses[i].clone(),
                    color,
                );

                // Draw graph
                let recognize_width =
                    graph_width * step.recognition_time as f32 / max_step_time as f32;
                let execution_width =
                    graph_width * step.execution_time as f32 / max_step_time as f32;
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        Pos2::new(rect.left() + graph_offset, rect.top() + 4.0),
                        Vec2::new(recognize_width, rect.height()),
                    ),
                    0.0,
                    color_for_recognition_step_index(step.major_step_index),
                );
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        Pos2::new(
                            rect.left() + graph_offset + recognize_width,
                            rect.top() + 4.0,
                        ),
                        Vec2::new(execution_width, rect.height()),
                    ),
                    0.0,
                    color_for_step_index(step.major_step_index),
                );

                cur_move_index += step.move_count;
            }

            // Divider to separate steps from totals
            ui.add_space(4.0);
            ui.scope(|ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                    width: 1.0,
                    color: Theme::Light.into(),
                };
                ui.separator();
            });

            // Draw totals
            let (_, rect) = ui.allocate_space(Vec2::new(
                target_width,
                ui.fonts().row_height(FontSize::Small.into()),
            ));

            // Draw total name
            let galley = ui
                .fonts()
                .layout_single_line(FontSize::Normal.into(), "Total".into());
            ui.painter().galley(
                Pos2::new(rect.left() + step_offset, rect.top()),
                galley,
                Theme::Content.into(),
            );

            // Draw total recognition time
            ui.painter().galley(
                Pos2::new(
                    rect.left() + recognition_offset - total_recognition_time_ui.size.x,
                    rect.top(),
                ),
                total_recognition_time_ui,
                Theme::Content.into(),
            );

            // Draw total execution time
            ui.painter().galley(
                Pos2::new(
                    rect.left() + execution_offset - total_execution_time_ui.size.x,
                    rect.top(),
                ),
                total_execution_time_ui,
                Theme::Content.into(),
            );

            // Draw total move count
            ui.painter().galley(
                Pos2::new(
                    rect.left() + move_count_offset - total_move_count_ui.size.x,
                    rect.top(),
                ),
                total_move_count_ui,
                Theme::Content.into(),
            );

            // Draw total tps
            ui.painter().galley(
                Pos2::new(rect.left() + tps_offset - total_tps_ui.size.x, rect.top()),
                total_tps_ui,
                Theme::Content.into(),
            );

            ui.add_space(8.0);
        });
    }

    pub fn update(
        &mut self,
        ctxt: &CtxRef,
        framerate: &mut Framerate,
        cube_rect: &mut Option<Rect>,
        open: &mut bool,
    ) {
        let full_rect = ctxt.available_rect();

        // Compute layout parameters
        let mut cube_size = (full_rect.height() / 2.0).min((full_rect.width() - 64.0) / 2.0);
        let mut target_width = ((full_rect.width() - 64.0) / 2.0)
            .min(TARGET_MAX_WIDTH)
            .max(cube_size);
        let mut tabbed = false;

        if target_width < TARGET_MIN_WIDTH {
            // If layout is too wide, use tabbed layout mode
            cube_size = (full_rect.height() / 2.0).min(full_rect.width() - 64.0);
            target_width = (full_rect.width() - 64.0)
                .min(TARGET_MAX_WIDTH)
                .max(cube_size);
            tabbed = true;
        }

        ctxt.set_visuals(dialog_visuals());
        Window::new(format!("Solve - {}", date_string(&self.solve.created)))
            .id(Id::new(format!(
                "solve_{}_{}",
                date_string(&self.solve.created),
                if tabbed { "_tabbed" } else { "" }
            )))
            .collapsible(false)
            .resizable(false)
            .open(open)
            .show(ctxt, |ui| {
                if tabbed && self.analysis.successful() {
                    // Tabbed layout with analysis, display options to choose replay or analysis
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.visuals_mut().widgets.hovered.fg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Content.into(),
                            };
                            ui.visuals_mut().widgets.active.fg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Green.into(),
                            };
                            ui.style_mut().spacing.item_spacing.x = 20.0;

                            ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                                width: 1.0,
                                color: if self.mode == SolveDetailsMode::Replay {
                                    Theme::Green
                                } else {
                                    Theme::Disabled
                                }
                                .into(),
                            };
                            if ui
                                .add(
                                    Label::new("🎞  Replay")
                                        .text_style(FontSize::Normal.into())
                                        .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                self.mode = SolveDetailsMode::Replay;
                            }

                            ui.visuals_mut().widgets.inactive.fg_stroke = Stroke {
                                width: 1.0,
                                color: if self.mode == SolveDetailsMode::Analysis {
                                    Theme::Green
                                } else {
                                    Theme::Disabled
                                }
                                .into(),
                            };
                            if ui
                                .add(
                                    Label::new("🖩  Analysis")
                                        .text_style(FontSize::Normal.into())
                                        .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                self.mode = SolveDetailsMode::Analysis;
                            }
                        });

                        ui.section_separator();

                        // Display correct UI based on current mode
                        match self.mode {
                            SolveDetailsMode::Replay => {
                                ui.allocate_ui(Vec2::new(target_width, 0.0), |ui| {
                                    self.scramble_and_replay(
                                        ctxt,
                                        ui,
                                        target_width,
                                        cube_size,
                                        framerate,
                                        cube_rect,
                                    )
                                });
                            }
                            SolveDetailsMode::Analysis => {
                                ui.allocate_ui(Vec2::new(target_width, 0.0), |ui| {
                                    self.solve_breakdown(ui, target_width)
                                });
                            }
                        }
                    });
                } else {
                    // Large layout, show replay and analysis side-by-side
                    ui.with_layout(
                        Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::TOP),
                        |ui| {
                            ui.allocate_ui(Vec2::new(target_width, 0.0), |ui| {
                                self.scramble_and_replay(
                                    ctxt,
                                    ui,
                                    target_width,
                                    cube_size,
                                    framerate,
                                    cube_rect,
                                );
                            });

                            if self.analysis.successful() {
                                ui.section_separator();

                                ui.allocate_ui(Vec2::new(target_width, 0.0), |ui| {
                                    self.solve_breakdown(ui, target_width);
                                });
                            }
                        },
                    );
                }
            });

        // Update replay state if playing
        if self.playing
            && self.solve.moves.is_some()
            && self.solve.moves.as_ref().unwrap().len() > 0
        {
            self.advance_replay_frame();

            // Always animate at full framerate during a replay so that the timer updates
            framerate.request_max();
        }
    }

    pub fn paint_cube(
        &mut self,
        ctxt: &CtxRef,
        gl: &mut GlContext<'_, '_>,
        rect: &Rect,
    ) -> Result<()> {
        self.renderer.draw(ctxt, gl, rect)
    }
}
