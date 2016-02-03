extern crate erya;
extern crate glium;


use std::rc::Rc;
use glium::glutin;
use glium::Surface;
use erya::queue::{ Queue, Event };
use erya::manager::TextureManager;
use erya::{ Renderer, Camera2D, Camera, Sprite, Renderable, sprite };


fn main()
{
    let display = erya::build_display("asyncloader", (800, 600));
    let queue = Rc::new(Queue::<Event>::new());
    let manager = TextureManager::new(&display, queue.clone());
    let id = manager.load("examples/assets/block.png");
    let mut sprite = None;
    let camera = Camera2D::new(&display);
    let renderer = Renderer::<sprite::Shader>::new(&display);

    'main: loop
    {
        if let Ok(e) = queue.receiver.try_recv()
        {
            manager.handle(e);
            let texture = manager.get(&id).unwrap();
            sprite = Some(Sprite::new(&display, texture, 128, 128));
        }

        let camera = camera.matrix();
        let mut target = display.draw();

        if let Some(ref sprite) = sprite
        {
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            sprite.draw(&renderer, &mut target, &camera);
        }
        else
        {
            target.clear_color(0.5, 0.5, 0.5, 0.0);
        }
        target.finish().unwrap();
        for event in display.poll_events()
        {
            match event
            {
                glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
    }
}
