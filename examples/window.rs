extern crate erya;
#[macro_use]
extern crate glium;


use glium::Surface;
use glium::glutin::Event;
use erya::build_display;


fn main()
{
    let display = build_display("window", (800, 600));

    'main: loop
    {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
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
