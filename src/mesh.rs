use glium;
use glium::Display;
use glium::index::PrimitiveType::TrianglesList;
use glium::index::NoIndices;
use either::Either;

pub use glium::VertexBuffer;
pub use glium::Vertex as VertexType;


/// Vertex index data type.
pub type Index = u16;
/// A list of indices loaded in the graphics card's memory.
pub type IndexBuffer = glium::IndexBuffer<Index>;


/// Mesh type, inclusive of vertex buffer and index buffer.
pub struct Mesh<T: VertexType>
{
    pub verties: VertexBuffer<T>,
    pub indices: Either<IndexBuffer, NoIndices>,
}


impl<T: VertexType> Mesh<T>
{
    pub fn new(display: &Display, verties: &[T]) -> Mesh<T>
    {
        Mesh
        {
            verties: VertexBuffer::new(display, verties).unwrap(),
            indices: Either::Right(NoIndices(TrianglesList)),
        }
    }


    pub fn with_indices(display: &Display, verties: &[T], index: &[Index]) -> Mesh<T>
    {
        let buffer = IndexBuffer::new(display, TrianglesList, index).unwrap();
        Mesh
        {
            verties: VertexBuffer::new(display, verties).unwrap(),
            indices: Either::Left(buffer),
        }
    }
}
