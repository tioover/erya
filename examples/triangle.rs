#[macro_use(implement_uniforms)]
extern crate erya;
#[macro_use(implement_vertex)]
extern crate glium;
extern crate cgmath;


use glium::glutin::Event;
use glium::Surface;
use cgmath::{ Matrix4, Rotation3, Rad, Quaternion, Angle };
use erya::shader;
use erya::{ Renderer, Renderable, Display, Mesh, Camera3D, Camera,
            Timer, Transform, Model };


#[derive(Copy, Clone)]
struct Vertex
{
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! { Vertex, position, color }


struct Uniforms
{
    mat: [[f32; 4]; 4],
}

implement_uniforms! { Uniforms, mat }


struct Shader;


impl shader::Shader for Shader
{
    type Vertex = Vertex;
    type Uniforms = Uniforms;

    fn vertex() -> &'static str
    {
        "
        #version 140
        uniform mat4 mat;
        in vec2 position;
        in vec3 color;
        out vec3 vColor;

        void main()
        {
            gl_Position = mat * vec4(position, 0.0, 1.0);
            vColor = color;
        }
        "
    }

    fn fragment() -> &'static str
    {
        "
        #version 140
        in vec3 vColor;
        out vec4 f_color;
        void main()
        {
            f_color = vec4(vColor, 1.0);
        }
        "
    }
}


struct Triangle 
{
    mesh: Mesh<Vertex>,
    transform: Transform,
}


impl Triangle
{
    fn new(display: &Display) -> Triangle
    {
        let mesh = Mesh::new(&display, &[
            Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },
        ]);
        Triangle
        {
            mesh: mesh,
            transform: Transform::new(),
        }
    }
}


impl Model<Shader> for Triangle
{
    fn uniforms(&self, parent: &Matrix4<f32>) -> Uniforms
    {
        let mat = parent * self.transform.matrix();
        Uniforms { mat: mat.into() }
    }
    
    fn mesh(&self) -> &Mesh<Vertex> { &self.mesh }
}


fn main()
{
    let display = erya::build_display("triangle", (800, 600));
    let renderer = Renderer::<Shader>::new(&display);
    let mut camera = Camera3D::new(&display);
    camera.eye = cgmath::Point3::new(3.0, 4.0, 4.0);
    let mut timer = Timer::new().limit(60);
    let mut angle: f32 = 0.0;
    let mut triangle = Triangle::new(&display);

    'main: loop
    {
        angle += 0.01;
        let mut target = display.draw();
        triangle.transform.rotation = Quaternion::from_angle_x(Rad::new(angle));
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        triangle.draw(&renderer, &mut target, &camera.matrix());
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
