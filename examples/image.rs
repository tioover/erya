extern crate erya;
#[macro_use]
extern crate glium;


use std::path::PathBuf;
use glium::glutin::Event;
use glium::Surface;
use erya::renderer::Renderer;
use erya::loader::{ Queue, QueueState };
use erya::texture::{ Texture, TextureRef };
use erya::camera::{ Camera, Camera2D };
use erya::sprite::Sprite;
use erya::timer::Timer;


fn main()
{
    let display = erya::build_display("image", (800, 600));
    let res_key = PathBuf::from("examples/assets/block.png");
    let mut queue = Queue::<Texture>::new(&display, vec![res_key.clone()]);
    let camera = Camera2D::new(&display);
    let renderer = Renderer::new(&display);
    let mut timer = Timer::new().limit(40);

    'main: loop
    {
        let camera = camera.matrix();
        let mut target = display.draw();
        if let QueueState::NotReceived = queue.try_recv()
        {
            target.clear_color(0.25, 0.25, 0.25, 0.0);
        }
        else
        {
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            let tex = TextureRef(queue.received[&res_key].clone());
            let sprite = Sprite::new(&display, tex, 128, 128);
            renderer.render(&mut target, &camera, &sprite);
        }
        target.finish().unwrap();
        for event in display.poll_events()
        {
            match event
            {
                Event::Closed => break 'main,
                _ => (),
            }
        }
        timer.update();
    }
}
