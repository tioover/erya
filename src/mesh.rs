use glium;
use glium::Display;
use utils::Ref;


pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}


implement_vertex!(Vertex, position, tex_coords);


pub struct Mesh (pub VertexBuffer, pub IndexBuffer);


pub trait Polygon {
    fn mesh<'a>(&'a self, &Display) -> Ref<'a, Mesh>;
}


impl Polygon for Mesh {
    fn mesh<'a>(&'a self, _: &Display) -> Ref<'a, Mesh> {
        Ref::Borrowed(self)
    }
}

