use std::ops::Deref;
use std::string::ToString;
use num::NumCast;
use glium::glutin::WindowBuilder;
use glium::{ Display, DisplayBuild };


pub fn build_display<T>(title: T, (width, height): (u32, u32)) -> Display
    where T: ToString
{
    WindowBuilder::new()
        .with_title(title.to_string())
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}


pub enum Ref<'a, B> where B: 'a
{
    Borrowed(&'a B),
    Owned(B),
}


impl<'a, B> Deref for Ref<'a, B> where B: 'a
{
    type Target = B;
    fn deref(&self) -> &B {
        match self {
            &Ref::Borrowed(x) => x,
            &Ref::Owned(ref x) => x,
        }
    }
}



#[inline]
pub fn cast<T, U>(x: T) -> U
    where T: NumCast, U: NumCast
{
    U::from(x).unwrap()
}


