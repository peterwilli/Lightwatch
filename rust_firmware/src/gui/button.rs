use crate::c_bindings::*;
use crate::gui::event_checks::widget_is_tapped;
use crate::gui::GuiWidget;
use crate::input::*;
use alloc::prelude::v1::Box;
use alloc::string::String;
use core::any::Any;
use cstr_core::CString;

use alloc::sync::Arc;
use no_std_compat::sync::Mutex;

pub struct Button {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub text: Option<String>,
    pub font: u8,
    pub on_tap: Option<Box<dyn Fn()>>,
    pub is_pressed: bool,
    needs_redraw: Option<Arc<Mutex<bool>>>,
}

impl GuiWidget for Button {
    fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        return Button {
            x: x,
            y: y,
            w: w,
            h: h,
            text: None,
            font: 1,
            is_pressed: false,
            on_tap: None,
            needs_redraw: None,
        };
    }

    fn r#loop(&mut self, _needs_redraw: &mut bool) {
        unsafe {
            let c_str =
                CString::new(self.text.as_ref().unwrap().as_bytes()).expect("CString::new failed");
            drawString(c_str.as_ptr(), self.x.into(), self.y.into(), self.font);

            let is_pressed = widget_is_tapped(self.x, self.y, self.w, self.h);
            if self.on_tap.is_some() {
                if self.is_pressed != is_pressed {
                    // Means we updated the button state
                    if !is_pressed {
                        (self.on_tap.as_ref().unwrap())();
                    }
                    self.is_pressed = is_pressed;
                }
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
