use crate::elements::GuiElement;
use alloc::prelude::v1::Box;
use alloc::vec::Vec;

pub struct GuiPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct GuiCanvas {
    pub elements: Vec<Box<dyn GuiElement>>,
}

impl GuiCanvas {
    pub fn get_pixel(x: u16, y: u16, output: &mut GuiPixel) {
        // TODO: Check which elements are overlapping this coordinate
    }
}
