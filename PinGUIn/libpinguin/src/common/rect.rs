use std::fmt;

pub struct Rect<T: num::PrimInt + Default> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: num::PrimInt + Default> fmt::Display for Rect<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.x, self.y, self.w, self.h)
    }
}
