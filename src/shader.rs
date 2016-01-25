use glium::{Display, Program};
pub use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use texture::TextureRef;
use mesh::{Vertex, VertexType};


pub trait Shader {
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    fn vertex() -> &'static str;
    fn fragment() -> &'static str;

    fn program(display: &Display) -> Program {
        program!(display,
            140 => {
                vertex: Self::vertex(),
                fragment: Self::fragment(),
            },
        ).unwrap()
    }
}


pub struct Default;


impl Shader for Default {
    type Vertex = Vertex;
    type Uniforms = DefaultUniforms;

    fn vertex() -> &'static str {
        include_str!("shader/default.vert")
    }

    fn fragment() -> &'static str {
        include_str!("shader/default.frag")
    }
}

#[macro_export]
macro_rules! implement_uniforms {
    ($name: ident, $($field: ident),*) => {
        impl $crate::shader::Uniforms for $name {
            fn visit_values<'a, F>(&'a self, mut output: F)
                where F: FnMut(&str, $crate::shader::UniformValue<'a>)
            {
                use $crate::shader::AsUniformValue;
                $(
                    output(stringify!($field), self.$field.as_uniform_value());
                )*
            }
        }
    }
}



pub struct DefaultUniforms {
    pub tex: TextureRef,
    pub matrix: [[f32; 4]; 4],
}

implement_uniforms! {DefaultUniforms, tex, matrix}

