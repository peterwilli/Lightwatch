use crate::common::GuiNumber;
use crate::common::Rect;
use crate::elements::GuiElement;
use crate::elements::GuiElementPixel;
use crate::elements::GuiRect;
use alloc::prelude::v1::Box;
use alloc::string::String;
use alloc::sync::Arc;
use core::any::Any;
use core::ops::{AddAssign, Div, Sub};
use no_std_compat::sync::Mutex;

pub struct Button<T: GuiNumber> {
    rect: Rect<T>,
    pub text: Option<String>,
    pub font: u8,
    pub on_tap: Option<Box<dyn Fn()>>,
    pub is_pressed: bool,
    needs_redraw: Option<Arc<Mutex<bool>>>,
}

impl<T: GuiNumber> GuiElement<T> for Button<T> {
    fn new(rect: Rect<T>) -> Self {
        return Button {
            rect: rect,
            text: None,
            font: 1,
            is_pressed: false,
            on_tap: None,
            needs_redraw: None,
        };
    }

    fn transform(&mut self, new_rect: Rect<T>) {
        self.rect = new_rect;
    }

    fn get_bounds(&self) -> &Rect<T> {
        return &self.rect;
    }

    fn is_inside(&self, x: T, y: T) -> bool {
        return true;
    }

    fn get_pixel(&self, x: T, y: T, output: &mut GuiElementPixel) {
        output.r = 255;
    }

    fn r#loop(&mut self) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
