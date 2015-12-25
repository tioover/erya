extern crate erya;
#[macro_use]
extern crate glium;


use std::rc::Rc;
use glium::glutin::Event;
use glium::Surface;
use erya::renderer::Renderer;
use erya::loader::Resource;
use erya::texture::Texture;
use erya::camera::Camera2D;
use erya::sprite::Sprite;


fn main() {
    let texture = Texture::get("examples/assets/block.png");
    let display = erya::build_display("image", (800, 600));
    let camera = Camera2D::new(&display);
    let renderer = Renderer::new(&display);
    let sprite = Sprite::new(Rc::new(texture.unwrap(&display)), 128, 128);

    'main: loop {
        let camera = camera.matrix();
        let mut target = display.draw();
        target.clear_color(0.25, 0.25, 0.25, 0.0);
        renderer.render(&mut target, &camera, &sprite);
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
