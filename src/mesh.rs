//! 3D mesh object.

use glium;
use glium::Display;
use glium::index::{ PrimitiveType, NoIndices };
use either::Either;

pub use glium::VertexBuffer;
pub use glium::Vertex as VertexType;


pub type Index = u16;
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
            indices: Either::Right(NoIndices(PrimitiveType::TrianglesList)),
        }
    }


    pub fn with_indices(display: &Display, verties: &[T], index: &[Index]) -> Mesh<T>
    {
        Mesh
        {
            verties: VertexBuffer::new(display, verties).unwrap(),
            indices: Either::Left(IndexBuffer::new(display, PrimitiveType::TrianglesList, index).unwrap()),
        }
    }
}


pub trait Polygon<T: VertexType>
{
    fn mesh(&self) -> &Mesh<T>;
}


impl<T> Polygon<T> for Mesh<T>
    where T: VertexType
{
    fn mesh(&self) -> &Mesh<T>
    {
        self
    }
}

