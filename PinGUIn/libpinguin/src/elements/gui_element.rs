use crate::common::GuiNumber;
use crate::elements::Rect;
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

pub trait GuiElement<T>
where
    T: GuiNumber,
{
    fn new(rect: Rect<T>) -> Self
    where
        Self: Sized;
    fn transform(&mut self, new_rect: Rect<T>);
    fn r#loop(&mut self);
    fn get_bounds(&self) -> &Rect<T>;
    fn is_inside(&self, x: T, y: T) -> bool;
    fn get_pixel(&self, x: T, y: T, output: &mut GuiElementPixel);
    fn as_any(&mut self) -> &mut dyn Any;
}
