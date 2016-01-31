//! Utilities functions.

use std::string::ToString;
use num::NumCast;
use glium::glutin::WindowBuilder;
use glium::{ Display, DisplayBuild };


/// Build OpenGL context and create window.
pub fn build_display<T>(title: T, (width, height): (u32, u32)) -> Display
    where T: ToString
{
    WindowBuilder::new()
        .with_title(title.to_string())
        .with_multisampling(4)
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}


/// Number cast function.
#[inline]
pub fn cast<T, U>(x: T) -> U
    where T: NumCast, U: NumCast
{
    U::from(x).unwrap()
}


