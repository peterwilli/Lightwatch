use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use crate::gui::*;
use alloc::vec;
use cstr_core::{CString, CStr};
use cstr_core::c_char;
use crate::c_bindings::*;
use std::prelude::v1::*;
use alloc::sync::Arc;
use no_std_compat::sync::Mutex;

pub struct HomeScreenApplication {
    gui_renderer: GUIRenderer
}

impl SystemApplication for HomeScreenApplication {
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

    fn init(&mut self) {
        unsafe {
            fillScreen(1929);
            setTextColor(400);
        }
        let mut label = Box::new(Label::new(10, 10, 100, 100));
        label.text = Some("Hello Love TCSD".to_string());
        self.gui_renderer.elements.push(label);
    }

    fn r#loop(&mut self) {
        unsafe {
            let mut x:i16 = 0;
            let mut y:i16 = 0;
            let is_touched = getTouch(&mut x, &mut y) == 1;
            self.gui_renderer.r#loop();
        }
    }
}