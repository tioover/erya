extern crate erya;
#[macro_use]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::glutin::Event;


fn main() {
    let display = erya::build_display("Window".to_string(), (800, 600));
    'main: loop {

        for event in display.poll_events() {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.finish().unwrap();

            match event {
                Event::Closed => break 'main,
                _ => (),
            }
        }
    }
}
