use id::Id;
use glium::Display;
use glium::texture::Texture2dDataSource;
use std::cmp::{PartialEq, Eq};


macro_rules! define_texture_data {
    ($x: ident) => {
        // hacky, avoid lifetime issue
        pub use glium::uniforms::UniformValue::$x as TextureUniform; 
        pub type TextureData = ::glium::texture::$x;
    }
}


define_texture_data! { CompressedSrgbTexture2d }


pub struct Texture {
    pub id: Id,
    pub height: u32,
    pub width: u32,
    pub data: TextureData,
}


impl Texture {
    pub fn new<'a, T>(display: &Display, source: T) -> Texture
            where T: Texture2dDataSource<'a>
    {
        let tex = TextureData::new(display, source).unwrap();
        Texture {
            id    : Id::new(),
            width : tex.get_width(),
            height: tex.get_height().unwrap(),
            data  : tex,
        }
    }
}


impl PartialEq<Texture> for Texture {
    fn eq(&self, other: &Texture) -> bool {
        self.id == other.id
    }
}


impl Eq for Texture {}

