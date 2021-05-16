use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::non_official_c_bindings::*;
use crate::system_applications::system_application::*;
use crate::SerialLogger;
use alloc::format;
use alloc::vec;
use core::ffi::c_void;
use cstr_core::CString;

pub struct ActivityApplication {
    gui_renderer: GUIRenderer,
}

impl SystemApplication for ActivityApplication {
    fn new() -> Self {
        return {
            ActivityApplication {
                gui_renderer: GUIRenderer::new(),
            }
        };
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.activity".to_string(),
            name: "Activity".to_string(),
            description: "Fitness activity application".to_string(),
            extras: vec![Extra::BackgroundLoop],
        };
    }

    fn init(&mut self) {
        unsafe {
            enableAccelerometer();
            fillScreen(2821);
            setTextColor(400);

            let c_str = CString::new("I love you".as_bytes()).expect("CString::new failed");
            drawString(c_str.as_ptr(), 10, 50, 1);
        }
    }

    fn r#loop(&mut self) {
        unsafe {
            let mut accel = Accel { x: 0, y: 0, z: 0 };
            let x = readAccelerometer(&mut accel);
            SerialLogger::println(format!("accel: {}x{}x{}", accel.x, accel.y, accel.z));
            delay(100);
        }
    }
}
