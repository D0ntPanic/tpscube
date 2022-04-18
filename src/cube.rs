use crate::center_generated::{CENTER_INDEX, CENTER_SOURCE_VERTS};
use crate::corner_generated::{CORNER_INDEX, CORNER_SOURCE_VERTS};
use crate::edge_generated::{EDGE_INDEX, EDGE_SOURCE_VERTS};
use crate::gl::{GlContext, GlRenderer, Vertex};
use anyhow::Result;
use egui::{CtxRef, Rect};
use gl_matrix::{
    common::{to_radian, Mat4},
    mat4, vec3,
};
use instant::Instant;
use num_traits::FloatConst;
use std::time::Duration;
use tpscube_core::{Cube, Cube3x3x3, CubeFace, Move};

const FACE_COLORS: [[f32; 3]; 6] = [
    [1.0, 1.0, 1.0],
    [0.003, 0.5, 0.017],
    [0.9, 0.003, 0.003],
    [0.016, 0.04, 0.55],
    [1.0, 0.167, 0.0025],
    [1.0, 1.0, 0.04],
];
const PLACEHOLDER_FACE_COLOR: [f32; 3] = [1.0, 0.0, 1.0];
const FACE_ROUGHNESS: f32 = 0.4;
const INNER_COLOR: [f32; 3] = [0.02, 0.02, 0.02];
const INNER_ROUGHNESS: f32 = 0.3;
const CUBE_SIZE: usize = 3;
const MODEL_OFFSET: [f32; 3] = [
    1.0 - CUBE_SIZE as f32,
    1.0 - CUBE_SIZE as f32,
    1.0 - CUBE_SIZE as f32,
];
const MODEL_SCALE: f32 = 1.0 / CUBE_SIZE as f32;

pub struct CubeRenderer {
    renderer: Option<GlRenderer>,
    cube: Cube3x3x3,
    target_cube: Cube3x3x3,
    move_queue: Vec<Move>,
    max_queued_moves: usize,
    pitch: f32,
    yaw: f32,
    verts: Vec<Vertex>,
    index: Vec<u16>,
    animation: Option<Animation>,
    anim_fixed_index: [Vec<u16>; 6],
    anim_moving_index: [Vec<u16>; 6],
    vert_ranges: [VertexRange; 6 * CUBE_SIZE * CUBE_SIZE],
}

pub struct SourceVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub color: i32,
}

#[derive(Clone, Copy)]
pub struct VertexRange {
    start_index: usize,
    model_verts: &'static [SourceVertex],
    face: i32,
}

struct Animation {
    start: Instant,
    length: Duration,
    face: CubeFace,
    angle: f32,
    move_: Move,
}

impl CubeRenderer {
    pub fn new() -> Self {
        let mut result = Self {
            renderer: None,
            cube: Cube3x3x3::new(),
            target_cube: Cube3x3x3::new(),
            move_queue: Vec::new(),
            max_queued_moves: 8,
            pitch: 30.0,
            yaw: -35.0,
            verts: Vec::new(),
            index: Vec::new(),
            animation: None,
            anim_fixed_index: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            anim_moving_index: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            vert_ranges: [VertexRange {
                start_index: 0,
                model_verts: EDGE_SOURCE_VERTS,
                face: 0,
            }; 6 * CUBE_SIZE * CUBE_SIZE],
        };

        // Add vertex information for corner pieces
        let e = CUBE_SIZE as i32 - 1;
        result.add_corner(
            [0, 0, 0],
            [1, -1],
            (CubeFace::Bottom, e, 0),
            (CubeFace::Back, e, e),
            (CubeFace::Left, e, 0),
        );
        result.add_corner(
            [e, 0, 0],
            [1, 0],
            (CubeFace::Bottom, e, e),
            (CubeFace::Right, e, e),
            (CubeFace::Back, e, 0),
        );
        result.add_corner(
            [0, e, 0],
            [-1, 2],
            (CubeFace::Top, 0, 0),
            (CubeFace::Left, 0, 0),
            (CubeFace::Back, 0, e),
        );
        result.add_corner(
            [e, e, 0],
            [-1, 1],
            (CubeFace::Top, 0, e),
            (CubeFace::Back, 0, 0),
            (CubeFace::Right, 0, e),
        );
        result.add_corner(
            [0, 0, e],
            [1, 2],
            (CubeFace::Bottom, 0, 0),
            (CubeFace::Left, e, e),
            (CubeFace::Front, e, 0),
        );
        result.add_corner(
            [e, 0, e],
            [1, 1],
            (CubeFace::Bottom, 0, e),
            (CubeFace::Front, e, e),
            (CubeFace::Right, e, 0),
        );
        result.add_corner(
            [0, e, e],
            [-1, -1],
            (CubeFace::Top, e, 0),
            (CubeFace::Front, 0, 0),
            (CubeFace::Left, 0, e),
        );
        result.add_corner(
            [e, e, e],
            [-1, 0],
            (CubeFace::Top, e, e),
            (CubeFace::Right, 0, 0),
            (CubeFace::Front, 0, e),
        );

        // Add vertex information for edge pieces
        for i in 1..CUBE_SIZE as i32 - 1 {
            result.add_edge(
                [i, e, e],
                [-1, -1],
                (CubeFace::Top, e, i),
                (CubeFace::Front, 0, i),
            );
            result.add_edge(
                [0, e - i, e],
                [0, 2],
                (CubeFace::Front, i, 0),
                (CubeFace::Left, i, e),
            );
            result.add_edge(
                [e, e - i, e],
                [0, 0],
                (CubeFace::Front, i, e),
                (CubeFace::Right, i, 0),
            );
            result.add_edge(
                [i, 0, e],
                [1, 1],
                (CubeFace::Bottom, 0, i),
                (CubeFace::Front, e, i),
            );
            result.add_edge(
                [e, e, i],
                [-1, 0],
                (CubeFace::Top, i, e),
                (CubeFace::Right, 0, e - i),
            );
            result.add_edge(
                [e, e - i, 0],
                [2, 0],
                (CubeFace::Back, i, 0),
                (CubeFace::Right, i, e),
            );
            result.add_edge(
                [e, 0, i],
                [1, 0],
                (CubeFace::Bottom, i, e),
                (CubeFace::Right, e, i),
            );
            result.add_edge(
                [e - i, e, 0],
                [-1, 1],
                (CubeFace::Top, 0, e - i),
                (CubeFace::Back, 0, i),
            );
            result.add_edge(
                [e - i, 0, 0],
                [1, -1],
                (CubeFace::Bottom, e, e - i),
                (CubeFace::Back, e, i),
            );
            result.add_edge(
                [0, e, i],
                [-1, 2],
                (CubeFace::Top, i, 0),
                (CubeFace::Left, 0, i),
            );
            result.add_edge(
                [0, e - i, 0],
                [2, 2],
                (CubeFace::Back, i, e),
                (CubeFace::Left, i, 0),
            );
            result.add_edge(
                [0, 0, i],
                [1, 2],
                (CubeFace::Bottom, e - i, 0),
                (CubeFace::Left, e, i),
            );
        }

        // Add vertex information for center pieces
        for row in 1..CUBE_SIZE as i32 - 1 {
            for col in 1..CUBE_SIZE as i32 - 1 {
                result.add_center([col, e, row], [-1, 0, 0], (CubeFace::Top, row, col));
                result.add_center([col, e - row, e], [0, 0, 0], (CubeFace::Front, row, col));
                result.add_center(
                    [e, e - row, e - col],
                    [0, 1, 0],
                    (CubeFace::Right, row, col),
                );
                result.add_center([e - col, e - row, 0], [2, 0, 0], (CubeFace::Back, row, col));
                result.add_center([0, e - row, col], [0, -1, 0], (CubeFace::Left, row, col));
                result.add_center([col, 0, e - row], [1, 0, 0], (CubeFace::Bottom, row, col));
            }
        }

        // Set colors to cube state
        result.update_colors();

        result
    }

    pub fn reset_cube_state(&mut self) {
        self.set_cube_state(Cube3x3x3::new());
    }

    pub fn set_cube_state(&mut self, cube: Cube3x3x3) {
        self.cube = cube.clone();
        self.target_cube = cube;
        self.update_colors();
    }

    pub fn do_move(&mut self, mv: Move) {
        self.do_moves(&[mv]);
    }

    pub fn do_moves(&mut self, moves: &[Move]) {
        if self.max_queued_moves == 0 {
            for mv in moves {
                self.target_cube.do_move(*mv);
            }
            self.cube = self.target_cube.clone();
            self.move_queue.clear();
            self.animation = None;
            self.update_colors();
        } else {
            for mv in moves {
                self.target_cube.do_move(*mv);
                self.move_queue.push(*mv);
            }
        }
    }

    pub fn verify_state(&mut self, cube: Cube3x3x3) {
        if self.target_cube != cube {
            self.cube = cube.clone();
            self.target_cube = cube;
            self.move_queue.clear();
            self.animation = None;
            self.update_colors();
        }
    }

    pub fn cube_state(&self) -> Cube3x3x3 {
        self.target_cube.clone()
    }

    pub fn is_solved(&self) -> bool {
        self.target_cube.is_solved()
    }

    pub fn reset_angle(&mut self) {
        self.pitch = 30.0;
        self.yaw = -35.0;
    }

    pub fn adjust_angle(&mut self, dx: f32, dy: f32) {
        if (self.pitch > 90.0) && (self.pitch < 270.0) {
            self.yaw -= dx;
        } else {
            self.yaw += dx;
        }
        self.pitch += dy;

        self.yaw %= 360.0;
        self.pitch %= 360.0;
        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }
        if self.pitch < 0.0 {
            self.pitch += 360.0;
        }
    }

    fn vert_range(&mut self, piece: (CubeFace, i32, i32)) -> &mut VertexRange {
        &mut self.vert_ranges[(piece.0 as u8 as usize * CUBE_SIZE * CUBE_SIZE)
            + (piece.1 as usize * CUBE_SIZE)
            + piece.2 as usize]
    }

    fn add_src_model_verts(
        &mut self,
        rotation: &Mat4,
        model: &[SourceVertex],
        pos: &[i32; 3],
    ) -> usize {
        let start_index = self.verts.len();
        for src_vert in model {
            let mut rotated_vert_pos = [0.0; 3];
            let mut vert_normal = [0.0; 3];
            vec3::transform_mat4(&mut rotated_vert_pos, &src_vert.pos, &rotation);
            vec3::transform_mat4(&mut vert_normal, &src_vert.normal, &rotation);
            let mut vert_pos = [0.0; 3];
            vec3::add(
                &mut vert_pos,
                &rotated_vert_pos,
                &[
                    pos[0] as f32 * 2.0,
                    pos[1] as f32 * 2.0,
                    pos[2] as f32 * 2.0,
                ],
            );
            let (color, roughness) = if src_vert.color == -1 {
                (INNER_COLOR, INNER_ROUGHNESS)
            } else {
                (PLACEHOLDER_FACE_COLOR, FACE_ROUGHNESS)
            };
            self.verts.push(Vertex {
                pos: vert_pos,
                normal: vert_normal,
                color,
                roughness,
            });
        }
        start_index
    }

    fn add_corner(
        &mut self,
        pos: [i32; 3],
        rot: [i32; 2],
        first: (CubeFace, i32, i32),
        second: (CubeFace, i32, i32),
        third: (CubeFace, i32, i32),
    ) {
        // Compute rotation matrix for rotating piece into place
        let mut rotation: Mat4 = [0.0; 16];
        let rotation_ref = &mut rotation;
        mat4::from_rotation(
            rotation_ref,
            rot[0] as f32 * f32::PI() / 2.0,
            &[1.0, 0.0, 0.0],
        );
        mat4::rotate(
            rotation_ref,
            &mat4::clone(rotation_ref),
            rot[1] as f32 * f32::PI() / 2.0,
            &[0.0, 0.0, 1.0],
        );

        // Update vertex buffer for the corner piece
        let start_index = self.add_src_model_verts(&rotation, CORNER_SOURCE_VERTS, &pos);

        // Update index buffer for the corner piece
        for src_index in CORNER_INDEX {
            let index = *src_index + start_index as u16;
            self.index.push(index);
            self.anim_moving_index[first.0 as u8 as usize].push(index);
            self.anim_moving_index[second.0 as u8 as usize].push(index);
            self.anim_moving_index[third.0 as u8 as usize].push(index);
            for i in 0..6 {
                if first.0 as u8 != i && second.0 as u8 != i && third.0 as u8 != i {
                    self.anim_fixed_index[i as usize].push(index);
                }
            }
        }

        // Keep track of vertex information for each face
        *self.vert_range(first) = VertexRange {
            start_index,
            model_verts: CORNER_SOURCE_VERTS,
            face: 0,
        };
        *self.vert_range(second) = VertexRange {
            start_index,
            model_verts: CORNER_SOURCE_VERTS,
            face: 1,
        };
        *self.vert_range(third) = VertexRange {
            start_index,
            model_verts: CORNER_SOURCE_VERTS,
            face: 2,
        };
    }

    fn add_edge(
        &mut self,
        pos: [i32; 3],
        rot: [i32; 2],
        first: (CubeFace, i32, i32),
        second: (CubeFace, i32, i32),
    ) {
        // Compute rotation matrix for rotating piece into place
        let mut rotation: Mat4 = [0.0; 16];
        let rotation_ref = &mut rotation;
        mat4::from_rotation(
            rotation_ref,
            rot[0] as f32 * f32::PI() / 2.0,
            &[1.0, 0.0, 0.0],
        );
        mat4::rotate(
            rotation_ref,
            &mat4::clone(rotation_ref),
            rot[1] as f32 * f32::PI() / 2.0,
            &[0.0, 0.0, 1.0],
        );

        // Update vertex buffer for the edge piece
        let start_index = self.add_src_model_verts(&rotation, EDGE_SOURCE_VERTS, &pos);

        // Update index buffer for the edge piece
        for src_index in EDGE_INDEX {
            let index = *src_index + start_index as u16;
            self.index.push(index);
            self.anim_moving_index[first.0 as u8 as usize].push(index);
            self.anim_moving_index[second.0 as u8 as usize].push(index);
            for i in 0..6 {
                if first.0 as u8 != i && second.0 as u8 != i {
                    self.anim_fixed_index[i as usize].push(index);
                }
            }
        }

        // Keep track of vertex information for each face
        *self.vert_range(first) = VertexRange {
            start_index,
            model_verts: EDGE_SOURCE_VERTS,
            face: 0,
        };
        *self.vert_range(second) = VertexRange {
            start_index,
            model_verts: EDGE_SOURCE_VERTS,
            face: 1,
        };
    }

    fn add_center(&mut self, pos: [i32; 3], rot: [i32; 3], piece: (CubeFace, i32, i32)) {
        // Compute rotation matrix for rotating piece into place
        let mut rotation: Mat4 = [0.0; 16];
        let rotation_ref = &mut rotation;
        mat4::from_rotation(
            rotation_ref,
            rot[0] as f32 * f32::PI() / 2.0,
            &[1.0, 0.0, 0.0],
        );
        mat4::rotate(
            rotation_ref,
            &mat4::clone(rotation_ref),
            rot[1] as f32 * f32::PI() / 2.0,
            &[0.0, 1.0, 0.0],
        );
        mat4::rotate(
            rotation_ref,
            &mat4::clone(rotation_ref),
            rot[2] as f32 * f32::PI() / 2.0,
            &[0.0, 0.0, 1.0],
        );

        // Update vertex buffer for the center piece
        let start_index = self.add_src_model_verts(&rotation, CENTER_SOURCE_VERTS, &pos);

        // Update index buffer for the center piece
        for src_index in CENTER_INDEX {
            let index = *src_index + start_index as u16;
            self.index.push(index);
            self.anim_moving_index[piece.0 as u8 as usize].push(index);
            for i in 0..6 {
                if piece.0 as u8 != i {
                    self.anim_fixed_index[i as usize].push(index);
                }
            }
        }

        // Keep track of vertex information for each face
        *self.vert_range(piece) = VertexRange {
            start_index,
            model_verts: CENTER_SOURCE_VERTS,
            face: 0,
        };
    }

    fn update_colors(&mut self) {
        let face_colors = self.cube.as_faces();
        for face in &[
            CubeFace::Top,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Left,
            CubeFace::Bottom,
        ] {
            for row in 0..CUBE_SIZE {
                for col in 0..CUBE_SIZE {
                    let color = FACE_COLORS[face_colors.color(*face, row, col) as u8 as usize];
                    let range = self.vert_range((*face, row as i32, col as i32)).clone();
                    for i in 0..range.model_verts.len() {
                        if range.model_verts[i].color == range.face {
                            self.verts[range.start_index + i].color = color;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>, rect: &Rect) -> Result<()> {
        if self.renderer.is_none() {
            let mut renderer = GlRenderer::new(gl)?;
            renderer.set_camera_pos([0.0, 0.0, 5.0]);
            renderer.set_light([0.0, 2.0, 4.0], [40.0, 40.0, 40.0]);
            self.renderer = Some(renderer);
        }

        // Start animation if there are new moves and there isn't alreay an animation
        if self.animation.is_none() && self.move_queue.len() != 0 {
            while self.move_queue.len() > self.max_queued_moves {
                let mv = self.move_queue.remove(0);
                self.cube.do_move(mv);
            }

            let tps = 4.0 * self.move_queue.len() as f32;
            let mv = self.move_queue.remove(0);
            let face = mv.face();
            let angle = -90.0 * mv.rotation() as f32;

            let start = Instant::now();

            self.animation = Some(Animation {
                start,
                length: Duration::from_secs_f32(1.0 / tps),
                face,
                angle,
                move_: mv,
            });

            self.update_colors();
        }

        // Draw cube
        let renderer = self.renderer.as_mut().unwrap();
        renderer.begin(ctxt, gl, rect);

        // Set up fixed model matrix
        let mut model = [0.0; 16];
        let model_ref = &mut model;
        mat4::from_x_rotation(model_ref, to_radian(self.pitch));
        mat4::rotate_y(model_ref, &mat4::clone(model_ref), to_radian(self.yaw));
        mat4::scale(
            model_ref,
            &mat4::clone(model_ref),
            &[MODEL_SCALE, MODEL_SCALE, MODEL_SCALE],
        );
        mat4::translate(model_ref, &mat4::clone(model_ref), &MODEL_OFFSET);
        renderer.set_model_matrix(model);

        let mut anim_done = None;
        if let Some(animation) = &mut self.animation {
            // Animation present, calculate fraction complete
            let now = Instant::now();
            let elapsed = (now - animation.start).as_secs_f32();
            let frac = elapsed / animation.length.as_secs_f32();
            let frac = if frac >= 1.0 {
                anim_done = Some(animation.move_);
                1.0
            } else {
                (frac * f32::PI() / 2.0).sin()
            };

            // Calculate axis and angle for animation
            let angle = animation.angle * frac;
            let axis = match animation.face {
                CubeFace::Top => [0.0, 1.0, 0.0],
                CubeFace::Front => [0.0, 0.0, 1.0],
                CubeFace::Right => [1.0, 0.0, 0.0],
                CubeFace::Back => [0.0, 0.0, -1.0],
                CubeFace::Left => [-1.0, 0.0, 0.0],
                CubeFace::Bottom => [0.0, -1.0, 0.0],
            };

            // Draw fixed part of the cube
            renderer.draw(
                gl,
                &self.verts,
                &self.anim_fixed_index[animation.face as u8 as usize],
            )?;

            // Compute model matrix of the moving part of the cube
            let mut model = [0.0; 16];
            let model_ref = &mut model;
            mat4::from_x_rotation(model_ref, to_radian(self.pitch));
            mat4::rotate_y(model_ref, &mat4::clone(model_ref), to_radian(self.yaw));
            mat4::rotate(model_ref, &mat4::clone(model_ref), to_radian(angle), &axis);
            mat4::scale(
                model_ref,
                &mat4::clone(model_ref),
                &[MODEL_SCALE, MODEL_SCALE, MODEL_SCALE],
            );
            mat4::translate(model_ref, &mat4::clone(model_ref), &MODEL_OFFSET);
            renderer.set_model_matrix(model);

            // Draw the moving part of the cube
            renderer.draw(
                gl,
                &self.verts,
                &self.anim_moving_index[animation.face as u8 as usize],
            )?;
        } else {
            // No animation, draw fixed model
            renderer.draw(gl, &self.verts, &self.index)?;
        }

        renderer.end(gl);

        // If there is a finished animation, transition to fixed state and apply the
        // animated move to the rendered cube state.
        if let Some(mv) = anim_done {
            self.cube.do_move(mv);
            self.animation = None;
            self.update_colors();
        }

        Ok(())
    }

    pub fn animating(&self) -> bool {
        self.animation.is_some() || self.move_queue.len() != 0
    }
}
