//! Texture handler types.

use std::path::Path;
use std::rc::Rc;
use std::cmp::{ PartialEq, Eq };
use std::ops::Deref;
use image::{ open, GenericImage };
use glium::Display;
use glium::texture::{ Texture2dDataSource, RawImage2d };
use glium::uniforms::{ AsUniformValue, UniformValue };
use id::Id;
use loader::Resource;


pub type TextureData = ::glium::texture::CompressedSrgbTexture2d;


/// 2D Texture Object.
pub struct Texture
{
    pub id: Id,
    pub height: u32,
    pub width: u32,
    pub data: TextureData,
}


/// Texture reference.
#[derive(Clone)]
pub struct TextureRef(pub Rc<Texture>);


impl TextureRef
{
    pub fn new(tex: Texture) -> TextureRef
    {
        TextureRef(Rc::new(tex))
    }
}


impl AsUniformValue for TextureRef
{
    fn as_uniform_value(&self) -> UniformValue
    {
        UniformValue::CompressedSrgbTexture2d(&self.0.data, None)
    }
}


impl Deref for TextureRef
{
    type Target = Texture;

    fn deref(&self) -> &Texture { self.0.deref() }
}



impl Texture {
    pub fn new<'a, T>(display: &Display, source: T) -> Texture
        where T: Texture2dDataSource<'a>
    {
        let tex = TextureData::new(display, source).unwrap();
        Texture
        {
            id: Id::new(),
            width: tex.get_width(),
            height: tex.get_height().unwrap(),
            data: tex,
        }
    }
}


impl Eq for Texture {}


impl PartialEq<Texture> for Texture
{
    fn eq(&self, other: &Texture) -> bool { self.id == other.id }
}


impl Resource for Texture
{
    type LoadData = RawImage2d<'static, u8>;

    fn load<P>(path: &P) -> Self::LoadData
        where P: AsRef<Path>
    {
        let image = open(path.as_ref()).unwrap();
        RawImage2d::from_raw_rgba_reversed(image.raw_pixels(), image.dimensions())
    }

    fn generate(display: &Display, data: Self::LoadData) -> Texture
    {
        Texture::new(display, data)
    }
}

