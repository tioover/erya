use num::NumCast;
use utils::cast;


pub struct Rect
{
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
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
