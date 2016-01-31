//! Some math function and constant.

use std::ops::{ Add, Sub, Mul };
use num;
use num::Float;
use cgmath::{ Vector2, BaseFloat };

/// 2π
pub const ROUND: f32 = 6.28318530717958647692528676655900576;


/// Calculate the cubic bézier curves.
pub fn curve<F>(control_point: [Vector2<F>; 4], t: F) -> Vector2<F>
    where F: BaseFloat
{
    let co = |x| num::cast::<_, F>(x).unwrap(); // Float covariance.

    let p = control_point;
    let r = co(1.0) - t;

    let a = r.powi(3);
    let b = co(3.0)*t*r.powi(2);
    let c = co(3.0)*t.powi(2)*r;
    let d = t.powi(3);

    p[0] * a + p[1] * b + p[2] * c + p[3] * d
}


/// Generics linear interpolation.
pub fn lerp<T, F: Float>(a: T, b: T, t: F) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<F, Output=T>
{
    (b-a) * t + a
}



