use glium::{Display, Program};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use texture::TextureRef;
use math::Matrix;
use mesh::{Vertex, VertexType};


pub trait Shader {
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    fn program(&Display) -> Program;
}


pub struct Default;


pub struct DefaultUniforms {
    pub texture: TextureRef,
    pub matrix: Matrix,
}


impl Shader for Default {
    type Vertex = Vertex;
    type Uniforms = DefaultUniforms;

    fn program(display: &Display) -> Program {
        let vert = include_str!("shader/default.vert");
        let frag = include_str!("shader/default.frag");
        program!(display,
            140 => {
                vertex: vert,
                fragment: frag,
            },
        ).unwrap()
    }
}


impl Uniforms for DefaultUniforms {
    fn visit_values<'a, F>(&'a self, mut f: F)
        where F: FnMut(&str, UniformValue<'a>)
    {
        f("matrix", (*self.matrix.as_ref()).as_uniform_value());
        f("tex", self.texture.as_uniform_value());
    }
}

