//! Draw the renderable object.

use std::marker::PhantomData;
use cgmath::Matrix4;
use glium::{ Display, Program, DrawParameters, Frame, Surface };
use shader::Shader;
use model::Model;


/// Render context object.
pub struct Renderer<'display, S>
    where S: Shader
{
    pub display: &'display Display,
    program: Program,
    params: DrawParameters<'static>,
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
    pub fn draw<T>(&self, target: &mut Frame, model: &T, parent: &Matrix4<f32>)
        where T: Model<S>
    {
        use glium::index::IndicesSource;
        use either::{ Left, Right };

        let mesh = model.mesh();
        let uniforms = model.uniforms(parent);
        let indices: IndicesSource = match mesh.indices
        {
            Left(ref x) => x.into(),
            Right(ref x) => x.into(),
        };
        target.draw(
            &mesh.verties,
            indices,
            &self.program,
            &uniforms,
            &self.params
        ).unwrap();
    }

    fn build_params() -> DrawParameters<'static>
    {
        DrawParameters
        {
            blend: ::glium::Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }
}


pub trait Renderable<S: Shader>
{
    /// Draw object to target with parent matrix.
    fn draw(&self, renderer: &Renderer<S>, target: &mut Frame, parent: &Matrix4<f32>);
}


impl<S, T> Renderable<S> for T
    where S: Shader, T: Model<S>
{
    fn draw(&self, renderer: &Renderer<S>, target: &mut Frame, parent: &Matrix4<f32>)
    {
        renderer.draw(target, self, parent);
    }
}
