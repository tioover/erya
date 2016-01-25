use num::Zero;
use cgmath::{Vector, Vector3, Matrix4, Basis3, Quaternion};


#[derive(Clone)]
pub struct Transform {
    pub scale: f32,
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub anchor: Vector3<f32>,
}


impl Transform {
    pub fn new() -> Transform {
        Transform {
            scale: 1.0,
            position: Vector::zero(),
            rotation: Quaternion::zero(),
            anchor: Vector::zero(),
        }
    }

    pub fn position(self, position: Vector3<f32>) -> Transform {
        Transform { position: position, ..self }
    }

    pub fn scale(self, scale: f32) -> Transform {
        Transform { scale: scale, ..self }
    }

    pub fn offset(self, anchor: Vector3<f32>) -> Transform {
        Transform { anchor: anchor, ..self }
    }

    #[inline]
    pub fn compute(&self, x: Vector3<f32>) -> Vector3<f32> {
        self.rotation * &((x - self.anchor) * self.scale) + self.position
    }

    #[inline]
    pub fn matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) *
        Matrix4::from(*Basis3::from(self.rotation).as_ref()) *
        Matrix4::from_scale(self.scale) *
        Matrix4::from_translation(-self.anchor)
    }
}
