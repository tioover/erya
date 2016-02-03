extern crate erya;
#[macro_use]
extern crate glium;
extern crate image;


use std::rc::Rc;
use glium::glutin::Event;
use glium::Surface;
use erya::sprite;
use erya::{ Renderer, Camera2D, Camera, Sprite, Renderable, Texture };


fn main()
{
    let display = erya::build_display("image", (800, 600));
    let texture =
    {
        let image = image::open("examples/assets/block.png").unwrap();
        Rc::new(Texture::from_image(&display, &image))
    };
    let sprite = Sprite::new(&display, texture, 128, 128);
    let camera = Camera2D::new(&display);
    let renderer = Renderer::<sprite::Shader>::new(&display);

    'main: loop
    {
        let camera = camera.matrix();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        sprite.draw(&renderer, &mut target, &camera);
        target.finish().unwrap();
        for event in display.poll_events()
        {
            match event
            {
                Event::Closed => break 'main,
                _ => (),
            }
        }
    }
}
