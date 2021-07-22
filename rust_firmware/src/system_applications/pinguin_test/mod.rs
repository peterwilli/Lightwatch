use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::system_applications::system_application::*;
use crate::utils::loop_time;
use crate::SerialLogger;
use alloc::format;
use alloc::vec;
use std::prelude::v1::*;
use alloc::prelude::v1::Box;
use libpinguin::elements::Button;
use libpinguin::common::Rect;
use libpinguin::elements::GuiElement;
use libpinguin::rendering::GuiCanvas;

pub struct PinguinTestApplication {
}

impl SystemApplication for PinguinTestApplication {
    fn new() -> Self {
        return {
            PinguinTestApplication {
            }
        };
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.test.pinguin".to_string(),
            name: "Pinguin Test".to_string(),
            description: "Pinguin Test".to_string(),
            extras: vec![],
        };
    }

    fn init(&mut self) {
        unsafe {
            fillScreen(0);
        }
        let mut gui_canvas = GuiCanvas::<i16, i16>::new(10, 10, 10, 10);
        let mut button = Box::new(Button::new(Rect {
            x: 0,
            y: 0,
            w: 100,
            h: 30,
        }));
        button.text = Some("Button".to_string());
        button.on_tap = Some(Box::new(|| {
            SerialLogger::println("Button tap!".to_string());
        }));
    }

    fn r#loop(&mut self) {
        
    }
}
