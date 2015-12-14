use glium::Display;
use math::Matrix;


pub struct Camera<'display> {
    display: &'display Display,
}


impl<'display> Camera<'display> {
    pub fn new(display: &'display Display) -> Camera<'display> {
        Camera {
            display: display,
        }
    }

    pub fn matrix(&self) -> Matrix {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Matrix::new(
            f/w,  0.0,  0.0, -1.0,
            0.0, -f/h,  0.0,  1.0,
            0.0,  0.0, -1.0,  0.0,
            0.0,  0.0,  0.0,  1.0,
        )
    }
}

