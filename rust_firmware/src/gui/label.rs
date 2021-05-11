use crate::gui::GuiWidget;
use crate::c_bindings::*;
use alloc::string::String;
use cstr_core::{CString, CStr};

pub struct Label {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub text: Option<String>,
    pub font: u8
}

impl GuiWidget for Label {
    fn new(x: i16, y: i16, w: i16, h: i16) -> Self {
        return Label {
            x: x,
            y: y,
            w: w,
            h: h,
            text: None,
            font: 1
        };
    }

    fn draw(&self) {
        unsafe {
            let c_str = CString::new(self.text.as_ref().unwrap().as_bytes()).expect("CString::new failed");
            drawString(c_str.as_ptr(), self.x.into(), self.y.into(), 0);
        }
    }
}