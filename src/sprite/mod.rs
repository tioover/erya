//! Basic 2D object and shader.

use std::rc::Rc;
use glium::{ Display, Frame };
use texture::{ Texture, TextureRef };
use mesh::Mesh;
use shader::ShaderType;
use transform::Transform;
use renderer::{ Renderer, Renderable };
use rect::Rect;
use cgmath::Matrix4;
use id::Id;


/// 2D Sprite vertex type
#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub texture_position: [f32; 2],
}


implement_vertex!{ Vertex, position, texture_position }


/// Simple sprite shader
pub struct Shader;


impl ShaderType for Shader
{
    type Vertex = Vertex;
    type Uniforms = Uniforms;

    fn vertex() -> &'static str
    {
        include_str!("sprite.vert")
    }

    fn fragment() -> &'static str
    {
        include_str!("sprite.frag")
    }
}


/// Sprite rendering uniforms.
pub struct Uniforms
{
    pub matrix: [[f32; 4]; 4],
    pub opacity: f32,
    pub image: TextureRef,
}


implement_uniforms! { Uniforms, matrix, opacity, image }


/// Basic 2D object.
pub struct Sprite
{
    pub id: Id,
    texture: TextureRef,
    width: f32,
    height: f32,
    opacity: f32,
    rect: Rect,
    mesh: Mesh<Vertex>,
    pub transform: Transform,
}


impl Sprite
{
    pub fn new(display: &Display, tex: Rc<Texture>, width: u32, height: u32)
        -> Sprite
    {
        let rect = Rect::new(0, 0, tex.width as i32, tex.height as i32);
        Sprite::with_rect(display, tex, rect, width, height)
    }

    /// Create a sprite with size and
    /// [texture atlas](https://en.wikipedia.org/wiki/Texture_atlas).
    pub fn with_rect(display: &Display, tex: Rc<Texture>,
                     rect: Rect, width: u32, height: u32)
        -> Sprite
    {
        let (w, h) = (width as f32, height as f32);
        Sprite
        {
            id: Id::new(),
            texture: TextureRef(tex),
            width: w,
            height: h,
            transform: Transform::new(),
            opacity: 1.0,
            mesh: Sprite::build_mesh(display, w, h, &rect),
            rect: rect,
        }
    }

    fn build_mesh(display: &Display, width: f32, height: f32, rect: &Rect)
        -> Mesh<Vertex>
    {
        let x = rect.x as f32;
        let y = rect.y as f32;
        let w = rect.width as f32;
        let h = rect.height as f32;
        let verties = [
            Vertex { position: [  0.0, height], texture_position: [  x, y+h] },
            Vertex { position: [  0.0,    0.0], texture_position: [  x,   y] },
            Vertex { position: [width,    0.0], texture_position: [x+w,   y] },
            Vertex { position: [width, height], texture_position: [x+w, y+h] },
        ];

        Mesh::with_indices(display, &verties, &[0, 1, 2, 2, 3, 0])
    }

    pub fn resize(&mut self, display: &Display, width: f32, height: f32)
    {
        self.mesh = Sprite::build_mesh(display, width, height, &self.rect);
    }

    /// Change the texture atlas rectangle.
    pub fn rect(&mut self, display: &Display, rect: Rect)
    {
        self.mesh = Sprite::build_mesh(display, self.width, self.height, &rect);
        self.rect = rect;
    }
}


impl Renderable<Shader> for Sprite
{
    fn draw(&self, renderer: &Renderer<Shader>, target: &mut Frame, parent: &Matrix4<f32>)
    {
        let uniforms = Uniforms
        {
            image: self.texture.clone(),
            opacity: self.opacity,
            matrix: (parent * self.transform.matrix()).into(),
        };
        renderer.draw(target, &self.mesh, &uniforms);
    }
}

