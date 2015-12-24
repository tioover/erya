use std::rc::Rc;
use glium::{Display, Program};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use texture::{Texture, TextureUniform};
use math::Matrix;
use mesh::{Vertex, VertexType};


pub trait Shader {
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    fn program(&Display) -> Program;
}


pub struct Default;


pub struct DefaultUniforms {
    pub texture: Rc<Texture>,
    pub matrix: Matrix,
}


impl Shader for Default {
    type Vertex = Vertex;
    type Uniforms = DefaultUniforms;

    fn program(display: &Display) -> Program {
        let vert = include_str!("shader/140/default.vert");
        let frag = include_str!("shader/140/default.frag");
        program!(display,
            140 => {
                vertex: vert,
                fragment: frag,
            },
        ).unwrap()
    }
}


impl Uniforms for DefaultUniforms {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("matrix", (*self.matrix.as_ref()).as_uniform_value());
        f("tex", TextureUniform(&self.texture.data, None));
    }
}

