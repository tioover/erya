extern crate erya;
#[macro_use]
extern crate glium;

use std::path::PathBuf;
use std::time::Duration;
use glium::Surface;
use glium::glutin::{Event, ElementState, VirtualKeyCode};
use erya::na;
use erya::renderer::Renderer;
use erya::resource::Manager;
use erya::texture::Texture;
use erya::mesh::Mesh;
use erya::rect::Rect;
use erya::math::Matrix;


fn main() {
    let display = erya::build_display("Image", (800, 600));
    let renderer = Renderer::new(&display);
    let textures = Manager::<Texture>::new(&display, PathBuf::from("examples/assets"));
    let mesh = Mesh::picture(&display, Rect::new(0., 0., 512., 512.));
    let tex = &textures.get("block.png").data;
    'main: loop {

        for event in display.poll_events() {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            renderer.draw(&mut target, &mesh, &uniform!{
                matrix: na::one::<Matrix>(),
                color_multiply: na::one::<Matrix>(),
                tex: tex,
            });
            target.finish().unwrap();

            match event {
                Event::Closed | Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => break 'main,
                _ => (),
            }
        }
        ::std::thread::sleep(Duration::from_millis(1));
    }
}
