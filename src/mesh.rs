use glium;
use glium::Display;
use rect::Rect;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}


implement_vertex!(Vertex, position, tex_coords);


pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;


pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}


impl Mesh {
    pub fn picture(display: &Display, rect: Rect) -> Mesh {
        let (w, h) = display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let factor = display.get_window().unwrap().hidpi_factor();

        let f = |x: f32, y: f32| -> [f32; 2] {
            [x * factor / (w / 2.0) - 1.0, (2.0 - y * factor / (h / 2.0)) - 1.0]
        };

        let x = rect.x;
        let y = rect.y;
        let w = rect.width;
        let h = rect.height;
        let a = x+w;
        let b = y+h;

        let verties = [
            Vertex { position: f(x, b), tex_coords: [0.0, 0.0] },
            Vertex { position: f(x, y), tex_coords: [0.0, 1.0] },
            Vertex { position: f(a, y), tex_coords: [1.0, 1.0] },
            Vertex { position: f(a, b), tex_coords: [1.0, 0.0] },
        ];
        let index = glium::index::PrimitiveType::TriangleStrip;
        Mesh {
            vertex_buffer: VertexBuffer::new(display, &verties).unwrap(),
            index_buffer: IndexBuffer::new(display, index, &[1, 2, 0, 3]).unwrap(),
        }
    }
}


pub trait Polygon {
    fn mesh(&self, &Display) -> Mesh;
}


