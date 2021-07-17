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

impl<T: GuiNumber + std::ops::Add<Output = T> + std::cmp::PartialOrd> Rect<T> {
    pub fn is_inside(&self, other_rect: &Rect<T>) -> bool {
        return (
            self.x > other_rect.x && self.x < other_rect.x + other_rect.w &&
            self.y > other_rect.y && self.y < other_rect.y + other_rect.h
        )
    }
}

impl<T: GuiNumber + fmt::Display> fmt::Display for Rect<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.x, self.y, self.w, self.h)
    }
}
