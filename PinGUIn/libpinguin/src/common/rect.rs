use alloc::fmt::Display;
use std::fmt;
use crate::common::GuiNumber;

#[derive(PartialEq, Debug)]
pub struct Rect<T: GuiNumber> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl fmt::Display for GuiNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.x, self.y, self.w, self.h)
    }
}

impl<T: GuiNumber> fmt::Display for Rect<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.x, self.y, self.w, self.h)
    }
}
