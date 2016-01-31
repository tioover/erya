//! Shader and uniforms definition tools.

use glium::{ Display, Program };
pub use glium::uniforms::{ Uniforms, UniformValue, AsUniformValue };
use mesh::VertexType;


/// Trait of the shader mark type,
/// that provide shader program code and related type convention.
pub trait Shader
{
    type Vertex: VertexType;
    type Uniforms: Uniforms;

    /// Vertex shader source code.
    fn vertex() -> &'static str;

    /// Fragment shader source code.
    fn fragment() -> &'static str;

    /// Compilation shaders and return shaders object.
    fn program(display: &Display) -> Program
    {
        program!(display,
            140 =>
            {
                vertex: Self::vertex(),
                fragment: Self::fragment(),
            },
        ).unwrap()
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
