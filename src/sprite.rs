use glium::Display;
use num::NumCast;
use texture::TextureRef;
use mesh::{Mesh, Vertex, Polygon};
use transform::Transform;
use rect::Rect;
use utils::Ref;
use renderer::Renderable;
use shader;
use math::Matrix;
use id::Id;


pub struct Sprite {
    pub id: Id,
    texture: TextureRef,
    width: f32,
    height: f32,
    rect: Rect,
    pub transform: Transform,
}


impl Sprite {
    pub fn new<N: NumCast>(tex: TextureRef, width: N, height: N) -> Sprite {
        let rect = Rect::new(0, 0, tex.width, tex.height);
        Sprite::with_rect(tex, rect, width, height)
    }

    pub fn with_rect<N: NumCast>(tex: TextureRef, rect: Rect, width: N, height: N)
        -> Sprite
    {
        Sprite {
            id: Id::new(),
            texture: tex,
            rect: rect,
            width: from!(width),
            height: from!(height),
            transform: Transform::new(),
        }
    }
}


impl Polygon<Vertex> for Sprite {
    fn mesh<'a>(&'a self, display: &Display) -> Ref<'a, Mesh<Vertex>> {
        use glium::index::PrimitiveType::TriangleStrip;
        use mesh::{VertexBuffer, IndexBuffer};

        let a = self.width;
        let b = self.height;

        let width = self.texture.width as f32;
        let height = self.texture.height as f32;


        let p = self.rect.x / width;
        let q = self.rect.y / height;
        let r = (self.rect.x + self.rect.width) / width;
        let s = (self.rect.y + self.rect.height) / height;

        let verties = [
            Vertex { position: [0.0,   b], tex_coords: [p, 1.0-s] },
            Vertex { position: [0.0, 0.0], tex_coords: [p, 1.0-q] },
            Vertex { position: [  a, 0.0], tex_coords: [r, 1.0-q] },
            Vertex { position: [  a,   b], tex_coords: [r, 1.0-s] },
        ];
        let index = [1, 2, 0, 3];
        Ref::Owned(
            Mesh (
                VertexBuffer::new(display, &verties).unwrap(),
                IndexBuffer::new(display, TriangleStrip, &index).unwrap(),
            )
        )
    }
}



impl Renderable<shader::Default> for Sprite {
    fn uniforms<'a>(&'a self, parent: &Matrix)
        -> Ref<'a, shader::DefaultUniforms>
    {
        Ref::Owned(
            shader::DefaultUniforms {
                tex: self.texture.clone(),
                matrix: (*parent * self.transform.matrix()).as_ref().clone(),
            }
        )
    }
}


