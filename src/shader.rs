//! Shader and uniforms definition tools.

use glium::{ Display, Program };
pub use glium::uniforms::{ Uniforms, UniformValue, AsUniformValue };
use mesh::VertexType;


/// Trait of the shader mark type,
/// that provide GLSL source code and related type convention.
pub trait ShaderType
{
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    /// Vertex shader GLSL source code.
    fn vertex() -> &'static str;

    /// Fragment shader GLSL source code.
    fn fragment() -> &'static str;

    /// Compilation shaders and return shaders object.
    fn program(display: &Display) -> Program
    {
        let result = Program::from_source(display, Self::vertex(), Self::fragment(), None);
        match result
        {
            Ok(program) => program,
            Err(err) => {
                println!("{}", err);
                panic!("Create shader program error.");
            },
        }
    }
}


/// Implements the `glium::uniforms::Uniforms` trait for the given type.
#[macro_export]
macro_rules! implement_uniforms
{
    ($name: ident, $($field: ident),*) =>
    {
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
