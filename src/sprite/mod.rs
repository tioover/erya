use glium::Display;
use num::NumCast;
use texture::TextureRef;
use mesh::{ Mesh, Polygon };
use transform::Transform;
use rect::Rect;
use utils::cast;
use renderer::Renderable;
use cgmath::Matrix4;
use id::Id;


#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub texture_position: [u32; 2],
}


implement_vertex!{ Vertex, position, texture_position }


pub struct Shader;


impl ::shader::Shader for Shader
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


pub struct Uniforms
{
    pub matrix: [[f32; 4]; 4],
    pub opacity: f32,
    pub image: TextureRef,
}


implement_uniforms! { Uniforms, matrix, opacity, image }


pub struct Sprite<'display>
{
    pub id: Id,
    display: &'display Display,
    texture: TextureRef,
    width: f32,
    height: f32,
    opacity: f32,
    rect: Rect,
    mesh: Mesh<Vertex>,
    pub transform: Transform,
}


impl<'display> Sprite<'display>
{
    pub fn new<N>(display: &'display Display, tex: TextureRef, width: N, height: N)
        -> Sprite<'display>
        where N: NumCast + Clone
    {
        let rect = Rect::new(0, 0, tex.width, tex.height);
        Sprite::with_rect(display, tex, rect, width, height)
    }

    pub fn with_rect<N>(display: &'display Display, tex: TextureRef,
                        rect: Rect, width: N, height: N) -> Sprite<'display>
        where N: NumCast + Clone
    {
        Sprite
        {
            id: Id::new(),
            texture: tex,
            width: cast(width.clone()),
            height: cast(height.clone()),
            transform: Transform::new(),
            opacity: 1.0,
            mesh: Sprite::build_mesh(display, cast(width), cast(height), &rect),
            rect: rect,
            display: display,
        }
    }

    fn build_mesh(display: &Display, width: f32, height: f32, rect: &Rect)
        -> Mesh<Vertex>
    {
        let x = rect.x;
        let y = rect.y;
        let w = rect.width;
        let h = rect.height;
        let verties = [
            Vertex { position: [  0.0, height], texture_position: [x, y+h] },
            Vertex { position: [  0.0,    0.0], texture_position: [x, y] },
            Vertex { position: [width,    0.0], texture_position: [x+w, y] },
            Vertex { position: [width, height], texture_position: [x+w,y+h] },
        ];

        Mesh::with_indices(display, &verties, &[0, 1, 2, 2, 3, 0])
    }

    pub fn resize(&mut self, width: f32, height: f32)
    {
        self.mesh = Sprite::build_mesh(self.display, width, height, &self.rect);
    }

    pub fn rect(&mut self, rect: Rect)
    {
        self.mesh = Sprite::build_mesh(self.display, self.width, self.height, &rect);
        self.rect = rect;
    }
}


impl<'display> Polygon<Vertex> for Sprite<'display>
{
    fn mesh(&self) -> &Mesh<Vertex>
    {
        &self.mesh
    }
}



impl<'display> Renderable<Shader> for Sprite<'display>
{
    fn uniforms(&self, parent: &Matrix4<f32>) -> Uniforms
    {
        Uniforms
        {
            image: self.texture.clone(),
            opacity: self.opacity,
            matrix: (parent * self.transform.matrix()).into(),
        }
    }
}


