use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::non_official_c_bindings::*;
use crate::system_applications::system_application::*;
use crate::utils::loop_time;
use crate::SerialLogger;
use alloc::format;
use alloc::vec;
use core::ffi::c_void;
use cstr_core::CString;
use std::prelude::v1::*;

pub struct LucidDreamingApplication {
    gui_renderer: GUIRenderer,
    app_start_time: u32,
    last_check_time: u32,
    rausis_1: u32,
    rausis_2: u32,
    alarm_state: u8,
}

impl LucidDreamingApplication {
    fn vibrate(&self, pattern: &[u16]) {
        for (i, pattern_piece) in pattern.iter().enumerate() {
            if i % 2 == 0 {
                let pattern_piece_calculated = (pattern_piece / 100);
                for i2 in 0..pattern_piece_calculated {
                    unsafe {
                        vibrate(100);
                        delay(100);
                    }
                }
            } else {
                unsafe {
                    delay(*pattern_piece as u32);
                }
            }
        }
    }
}

impl SystemApplication for LucidDreamingApplication {
    fn new() -> Self {
        return {
            LucidDreamingApplication {
                gui_renderer: GUIRenderer::new(),
                last_check_time: 0,
                app_start_time: 0,
                rausis_1: 18000,
                rausis_2: 720,
                alarm_state: unsafe { getRTCDataAtIndex(0) },
            }
        };
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.luciddreaming".to_string(),
            name: "Lucid Dreaming".to_string(),
            description: "LD alarm helper application".to_string(),
            extras: vec![Extra::BackgroundLoop],
        };
    }

    fn init(&mut self) {
        unsafe {
            self.app_start_time = loop_time.secs();
            fillScreen(0);
            if self.alarm_state == 0 {
                setRTCDataAtIndex(0, 1);
                deepSleep(self.rausis_1 * 1000);
            } else if self.alarm_state == 1 {
                enableVibrator();
                self.vibrate(&vec![1000, 100, 1000, 100, 1000, 100, 1000, 100]);
                setRTCDataAtIndex(0, 2);
                deepSleep(self.rausis_2 * 1000);
            } else if self.alarm_state == 2 {
                enableVibrator();
                enableAccelerometer();
                self.vibrate(&vec![500, 1000, 500, 1000, 500, 1000, 500, 1000]);
                deepSleep(60 * 60 * 24 * 1000);
            }
        }
    }

    fn r#loop(&mut self) {}
}
