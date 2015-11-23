extern crate erya;
#[macro_use]
extern crate glium;

use std::path::PathBuf;
use glium::Surface;
use glium::glutin::Event;
use erya::na;
use erya::renderer::Renderer;
use erya::resource::Manager;
use erya::texture::Texture;
use erya::mesh::{Mesh, Vertex, VertexBuffer, IndexBuffer};


fn main() {
    let display = erya::build_display("Image", (800, 600));
    let renderer = Renderer::new(&display);
    let textures = Manager::<Texture>::new(&display, PathBuf::from("examples/assets"));
    let verties = [
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
    ];
    let mesh = Mesh {
        vertex_buffer: VertexBuffer::new(&display, &verties).unwrap(),
        index_buffer: IndexBuffer::new(&display,
            glium::index::PrimitiveType::TriangleStrip, &[1, 2, 0, 3]).unwrap(),
    };
    let tex = &textures.get("block.png").data;
    'main: loop {

        for event in display.poll_events() {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            renderer.draw(&mut target, &mesh, &uniform!{
                matrix: na::one::<na::Mat4<f32>>(),
                color_multiply: na::one::<na::Vec4<f32>>(),
                tex: tex,
            });
            target.finish().unwrap();

            match event {
                Event::Closed => break 'main,
                _ => (),
            }
        }
    }
}
