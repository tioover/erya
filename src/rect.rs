//! Integer rectangle.


pub struct Rect
{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}


impl Rect
{
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect
    {
        Rect
        {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }
}
