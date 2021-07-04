use crate::common::Rect;
use crate::elements::GuiElement;
use crate::elements::GuiElementPixel;
use alloc::prelude::v1::Box;
use alloc::string::String;
use alloc::sync::Arc;
use core::any::Any;
use no_std_compat::sync::Mutex;

pub struct Button {
    rect: Rect,
    pub text: Option<String>,
    pub font: u8,
    pub on_tap: Option<Box<dyn Fn()>>,
    pub is_pressed: bool,
    needs_redraw: Option<Arc<Mutex<bool>>>,
}

impl GuiElement for Button {
    fn new(rect: Rect) -> Self {
        return Button {
            rect: rect,
            text: None,
            font: 1,
            is_pressed: false,
            on_tap: None,
            needs_redraw: None,
        };
    }

    fn get_bounds(&self) -> &Rect {
        return &self.rect;
    }

    fn is_inside(&self, x: u16, y: u16) -> bool {
        return true;
    }

    fn get_pixel(&self, x: u16, y: u16, output: &mut GuiElementPixel) {
        output.r = 1;
    }

    fn r#loop(&mut self) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
