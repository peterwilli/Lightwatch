use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use crate::gui::*;
use alloc::vec;
use cstr_core::{CString, CStr};
use cstr_core::c_char;
use crate::c_bindings::*;
use std::prelude::v1::*;

pub struct HomeScreenApplication<'a> {
    gui_renderer: GUIRenderer<'a>
}

impl SystemApplication for HomeScreenApplication<'_> {
    fn new() -> Self {
        return {
            HomeScreenApplication {
                gui_renderer: GUIRenderer::new()
            }
        }
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.home".to_string(),
            name: "Home".to_string(),
            description: "Home screen".to_string(),
            extras: vec![Extra::BackgroundLoop]
        }
    }

    fn init(&self) {
        unsafe {
            fillScreen(1929);
            setTextColor(400);
        }
    }

    fn r#loop(&self) {
        unsafe {
            let mut x:i16 = 0;
            let mut y:i16 = 0;
            let is_touched = getTouch(&mut x, &mut y) == 1;
            if is_touched {
                let hello = CString::new(format!("{} {}", x, y)).expect("CString::new failed");
                // serialPrintln(hello.as_ptr());
                fillScreen(10);
                drawString(hello.as_ptr(), 10, 10, 7);
            }
        }
    }
}