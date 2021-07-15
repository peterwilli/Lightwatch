use core::default::Default;
use core::ops::{AddAssign, Div};

pub trait BasicGuiData {}

impl<T: num::PrimInt + AddAssign + Default> BasicGuiData for T {}

pub trait GuiData {}

impl<T: PartialOrd<T> + Div<T>> GuiData for T {}

struct Button<T: BasicGuiData + GuiData> {
    pub test: T,
    rect: Rect<T>,
}

#[derive(PartialEq, Debug)]
pub struct Rect<T: BasicGuiData> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

fn main() {
    println!("Hello, world!");
}
