extern crate erya;
#[macro_use]
extern crate glium;

use std::path::PathBuf;
use glium::Surface;
use erya::renderer::Renderer;
use erya::resource::Manager;
use erya::texture::Texture;
use erya::mesh::Mesh;
use erya::rect::Rect;
use erya::camera::Camera;


fn main() {
    let display = erya::build_display("Image", (800, 600));
    let camera = Camera::new(&display);
    let renderer = Renderer::new(&display);
    let textures = Manager::<Texture>::new(&display, PathBuf::from("examples/assets"));
    let tex = &textures.get("block.png").data;

    'main: loop {

        let mesh = Mesh::picture(&display, Rect::new(0.0, 0.0, 128.0, 128.0));
        let mut target = display.draw();
        target.clear_color(0.25, 0.25, 0.25, 0.0);
        renderer.draw(&mut target, &mesh, &uniform!{
            matrix: *camera.matrix().as_ref(),
            color_multiply: [1.0, 1.0, 1.0, 1.0],
            tex: tex,
        });
        target.finish().unwrap();
        for event in display.poll_events() {
            use glium::glutin::Event;
            use glium::glutin::ElementState::*;
            use glium::glutin::VirtualKeyCode::*;

            match event {
                Event::Closed|Event::KeyboardInput(Released, _, Some(Escape)) => break 'main,
                _ => (),
            }
        }
        erya::timer::sleep_ms(1);
    }
}
