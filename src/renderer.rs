use std::marker::PhantomData;
use glium::{Display, Program, DrawParameters, Frame, Surface};
use mesh::{Mesh, Polygon};
use shader;
use shader::Shader;
use math::Matrix;
use utils::Ref;


pub struct Renderer<'display, S=shader::Default>
    where S: Shader
{
    pub display: &'display Display,
    program: Program,
    params: DrawParameters<'display>,
    _mark: PhantomData<S>,
}



impl<'display, S: Shader> Renderer<'display, S> {
    pub fn new(display: &'display Display) -> Renderer<'display, S> {
        let program = S::program(display);
        Renderer {
            display: display,
            program: program,
            params: Renderer::<S>::build_params(),
            _mark: PhantomData,
        }
    }

    pub fn draw<P>(&self, target: &mut Frame, polygon: &P, uniforms: &S::Uniforms)
        where P: Polygon<S::Vertex>
    {
        let &Mesh(ref vertex_buffer, ref index_buffer) = &*polygon.mesh(self.display);
        target.draw(
            vertex_buffer,
            index_buffer,
            &self.program,
            uniforms,
            &self.params
        ).unwrap();
    }

    pub fn render<R>(&self, target: &mut Frame, parent: &Matrix, renderable: &R)
        where R: Renderable<S>
    {
        let uniforms = renderable.uniforms(parent);
        self.draw(target, renderable, &*uniforms);
    }

    fn build_params<'a>() -> DrawParameters<'a> {
        DrawParameters {
            blend: ::glium::Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }
}

pub trait Renderable<S: Shader>: Polygon<S::Vertex> {
    fn uniforms<'a>(&'a self, parent: &Matrix) -> Ref<'a, S::Uniforms>;
}
