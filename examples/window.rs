extern crate erya;
#[macro_use]
extern crate glium;

use glium::Surface;
use glium::glutin::Event;


fn main() {
    let display = erya::build_display("Window", (800, 600));

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish().unwrap();
        for event in display.poll_events() {
            match event {
                Event::Closed => break 'main,
                _ => (),
            }
        }
        erya::timer::sleep_ms(10);
    }
}
