extern crate nalgebra;
extern crate time;
extern crate rand;
extern crate image;
extern crate num;
#[macro_use]
extern crate glium;


#[macro_export]
macro_rules! na {
    ($x: expr) =>
        ($crate::na::Vec1::new($x));
    ($x: expr, $y: expr) =>
        ($crate::na::Vec2::new($x, $y));
    ($x: expr, $y: expr, $z: expr) =>
        ($crate::na::Vec3::new($x, $y, $z));
    ($x: expr, $y: expr, $z: expr, $w: expr) =>
        ($crate::na::Vec4::new($x, $y, $z, $w));
}


macro_rules! from {
    ($x: expr) => ($crate::num::NumCast::from($x).unwrap())
}


pub use nalgebra as na;
pub use glium::Display;


pub mod renderer;
pub mod mesh;
pub mod texture;
pub mod id;
pub mod math;
pub mod rect;
pub mod timer;
pub mod camera;
pub mod transform;
pub mod sprite;
pub mod shader;
pub mod utils;
pub mod loader;


pub use utils::{Ref, build_display};
