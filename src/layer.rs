//! Renderable object manager.

use glium::Frame;
use cgmath::Matrix4;
use id::Id;
use renderer::{ Renderable, Renderer };
use transform::Transform;
use shader::ShaderType;


pub struct Layer<S: ShaderType>
{
    pub id: Id,
    pub layers: Vec<Layer<S>>,
    pub objects: Vec<Box<Renderable<S>>>,
    pub transform: Option<Transform>,
}


impl<S: ShaderType> Layer<S>
{
    pub fn new() -> Layer<S>
    {
        Layer
        {
            id: Id::new(),
            layers: Vec::new(),
            objects: Vec::new(),
            transform: None,
        }
    }
}


impl<S: ShaderType> Renderable<S> for Layer<S>
{

    fn draw(&self, renderer: &Renderer<S>, target: &mut Frame, parent: &Matrix4<f32>)
    {
        if let Some(ref transform) = self.transform
        {
            let mat = parent * transform.matrix();
            for x in &self.layers { x.draw(renderer, target, &mat) }
            for x in &self.objects { x.draw(renderer, target, &mat) }
        }
        else {
            for x in &self.layers { x.draw(renderer, target, parent) }
            for x in &self.objects { x.draw(renderer, target, parent) }
        };
    }
}


