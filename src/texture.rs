//! Texture handler types.

use std::rc::Rc;
use std::cmp::{ PartialEq, Eq };
use std::ops::Deref;
use image::{ DynamicImage, GenericImage };
use glium::Display;
use glium::texture::{ Texture2dDataSource, RawImage2d };
use glium::uniforms::{ AsUniformValue, UniformValue };
use id::Id;


pub type TextureData = ::glium::texture::CompressedSrgbTexture2d;


/// 2D Texture Object.
pub struct Texture
{
    pub id: Id,
    pub height: u32,
    pub width: u32,
    pub data: TextureData,
}


impl Texture {

    pub fn new<'a, T>(display: &Display, source: T) -> Texture
        where T: Texture2dDataSource<'a>
    {
        Texture::with_id(display, source, Id::new())
    }

    pub fn with_id<'a, T>(display: &Display, source: T, id: Id) -> Texture
        where T: Texture2dDataSource<'a>
    {
        let tex = TextureData::new(display, source).unwrap();
        Texture
        {
            id: id,
            width: tex.get_width(),
            height: tex.get_height().unwrap(),
            data: tex,
        }
    }

    pub fn from_image(display: &Display, image: &DynamicImage) -> Texture
    {
        Texture::new(display, RawImage2d::from_raw_rgba_reversed(image.raw_pixels(), image.dimensions()))
    }
}


impl Eq for Texture {}


impl PartialEq<Texture> for Texture
{
    fn eq(&self, other: &Texture) -> bool { self.id == other.id }
}


/// Texture reference.
#[derive(Clone)]
pub struct TextureRef(pub Rc<Texture>);


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
