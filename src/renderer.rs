use glium::{Display, Program, DrawParameters, Frame, Surface};
use glium::uniforms::Uniforms;
use mesh::Mesh;

pub struct Renderer<'display> {
    pub display: &'display Display,
    program: Program,
    params: DrawParameters<'display>,
}



impl<'display> Renderer<'display> {
    pub fn new(display: &'display Display) -> Renderer<'display> {
        let vert = include_str!("shader/140/default.vert");
        let frag = include_str!("shader/140/default.frag");
        Renderer::with_shader(display, vert, frag)
    }

    pub fn with_shader( display: &'display Display,
                         vertex: &str,
                       fragment: &str)
                       -> Renderer<'display> {
        let program = program!(display,
            140 => {
                vertex: vertex,
                fragment: fragment,
            },
        ).unwrap();
        Renderer {
            display: display,
            program: program,
            params: Renderer::build_params(),
        }
    }

    pub fn draw<U>(&self, target: &mut Frame, mesh: &Mesh, uniforms: &U)
        where U: Uniforms
    {
        target.draw(
            &mesh.vertex_buffer,
            &mesh.index_buffer,
            &self.program,
            uniforms,
            &self.params
        ).unwrap();
    }

    fn build_params<'a>() -> DrawParameters<'a> {
        use glium::Blend;
        use std::default::Default;

        DrawParameters { blend: Blend::alpha_blending(), ..Default::default() }
    }
}
