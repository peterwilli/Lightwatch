use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use crate::gui::*;
use alloc::vec;
use cstr_core::{CString};
use crate::SerialLogger;

use crate::c_bindings::*;
use std::prelude::v1::*;

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
        
        let mut button = Box::new(Button::new(10, 30, 100, 100));
        button.text = Some("Button".to_string());
        button.on_tap = Some(Box::new(|| {
            let c_str = CString::new("I love you".as_bytes()).expect("CString::new failed");
            unsafe {
                drawString(c_str.as_ptr(), 10, 50, 1);
            }
        }));
        
        self.gui_renderer.elements.push(label);
        self.gui_renderer.elements.push(button);
    }

    fn r#loop(&mut self) {
        self.gui_renderer.r#loop();
    }
}