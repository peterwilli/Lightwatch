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
use crate::rendering::FontRenderer;
use crate::alloc::string::ToString;
use core::convert::TryInto;

pub struct Label<T: GuiNumber + 'static> {
    rect: Rect<T>,
    pub text: Option<String>,
    pub font: u8,
    needs_redraw: bool,
    font_renderer: FontRenderer<T>
}

impl<T: GuiNumber + num::Zero + TryInto<usize>> GuiElement<T> for Label<T> where T::Error: std::fmt::Debug {
    fn new(rect: Rect<T>) -> Self {
        return Label {
            rect: rect,
            text: None,
            font: 1,
            needs_redraw: true,
            font_renderer: FontRenderer::new()
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
        // self.font_renderer.pixel_for_letter(self.text.as_ref().unwrap().chars().next().unwrap(), "test".to_string(), 20.0, x, y, output);
    }

    fn r#loop(&mut self) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
