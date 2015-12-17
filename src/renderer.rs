use std::marker::PhantomData;
use glium::{Display, Program, DrawParameters, Frame, Surface};
use mesh::{Mesh, Polygon};
use shader;
use shader::Shader;


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

    pub fn draw<P>(&self, target: &mut Frame, polygon: &P, uniforms: &S)
        where P: Polygon<S::Vertex>
    {
        let &Mesh(ref vb, ref ib) = &*polygon.mesh(&self.display);
        target.draw(vb, ib, &self.program, uniforms, &self.params).unwrap();
    }

    fn build_params<'a>() -> DrawParameters<'a> {
        DrawParameters {
            blend: ::glium::Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }
}
