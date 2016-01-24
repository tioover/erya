#[macro_use(uniforms_define)]
extern crate erya;
#[macro_use]
extern crate glium;


use std::default::Default;
use glium::glutin::Event;
use glium::Surface;
use glium::index::PrimitiveType::TrianglesList;
use erya::renderer::Renderer;
use erya::camera::{Camera3D, Camera};
use erya::mesh::{IndexBuffer, VertexBuffer, Mesh};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);


uniforms_define! {
    Uniforms {
        matrix: [[f32; 4]; 4],
    }
}


struct Shader;


impl erya::shader::Shader for Shader {
    type Vertex = Vertex;
    type Uniforms = Uniforms;

    fn vertex() -> &'static str {
        "
        #version 140
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 vColor;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
            vColor = color;
        }
        "
    }

    fn fragment() -> &'static str {
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


fn main() {
    let display = erya::build_display("triangle", (800, 600));
    let renderer = Renderer::<Shader>::new(&display);
    let camera = Camera3D::new(&display);
    let mesh = {
        let vb = VertexBuffer::new(&display, &[
            Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]).unwrap();
        let ib = IndexBuffer::new(&display, TrianglesList, &[0, 1, 2]).unwrap();
        Mesh(vb, ib)
    };

    let uniforms = Uniforms {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    };

    'main: loop {
        // let camera = camera.matrix();
        let mut target = display.draw();
        target.clear_color(0.25, 0.25, 0.25, 0.0);
        renderer.draw(&mut target, &mesh, &uniforms);
        target.finish().unwrap();
        for event in display.poll_events() {
            match event {
                Event::Closed => break 'main,
                _ => (),
            }
        }
        erya::timer::sleep_ms(1);
    }
}
