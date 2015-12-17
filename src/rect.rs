use num::NumCast;
use na::Vec2;


pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}


impl Rect {
    pub fn new<N: NumCast>(x: N, y: N, w: N, h: N) -> Rect {
        Rect {
            x: from!(x),
            y: from!(y),
            width: from!(w),
            height: from!(h)
        }
    }


    pub fn position(&self) -> Vec2<f32> {
        Vec2::new(self.x, self.y)
    }

    pub fn size(&self) -> Vec2<f32> {
        Vec2::new(self.width, self.height)
    }
}
