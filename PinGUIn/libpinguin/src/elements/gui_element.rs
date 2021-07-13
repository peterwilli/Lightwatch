use crate::elements::Rect;
use core::any::Any;
use core::ops::{AddAssign, Div, Sub};

pub struct GuiElementPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl GuiElementPixel {
    pub fn new() -> Self {
        return GuiElementPixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
    }
}

pub trait GuiData {}

impl<T: num::PrimInt + AddAssign + PartialOrd<T> + Div<T> + Default> GuiData for T {}

pub trait GuiElement<T>
where
    T: GuiData,
{
    fn new(r: Rect<T>) -> Self
    where
        Self: Sized;
    fn transform(&mut self, new_rect: Rect<T>);
    fn r#loop(&mut self);
    fn get_bounds(&self) -> &Rect<T>;
    fn is_inside(&self, x: u16, y: u16) -> bool;
    fn get_pixel(&self, x: u16, y: u16, output: &mut GuiElementPixel);
    fn as_any(&mut self) -> &mut dyn Any;
}
