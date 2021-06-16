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
use num_traits::FloatConst;
use tpscube_core::{Cube, Cube3x3x3, Face};

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
    pitch: f32,
    yaw: f32,
    verts: Vec<Vertex>,
    index: Vec<u16>,
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

impl CubeRenderer {
    pub fn new() -> Self {
        let mut result = Self {
            renderer: None,
            cube: Cube3x3x3::new(),
            pitch: 30.0,
            yaw: -35.0,
            verts: Vec::new(),
            index: Vec::new(),
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
            (Face::Bottom, e, 0),
            (Face::Back, e, e),
            (Face::Left, e, 0),
        );
        result.add_corner(
            [e, 0, 0],
            [1, 0],
            (Face::Bottom, e, e),
            (Face::Right, e, e),
            (Face::Back, e, 0),
        );
        result.add_corner(
            [0, e, 0],
            [-1, 2],
            (Face::Top, 0, 0),
            (Face::Left, 0, 0),
            (Face::Back, 0, e),
        );
        result.add_corner(
            [e, e, 0],
            [-1, 1],
            (Face::Top, 0, e),
            (Face::Back, 0, 0),
            (Face::Right, 0, e),
        );
        result.add_corner(
            [0, 0, e],
            [1, 2],
            (Face::Bottom, 0, 0),
            (Face::Left, e, e),
            (Face::Front, e, 0),
        );
        result.add_corner(
            [e, 0, e],
            [1, 1],
            (Face::Bottom, 0, e),
            (Face::Front, e, e),
            (Face::Right, e, 0),
        );
        result.add_corner(
            [0, e, e],
            [-1, -1],
            (Face::Top, e, 0),
            (Face::Front, 0, 0),
            (Face::Left, 0, e),
        );
        result.add_corner(
            [e, e, e],
            [-1, 0],
            (Face::Top, e, e),
            (Face::Right, 0, 0),
            (Face::Front, 0, e),
        );

        // Add vertex information for edge pieces
        for i in 1..CUBE_SIZE as i32 - 1 {
            result.add_edge([i, e, e], [-1, -1], (Face::Top, e, i), (Face::Front, 0, i));
            result.add_edge(
                [0, e - i, e],
                [0, 2],
                (Face::Front, i, 0),
                (Face::Left, i, e),
            );
            result.add_edge(
                [e, e - i, e],
                [0, 0],
                (Face::Front, i, e),
                (Face::Right, i, 0),
            );
            result.add_edge([i, 0, e], [1, 1], (Face::Bottom, 0, i), (Face::Front, e, i));
            result.add_edge(
                [e, e, i],
                [-1, 0],
                (Face::Top, i, e),
                (Face::Right, 0, e - i),
            );
            result.add_edge(
                [e, e - i, 0],
                [2, 0],
                (Face::Back, i, 0),
                (Face::Right, i, e),
            );
            result.add_edge([e, 0, i], [1, 0], (Face::Bottom, i, e), (Face::Right, e, i));
            result.add_edge(
                [e - i, e, 0],
                [-1, 1],
                (Face::Top, 0, e - i),
                (Face::Back, 0, i),
            );
            result.add_edge(
                [e - i, 0, 0],
                [1, -1],
                (Face::Bottom, e, e - i),
                (Face::Back, e, i),
            );
            result.add_edge([0, e, i], [-1, 2], (Face::Top, i, 0), (Face::Left, 0, i));
            result.add_edge(
                [0, e - i, 0],
                [2, 2],
                (Face::Back, i, e),
                (Face::Left, i, 0),
            );
            result.add_edge(
                [0, 0, i],
                [1, 2],
                (Face::Bottom, e - i, 0),
                (Face::Left, e, i),
            );
        }

        // Add vertex information for center pieces
        for row in 1..CUBE_SIZE as i32 - 1 {
            for col in 1..CUBE_SIZE as i32 - 1 {
                result.add_center([col, e, row], [-1, 0, 0], (Face::Top, row, col));
                result.add_center([col, e - row, e], [0, 0, 0], (Face::Front, row, col));
                result.add_center([e, e - row, e - col], [0, 1, 0], (Face::Right, row, col));
                result.add_center([e - col, e - row, 0], [2, 0, 0], (Face::Back, row, col));
                result.add_center([0, e - row, col], [0, -1, 0], (Face::Left, row, col));
                result.add_center([col, 0, e - row], [1, 0, 0], (Face::Bottom, row, col));
            }
        }

        // Set colors to cube state
        result.update_colors();

        result
    }

    pub fn set_cube_state(&mut self, cube: Cube3x3x3) {
        self.cube = cube;
        self.update_colors();
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

    fn vert_range(&mut self, piece: (Face, i32, i32)) -> &mut VertexRange {
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
        first: (Face, i32, i32),
        second: (Face, i32, i32),
        third: (Face, i32, i32),
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
        first: (Face, i32, i32),
        second: (Face, i32, i32),
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

    fn add_center(&mut self, pos: [i32; 3], rot: [i32; 3], piece: (Face, i32, i32)) {
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
        for face in [
            Face::Top,
            Face::Front,
            Face::Right,
            Face::Back,
            Face::Left,
            Face::Bottom,
        ] {
            for row in 0..CUBE_SIZE {
                for col in 0..CUBE_SIZE {
                    let color = FACE_COLORS[face_colors.color(face, row, col) as u8 as usize];
                    let range = self.vert_range((face, row as i32, col as i32)).clone();
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

        let renderer = self.renderer.as_mut().unwrap();

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

        renderer.begin(ctxt, gl, rect);
        renderer.draw(gl, &self.verts, &self.index)?;
        renderer.end(gl);

        Ok(())
    }
}
