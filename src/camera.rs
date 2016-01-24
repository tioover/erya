use glium::Display;
use math::Matrix;
use transform::Transform;


pub trait Camera {
    fn matrix(&self) -> Matrix;
}

pub struct Camera2D<'display> {
    display: &'display Display,
    transform: Transform,
}


impl<'display> Camera2D<'display> {
    pub fn new(display: &'display Display) -> Camera2D<'display> {
        Camera2D {
            display: display,
            transform: Transform::new(),
        }
    }
}


impl<'a> Camera for Camera2D<'a> {
    fn matrix(&self) -> Matrix {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Matrix::new(
            f/w,  0.0,  0.0, -1.0,
            0.0, -f/h,  0.0,  1.0,
            0.0,  0.0, -1.0,  0.0,
            0.0,  0.0,  0.0,  1.0,
        ) * self.transform.matrix()
    }
}


pub struct Camera3D<'display> {
    display: &'display Display,
    transform: Transform,
}


impl<'display> Camera3D<'display> {
    pub fn new(display: &'display Display) -> Camera3D<'display> {
        Camera3D {
            display: display,
            transform: Transform::new(),
        }
    }
}

