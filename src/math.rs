use std::ops::{Add, Sub, Mul};
use num;
use num::Float;
use cgmath::{Vector2, BaseFloat};


pub const ROUND: f32 = 6.28318530717958647692528676655900576;


pub fn curve<F: BaseFloat>(control: [Vector2<F>; 4], t: F) -> Vector2<F> {
    // Cubic Bézier curves

    macro_rules! cast (
        ($x: expr) => (
            num::cast::<_, F>($x).unwrap()
        )
    );
    let p = control;
    let r = cast!(1.0) - t;

    let a = r.powi(3);
    let b = cast!(3.0)*t*r.powi(2);
    let c = cast!(3.0)*t.powi(2)*r;
    let d = t.powi(3);

    p[0] * a + p[1] * b + p[2] * c + p[3] * d
}


pub fn linear<T, F: Float>(a: T, b: T, t: F) -> T
    where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<F, Output=T>
{
    (b-a) * t + a
}



