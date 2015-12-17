use std::rc::Rc;
use glium::{Display, Program};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use texture::Texture;
// use texture::TextureData;
use math::Matrix;
use mesh::{Vertex, VertexType};


pub trait Shader: Uniforms {
    type Vertex: VertexType;
    fn program(&Display) -> Program;
}


pub struct Default {
    pub texture: Rc<Texture>,
    pub matrix: Matrix,
}


impl Default {
    // pub fn get_texture<'a>(&'a self) -> &'a TextureData {
    //     &self.texture.data
    // }
}


impl Shader for Default {
    type Vertex = Vertex;
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


impl Uniforms for Default {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("matrix", (*self.matrix.as_ref()).as_uniform_value());
        // FIXME: use AsUniformValue::as_uniform_value(&self.texture.data)
        f("tex", UniformValue::CompressedSrgbTexture2d(&self.texture.data, None));
    }
}

