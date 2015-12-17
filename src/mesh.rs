use glium;
use glium::Display;
use utils::Ref;

pub use glium::VertexBuffer;
pub use glium::Vertex as VertexType;


pub type IndexBuffer = glium::IndexBuffer<u16>;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}


implement_vertex!(Vertex, position, tex_coords);


pub struct Mesh<V: VertexType> (pub VertexBuffer<V>, pub IndexBuffer);


pub trait Polygon<V: VertexType> {
    fn mesh<'a>(&'a self, &Display) -> Ref<'a, Mesh<V>>;
}


impl<V> Polygon<V> for Mesh<V>
    where V: VertexType
{
    fn mesh<'a>(&'a self, _: &Display) -> Ref<'a, Mesh<V>> {
        Ref::Borrowed(self)
    }
}

