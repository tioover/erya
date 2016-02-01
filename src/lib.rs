//! # Erya
//! [Erya](https://github.com/ruoruo-dev/erya) is a simple graphics rendering library.
//!
//! ## Examples
//! * [2D Sprite and texture asynchronous load](https://github.com/ruoruo-dev/erya/blob/master/examples/image.rs)
//! * [Simple 3D rendering](https://github.com/ruoruo-dev/erya/blob/master/examples/triangle.rs)

extern crate cgmath;
extern crate time;
extern crate rand;
extern crate image;
extern crate num;
extern crate either;
#[macro_use]
extern crate glium;


pub use glium::Display;


pub mod renderer;
pub mod model;
pub mod texture;
pub mod id;
pub mod math;
pub mod rect;
pub mod timer;
pub mod camera;
pub mod transform;
#[macro_use]
pub mod shader;
pub mod sprite;
pub mod utils;
pub mod loader;


pub use utils::build_display;
pub use renderer::{ Renderer, Renderable };
pub use texture::{ Texture, TextureRef };
pub use camera::{ Camera, Camera2D, Camera3D };
pub use sprite::Sprite;
pub use timer::Timer;
pub use model::{ Mesh, Model };
pub use transform::Transform;

