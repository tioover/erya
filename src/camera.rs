//! Perspective and view matrix.

use glium::Display;
use cgmath::{ Matrix, Matrix4, PerspectiveFov, Rad, Deg, deg, Point3, vec3 };
use transform::Transform;


/// A trait of camera types
pub trait Camera
{
    fn matrix(&self) -> Matrix4<f32>;

    fn array(&self) -> [[f32; 4]; 4]
    {
        self.matrix().into()
    }
}


/// A camera matrix, convert screen coordinates to OpenGL coordinates.
pub struct Camera2D
{
    display: Display,
    transform: Transform,
}


impl Camera2D
{
    pub fn new(display: &Display) -> Camera2D
    {
        Camera2D
        {
            display: display.clone(),
            transform: Transform::new(),
        }
    }
}


impl Camera for Camera2D
{
    fn matrix(&self) -> Matrix4<f32>
    {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Matrix4::new(
            f/w,  0.0,  0.0, -1.0,
            0.0, -f/h,  0.0,  1.0,
            0.0,  0.0, -1.0,  0.0,
            0.0,  0.0,  0.0,  1.0,
        ).transpose() * self.transform.matrix()
    }
}


/// 3D perspective camera.
pub struct Camera3D
{
    display: Display,
    pub pov: Deg<f32>,
    pub near: f32,
    pub far: f32,
    pub eye: Point3<f32>,
    pub center: Point3<f32>,
}


impl Camera3D
{
    pub fn new(display: &Display) -> Camera3D
    {
        Camera3D
        {
            display: display.clone(),
            pov: deg(45.0),
            near: 0.1,
            far: 100.0,
            eye: Point3::new(0.0, 0.0, 1.0),
            center: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn pov(self, pov: f32) -> Camera3D
    {
        Camera3D { pov: deg(pov), ..self }
    }

    pub fn near(self, near: f32) -> Camera3D
    {
        Camera3D { near: near, ..self }
    }

    pub fn far(self, far: f32) -> Camera3D
    {
        Camera3D { far: far, ..self }
    }

    fn aspect(&self) -> f32
    {
        let (w, h) = self.display.get_framebuffer_dimensions();
        w as f32 / h as f32
    }
}


impl Camera for Camera3D
{
    fn matrix(&self) -> Matrix4<f32>
    {
        let persp = PerspectiveFov
        {
            fovy: Rad::from(self.pov),
            aspect: self.aspect(),
            near: self.near,
            far: self.far,
        };
        let view = Matrix4::look_at(
            self.eye,
            self.center,
            vec3(0.0, 1.0, 0.0)
        );
        Matrix4::from(persp) * view
    }
}
