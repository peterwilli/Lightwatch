use crate::common::Rect;
use core::any::Any;

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

pub trait GuiElement {
    fn new(r: Rect) -> Self
    where
        Self: Sized;

    fn r#loop(&mut self);
    fn get_bounds(&self) -> &Rect;
    fn is_inside(&self, x: u16, y: u16) -> bool;
    fn get_pixel(&self, x: u16, y: u16, output: &mut GuiElementPixel);
    fn as_any(&mut self) -> &mut dyn Any;
}
