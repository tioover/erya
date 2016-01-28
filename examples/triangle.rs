#[macro_use(implement_uniforms)]
extern crate erya;
#[macro_use(implement_vertex)]
extern crate glium;
extern crate cgmath;


use glium::glutin::Event;
use glium::Surface;
use erya::{ Renderer, Mesh, Camera3D, Camera, Timer };


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


impl erya::shader::Shader for Shader
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

        void main() {
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
        void main() {
            f_color = vec4(vColor, 1.0);
        }
        "
    }
}


fn main()
{
    let display = erya::build_display("triangle", (800, 600));
    let renderer = Renderer::<Shader>::new(&display);
    let mut camera = Camera3D::new(&display);
    let mut timer = Timer::new().limit(40);
    camera.eye = cgmath::Point3::new(3.0, 4.0, 4.0);
    let mesh = Mesh::new(&display, &[
            Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },
        ]);

    'main: loop {
        let mut target = display.draw();

        let uniforms = Uniforms { mat: camera.matrix().into() };
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        renderer.draw(&mut target, &mesh, &uniforms);
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
