use crate::common::GuiNumber;
use crate::common::Rect;
use crate::elements::GuiElement;
use crate::elements::GuiElementPixel;
use crate::elements::GuiRect;
use crate::elements::Label;
use alloc::prelude::v1::Box;
use alloc::string::String;
use alloc::sync::Arc;
use core::any::Any;
use core::ops::{AddAssign, Div, Sub};
use no_std_compat::sync::Mutex;
use core::convert::TryInto;

pub struct Button<T: GuiNumber + 'static> {
    rect: Rect<T>,
    pub label: Label<T>,
    pub font: u8,
    pub on_tap: Option<Box<dyn Fn()>>,
    pub is_pressed: bool,
    needs_redraw: bool,
}

impl<T: GuiNumber> Button<T> {
    pub fn set_text(&mut self, text: String) {
        // self.label.text = Some(text);
    }
}

impl<T: GuiNumber + num::Zero + Copy + TryInto<usize>> GuiElement<T> for Button<T> where T::Error: std::fmt::Debug {
    fn new(rect: Rect<T>) -> Self {
        let rect_clone = rect.clone();
        return Button {
            rect: rect,
            label: Label::new(rect_clone),
            font: 1,
            is_pressed: false,
            on_tap: None,
            needs_redraw: true,
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

    fn get_pixel(&mut self, x: T, y: T, output: &mut GuiElementPixel) {
        output.r = 255;
    }

    fn r#loop(&mut self) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
