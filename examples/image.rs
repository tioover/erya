extern crate erya;
#[macro_use]
extern crate glium;


use std::path::PathBuf;
use glium::glutin::Event;
use glium::Surface;
use erya::renderer::Renderer;
use erya::loader::{Queue, RecvState};
use erya::texture::Texture;
use erya::camera::Camera2D;
use erya::sprite::Sprite;


fn main() {
    let display = erya::build_display("image", (800, 600));
    let res_key = PathBuf::from("examples/assets/block.png");
    let mut queue = Queue::<Texture>::new(&display, vec![res_key.clone()]);
    let camera = Camera2D::new(&display);
    let renderer = Renderer::new(&display);

    'main: loop {
        let camera = camera.matrix();
        let mut target = display.draw();
        if let RecvState::NotGot = queue.try_recv() {
            target.clear_color(0.25, 0.25, 0.25, 0.0);
        }
        else {
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            let sprite = Sprite::new(queue.received[&res_key].clone(), 128, 128);
            renderer.render(&mut target, &camera, &sprite);
        }
        target.finish().unwrap();
        for event in display.poll_events() {
            match event {
                Event::Closed => break 'main,
                _ => (),
            }
        }
        erya::timer::sleep_ms(1);
    }
}
