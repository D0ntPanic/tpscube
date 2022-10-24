mod list;
mod report;

use crate::font::FontSize;
use crate::style::{content_visuals, side_visuals};
use crate::theme::Theme;
use crate::widgets::CustomWidgets;
use egui::{
    epaint::{Mesh, TextureId, Vertex, WHITE_UV},
    Align, CentralPanel, Color32, CtxRef, Direction, Layout, Rect, Rgba, Shape, SidePanel, Stroke,
    TopBottomPanel, Ui, Vec2,
};
use list::AlgorithmList;
use report::TPSReport;
use std::collections::HashMap;
use tpscube_core::{
    Algorithm, AlgorithmType, Analysis, Color, Cube, Cube3x3x3, Cube3x3x3Faces, CubeFace,
    CubeRotation, CubeWithSolution, ExtendedMove, ExtendedMoveContext, ExtendedMoveSequence,
    History, InitialCubeState, OLLAlgorithm, PLLAlgorithm,
};

pub struct AlgorithmsWidget {
    cached_update_id: Option<u64>,
    algorithm_stats: AlgorithmStats,
    mode: AlgorithmMode,
    sort: Sort,
}

struct AlgorithmStats {
    oll: HashMap<OLLAlgorithm, AlgorithmCounts>,
    pll: HashMap<PLLAlgorithm, AlgorithmCounts>,
}

#[derive(Default)]
struct AlgorithmCounts {
    perform_count: usize,
    total_moves: usize,
    total_recognition_time: u64,
    total_execution_time: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AlgorithmMode {
    Algorithms(AlgorithmType),
    TPSReport(AlgorithmType),
}

struct Sort {
    column: SortColumn,
    order: SortOrder,
}

enum SortColumn {
    Count,
    RecognitionTime,
    ExecutionTime,
    TotalTime,
    MoveCount,
    TPS,
    ExecutionTPS,
}

enum SortOrder {
    Ascending,
    Descending,
}

impl AlgorithmsWidget {
    pub fn new() -> Self {
        Self {
            cached_update_id: None,
            algorithm_stats: AlgorithmStats {
                oll: HashMap::new(),
                pll: HashMap::new(),
            },
            mode: AlgorithmMode::Algorithms(AlgorithmType::OLL),
            sort: Sort {
                column: SortColumn::TPS,
                order: SortOrder::Descending,
            },
        }
    }

    fn analyze(&mut self, history: &History) {
        self.algorithm_stats.oll.clear();
        self.algorithm_stats.pll.clear();

        for solve in history.iter() {
            if let Some(moves) = &solve.moves {
                let mut unsolved_state = Cube3x3x3::new();
                unsolved_state.do_moves(&solve.scramble);
                let analysis = Analysis::analyze(&CubeWithSolution {
                    initial_state: unsolved_state.clone(),
                    solution: moves.clone(),
                });

                if let Analysis::CFOP(cfop) = analysis {
                    for oll in cfop.oll {
                        let oll_entry = self
                            .algorithm_stats
                            .oll
                            .entry(oll.performed_algorithm)
                            .or_insert(AlgorithmCounts::default());
                        oll_entry.perform_count += 1;
                        oll_entry.total_moves += oll.moves.len();
                        oll_entry.total_recognition_time += oll.recognition_time as u64;
                        oll_entry.total_execution_time += oll.execution_time as u64;
                    }

                    for pll in cfop.pll {
                        let pll_entry = self
                            .algorithm_stats
                            .pll
                            .entry(pll.performed_algorithm)
                            .or_insert(AlgorithmCounts::default());
                        pll_entry.perform_count += 1;
                        pll_entry.total_moves += pll.moves.len();
                        pll_entry.total_recognition_time += pll.recognition_time as u64;
                        pll_entry.total_execution_time += pll.execution_time as u64;
                    }
                }
            }
        }
    }

    fn algorithm_options(&mut self, ui: &mut Ui) {
        if ui
            .mode_label(
                "OLL",
                self.mode == AlgorithmMode::Algorithms(AlgorithmType::OLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::Algorithms(AlgorithmType::OLL);
        }

        if ui
            .mode_label(
                "PLL",
                self.mode == AlgorithmMode::Algorithms(AlgorithmType::PLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::Algorithms(AlgorithmType::PLL);
        }
    }

    fn report_options(&mut self, ui: &mut Ui) {
        if ui
            .mode_label(
                "OLL",
                self.mode == AlgorithmMode::TPSReport(AlgorithmType::OLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::TPSReport(AlgorithmType::OLL);
        }

        if ui
            .mode_label(
                "PLL",
                self.mode == AlgorithmMode::TPSReport(AlgorithmType::PLL),
            )
            .clicked()
        {
            self.mode = AlgorithmMode::TPSReport(AlgorithmType::PLL);
        }
    }

    fn landscape_sidebar(&mut self, ctxt: &CtxRef) {
        SidePanel::left("left_algorithm_options")
            .default_width(160.0)
            .resizable(false)
            .show(ctxt, |ui| {
                ui.vertical(|ui| {
                    ui.section("Algorithms");
                    self.algorithm_options(ui);

                    ui.add_space(8.0);
                    ui.section("TPS Reports");
                    self.report_options(ui);
                });
            });
    }

    fn portrait_top_bar(&mut self, ctxt: &CtxRef) {
        TopBottomPanel::top("top_algorithm_options").show(ctxt, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(
                    Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::TOP),
                    |ui| {
                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("Algorithms");
                                    self.algorithm_options(ui);
                                    ui.add_space(4.0);
                                });
                            },
                        );

                        // Show separator between sections
                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::Disabled.into(),
                            };
                            ui.separator();
                        });

                        ui.allocate_ui(
                            Vec2::new((ui.max_rect().width() - 48.0) / 2.0, ui.max_rect().height()),
                            |ui| {
                                ui.vertical(|ui| {
                                    ui.section("TPS Reports");
                                    self.report_options(ui);
                                    ui.add_space(4.0);
                                });
                            },
                        );
                    },
                );
            });
        });
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        if self.cached_update_id != Some(history.update_id()) {
            self.cached_update_id = Some(history.update_id());
            self.analyze(history);
        }

        ctxt.set_visuals(side_visuals());
        let aspect = ctxt.available_rect().width() / ctxt.available_rect().height();
        if aspect >= 1.0 {
            // Landscape mode. Report options to the left.
            self.landscape_sidebar(ctxt);
        } else {
            // Portrait mode. Report options at the top.
            self.portrait_top_bar(ctxt);
        }

        ctxt.set_visuals(content_visuals());
        CentralPanel::default().show(ctxt, |ui| match self.mode {
            AlgorithmMode::Algorithms(alg_type) => {
                let list = AlgorithmList::new(alg_type);
                list.update(ui);
            }
            AlgorithmMode::TPSReport(alg_type) => {
                let report = TPSReport::new(&self.algorithm_stats, alg_type, &mut self.sort);
                report.update(ui);
            }
        });
    }
}

pub trait AlgorithmRender {
    fn draw(
        &self,
        ui: &mut Ui,
        moves: &[ExtendedMove],
        cube_size: f32,
        show_name: bool,
        rect: Option<Rect>,
    );
}

impl AlgorithmRender for Algorithm {
    fn draw(
        &self,
        ui: &mut Ui,
        moves: &[ExtendedMove],
        cube_size: f32,
        show_name: bool,
        rect: Option<Rect>,
    ) {
        let yellow_only = matches!(self, Algorithm::OLL(_));

        // Generate cube state from inverted algorithm
        let mut cube = Cube3x3x3Faces::new();
        let mut move_ctxt = ExtendedMoveContext::new(&mut cube);
        move_ctxt.do_move(ExtendedMove::Rotation(CubeRotation::X2));
        move_ctxt.do_moves(&moves.inverse());

        // Compute sizes of each element (cube representation and name below)
        let piece_size = cube_size / 4.0;
        let half_size = piece_size / 2.0;
        let quarter_size = piece_size / 6.0;
        let eighth_size = piece_size / 12.0;
        let name_height = if show_name {
            ui.fonts().row_height(FontSize::Normal.into())
        } else {
            0.0
        };
        let width = if show_name {
            ui.fonts()
                .layout_single_line(
                    FontSize::Normal.into(),
                    match self {
                        Algorithm::OLL(_) => "Antisune (#26)".into(),
                        Algorithm::PLL(_) => "Aa".into(),
                    },
                )
                .size
                .x
                .max(cube_size + name_height)
        } else {
            cube_size
        };

        // Allocate UI space for the cube and state name
        let rect = if let Some(rect) = rect {
            rect
        } else {
            ui.allocate_space(Vec2::new(width, cube_size + name_height))
                .1
        };

        // Prepare to render cube state
        let cube_rect = Rect::from_min_size(
            rect.center_top() - Vec2::new(cube_size / 2.0, 0.0),
            Vec2::new(cube_size, cube_size),
        );
        let right = piece_size * 4.0;
        let bottom = piece_size * 4.0;

        const FACE_COLORS: [[f32; 3]; 6] = [
            [1.0, 1.0, 1.0],
            [0.003, 0.5, 0.017],
            [0.6, 0.0036, 0.0036],
            [0.024, 0.06, 0.825],
            [1.0, 0.25, 0.00375],
            [1.0, 1.0, 0.04],
        ];

        let get_color = |face, row, col| -> Color32 {
            let color = cube.color(face, row, col);
            if !yellow_only || color == Color::Yellow {
                let rgba = &FACE_COLORS[color as u8 as usize];
                Rgba::from_rgb(rgba[0], rgba[1], rgba[2]).into()
            } else {
                Theme::Light.into()
            }
        };

        let render =
            |color, top_left_offset, top_right_offset, bottom_left_offset, bottom_right_offset| {
                let mut verts = Vec::new();
                let mut idx = Vec::new();
                verts.push(Vertex {
                    pos: cube_rect.left_top() + top_left_offset,
                    uv: WHITE_UV,
                    color,
                });
                verts.push(Vertex {
                    pos: cube_rect.left_top() + top_right_offset,
                    uv: WHITE_UV,
                    color,
                });
                verts.push(Vertex {
                    pos: cube_rect.left_top() + bottom_left_offset,
                    uv: WHITE_UV,
                    color,
                });
                verts.push(Vertex {
                    pos: cube_rect.left_top() + bottom_right_offset,
                    uv: WHITE_UV,
                    color,
                });
                idx.push(0);
                idx.push(1);
                idx.push(2);
                idx.push(1);
                idx.push(2);
                idx.push(3);
                ui.painter().add(Shape::mesh(Mesh {
                    indices: idx,
                    vertices: verts,
                    texture_id: TextureId::Egui,
                }));
            };

        // Render yellow face
        for row in 0..3 {
            for col in 0..3 {
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        cube_rect.left_top()
                            + Vec2::new(half_size + 1.0, half_size + 1.0)
                            + Vec2::new(piece_size, 0.0) * col as f32
                            + Vec2::new(0.0, piece_size) * row as f32,
                        Vec2::new(piece_size - 1.0, piece_size - 1.0),
                    ),
                    0.0,
                    get_color(CubeFace::Bottom, row, col),
                );
            }
        }

        // Render face adjacent to top of yellow face
        render(
            get_color(CubeFace::Front, 2, 0),
            Vec2::new(half_size + quarter_size + 1.0, 1.0),
            Vec2::new(half_size + piece_size + eighth_size, 1.0),
            Vec2::new(half_size + 1.0, half_size - 1.0),
            Vec2::new(half_size + piece_size, half_size - 1.0),
        );
        render(
            get_color(CubeFace::Front, 2, 1),
            Vec2::new(half_size + piece_size + eighth_size + 1.0, 1.0),
            Vec2::new(half_size + piece_size * 2.0 - eighth_size, 1.0),
            Vec2::new(half_size + piece_size + 1.0, half_size - 1.0),
            Vec2::new(half_size + piece_size * 2.0, half_size - 1.0),
        );
        render(
            get_color(CubeFace::Front, 2, 2),
            Vec2::new(right - half_size - piece_size - eighth_size + 1.0, 1.0),
            Vec2::new(right - half_size - quarter_size, 1.0),
            Vec2::new(right - half_size - piece_size + 1.0, half_size - 1.0),
            Vec2::new(right - half_size, half_size - 1.0),
        );

        // Render face adjacent to bottom of yellow face
        render(
            get_color(CubeFace::Back, 2, 2),
            Vec2::new(half_size + 1.0, bottom - half_size + 2.0),
            Vec2::new(half_size + piece_size, bottom - half_size + 2.0),
            Vec2::new(half_size + quarter_size + 1.0, bottom),
            Vec2::new(half_size + piece_size + eighth_size, bottom),
        );
        render(
            get_color(CubeFace::Back, 2, 1),
            Vec2::new(half_size + piece_size + 1.0, bottom - half_size + 2.0),
            Vec2::new(half_size + piece_size * 2.0, bottom - half_size + 2.0),
            Vec2::new(half_size + piece_size + eighth_size + 1.0, bottom),
            Vec2::new(half_size + piece_size * 2.0 - eighth_size, bottom),
        );
        render(
            get_color(CubeFace::Back, 2, 0),
            Vec2::new(
                right - half_size - piece_size + 1.0,
                bottom - half_size + 2.0,
            ),
            Vec2::new(right - half_size, bottom - half_size + 2.0),
            Vec2::new(right - half_size - piece_size - eighth_size + 1.0, bottom),
            Vec2::new(right - half_size - quarter_size, bottom),
        );

        // Render face adjacent to left of yellow face
        render(
            get_color(CubeFace::Left, 2, 2),
            Vec2::new(1.0, half_size + quarter_size + 1.0),
            Vec2::new(1.0, half_size + piece_size + eighth_size),
            Vec2::new(half_size - 1.0, half_size + 1.0),
            Vec2::new(half_size - 1.0, half_size + piece_size),
        );
        render(
            get_color(CubeFace::Left, 2, 1),
            Vec2::new(1.0, half_size + piece_size + eighth_size + 1.0),
            Vec2::new(1.0, half_size + piece_size * 2.0 - eighth_size),
            Vec2::new(half_size - 1.0, half_size + piece_size + 1.0),
            Vec2::new(half_size - 1.0, half_size + piece_size * 2.0),
        );
        render(
            get_color(CubeFace::Left, 2, 0),
            Vec2::new(1.0, bottom - half_size - piece_size - eighth_size + 1.0),
            Vec2::new(1.0, bottom - half_size - quarter_size),
            Vec2::new(half_size - 1.0, bottom - half_size - piece_size + 1.0),
            Vec2::new(half_size - 1.0, bottom - half_size),
        );

        // Render face adjacent to right of yellow face
        render(
            get_color(CubeFace::Right, 2, 0),
            Vec2::new(right - half_size + 2.0, half_size + 1.0),
            Vec2::new(right - half_size + 2.0, half_size + piece_size),
            Vec2::new(right, half_size + quarter_size + 1.0),
            Vec2::new(right, half_size + piece_size + eighth_size),
        );
        render(
            get_color(CubeFace::Right, 2, 1),
            Vec2::new(right - half_size + 2.0, half_size + piece_size + 1.0),
            Vec2::new(right - half_size + 2.0, half_size + piece_size * 2.0),
            Vec2::new(right, half_size + piece_size + eighth_size + 1.0),
            Vec2::new(right, half_size + piece_size * 2.0 - eighth_size),
        );
        render(
            get_color(CubeFace::Right, 2, 2),
            Vec2::new(
                right - half_size + 2.0,
                bottom - half_size - piece_size + 1.0,
            ),
            Vec2::new(right - half_size + 2.0, bottom - half_size),
            Vec2::new(right, bottom - half_size - piece_size - eighth_size + 1.0),
            Vec2::new(right, bottom - half_size - quarter_size),
        );

        if show_name {
            // Render name
            let name = ui.fonts().layout_single_line(
                FontSize::Normal.into(),
                match self {
                    Algorithm::OLL(oll) => match oll {
                        OLLAlgorithm::OLL(_) => oll.to_string(),
                        _ => format!("{} (#{})", oll.to_string(), oll.as_number()),
                    },
                    _ => self.to_string(),
                },
            );
            ui.painter().galley(
                rect.center_bottom() - Vec2::new(name.size.x / 2.0, name.size.y),
                name,
                Theme::Content.into(),
            );
        }
    }
}
