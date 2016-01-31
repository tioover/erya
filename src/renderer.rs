//! Draw the renderable object.

use std::marker::PhantomData;
use cgmath::Matrix4;
use glium::{ Display, Program, DrawParameters, Frame, Surface };
use shader::Shader;
use mesh::Polygon;


/// Render context object.
pub struct Renderer<'display, S>
    where S: Shader
{
    pub display: &'display Display,
    program: Program,
    params: DrawParameters<'display>,
    _mark: PhantomData<S>,
}



impl<'display, S: Shader> Renderer<'display, S>
{
    /// Create default renderer.
    pub fn new(display: &'display Display) -> Renderer<'display, S>
    {
        let program = S::program(display);
        Renderer
        {
            display: display,
            program: program,
            params: Renderer::<S>::build_params(),
            _mark: PhantomData,
        }
    }

    /// Draw mesh with uniforms.
    pub fn draw<P>(&self, target: &mut Frame, polygon: &P, uniforms: &S::Uniforms)
        where P: Polygon<S::Vertex>
    {
        use glium::index::IndicesSource;
        use either::{ Left, Right };

        let mesh = polygon.as_ref();
        let indices: IndicesSource = match mesh.indices
        {
            Left(ref x) => x.into(),
            Right(ref x) => x.into(),
        };
        target.draw(
            &mesh.verties,
            indices,
            &self.program,
            uniforms,
            &self.params
        ).unwrap();
    }

    /// Render renderable object with parent matrix.
    pub fn render<R>(&self, target: &mut Frame, parent: &Matrix4<f32>, renderable: &R)
        where R: Renderable<S>
    {
        let uniforms = renderable.uniforms(parent);
        self.draw(target, renderable, &uniforms);
    }

    fn build_params<'a>() -> DrawParameters<'a>
    {
        DrawParameters
        {
            blend: ::glium::Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }
}


pub trait Renderable<S: Shader>: Polygon<S::Vertex>
{
    fn uniforms(&self, parent: &Matrix4<f32>) -> S::Uniforms;
}
