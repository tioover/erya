extern crate erya;
#[macro_use]
extern crate glium;


use glium::glutin::Event;
use glium::Surface;
use erya::{ Renderer, Camera2D, Camera, Timer, Sprite };
use erya::loader::{ Queue, Key };
use erya::texture::{ Texture, TextureRef };


fn main()
{
    let display = erya::build_display("image", (800, 600));
    let res_key = Key::from("examples/assets/block.png");
    let mut queue = Queue::<Texture>::new(&display, vec![res_key.clone()]);
    let camera = Camera2D::new(&display);
    let renderer = Renderer::new(&display);
    let mut timer = Timer::new().limit(40);
    let mut sprite: Option<Sprite> = None;

    'main: loop
    {
        use erya::loader::QueueState::{ Empty, NotReceived, Received };
        let camera = camera.matrix();
        let mut target = display.draw();


        match queue.try_recv()
        {
            NotReceived => target.clear_color(0.25, 0.25, 0.25, 0.0),
            Received => {
                let tex = TextureRef(queue.received[&res_key].clone());
                sprite = Some(Sprite::new(&display, tex, 128, 128));
            }
            Empty => {
                target.clear_color(0.0, 0.0, 0.0, 0.0);
                renderer.render(&mut target, &camera, sprite.as_ref().unwrap())
            }

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
