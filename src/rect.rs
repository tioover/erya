use num::NumCast;
use utils::cast;


pub struct Rect
{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}


impl Rect
{
    pub fn new<N: NumCast>(x: N, y: N, w: N, h: N) -> Rect
    {
        Rect
        {
            x: cast(x),
            y: cast(y),
            width: cast(w),
            height: cast(h)
        }
    }
}
