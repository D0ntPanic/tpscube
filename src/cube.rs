use crate::gl::{GlContext, GlRenderer, Vertex};
use anyhow::Result;
use egui::{CtxRef, Rect};

pub struct CubeRenderer {
    renderer: Option<GlRenderer>,
}

pub struct SourceVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub color: i32,
}

impl CubeRenderer {
    pub fn new() -> Self {
        Self { renderer: None }
    }

    pub fn draw(&mut self, ctxt: &CtxRef, gl: &mut GlContext<'_, '_>, rect: &Rect) -> Result<()> {
        if self.renderer.is_none() {
            self.renderer = Some(GlRenderer::new(gl)?);
        }

        let renderer = self.renderer.as_mut().unwrap();

        let verts = vec![
            Vertex {
                pos: [-1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 1.0, 1.0],
                roughness: 0.0,
            },
            Vertex {
                pos: [1.0, -1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 1.0, 1.0],
                roughness: 0.0,
            },
            Vertex {
                pos: [1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 1.0, 1.0],
                roughness: 0.0,
            },
            Vertex {
                pos: [-1.0, 1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                color: [1.0, 1.0, 1.0],
                roughness: 0.0,
            },
        ];

        renderer.set_camera_pos([0.0, 0.0, 5.0]);
        renderer.set_light([0.0, 2.0, 4.0], [40.0, 40.0, 40.0]);

        let mut model = [0.0; 16];
        gl_matrix::mat4::from_y_rotation(&mut model, gl_matrix::common::to_radian(0.0));
        renderer.set_model_matrix(model);

        renderer.begin(ctxt, gl, rect);
        renderer.draw(gl, &verts, &[0, 1, 2, 0, 2, 3])?;
        renderer.end(gl);

        Ok(())
    }
}
