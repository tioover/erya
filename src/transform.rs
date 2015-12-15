use na;
use na::{Vec3, Rot3, ToHomogeneous};
use math;
use math::Matrix;


#[derive(Clone)]
pub struct Transform {
    pub scale: f32,
    pub position: Vec3<f32>,
    pub rotation: Rot3<f32>,
    pub anchor: Vec3<f32>,
}


impl Transform {
    pub fn new() -> Transform {
        Transform {
            scale: 1.0,
            position: na::zero(),
            rotation: Rot3::new(na::zero()),
            anchor: na::zero(),
        }
    }

    pub fn position(self, position: Vec3<f32>) -> Transform {
        Transform { position: position, ..self }
    }

    pub fn scale(self, scale: f32) -> Transform {
        Transform { scale: scale, ..self }
    }

    pub fn offset(self, anchor: Vec3<f32>) -> Transform {
        Transform { anchor: anchor, ..self }
    }

    #[inline]
    pub fn compute(&self, x: Vec3<f32>) -> Vec3<f32> {
        (x - self.anchor) * self.scale * self.rotation + self.position
    }

    #[inline]
    pub fn matrix(&self) -> Matrix {
        math::translation(self.position) * 
        self.rotation.to_homogeneous() * 
        math::scaling(self.scale) * 
        math::translation(-self.anchor)
    }
}
