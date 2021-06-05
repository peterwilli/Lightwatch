use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::loop_time::loop_time;
use crate::non_official_c_bindings::*;
use crate::system_applications::system_application::*;
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
                rausis_1: /*18000*/ 5,
                rausis_2: 5/*720*/,
                alarm_state: 0
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
            fillScreen(2821);
            enableVibrator();
            let mut label = Box::new(Label::new(10, 10, 100, 100));
            label.font_size = 2;
            label.text = Some("0".to_string());
            self.gui_renderer.elements.push(label);

            let mut label = Box::new(Label::new(10, 50, 100, 100));
            label.font_size = 2;
            label.text = Some("0".to_string());
            self.gui_renderer.elements.push(label);
        }
    }

    fn r#loop(&mut self) {
        unsafe {
            if loop_time.millis > self.last_check_time {
                let seconds_app_up = loop_time.secs() - self.app_start_time;
                if self.alarm_state == 0 {
                    if seconds_app_up > 3 && seconds_app_up < 5 {
                        // Don't spam it but make sure we can't skip it either
                        displaySleep();
                        powerOffEverythingExceptESP32();
                    }
                    let mut counter_label_1: &mut Label = self.gui_renderer.elements[0]
                        .as_any()
                        .downcast_mut::<Label>()
                        .expect("Wasn't a label!");
                    let old_text = counter_label_1.text.clone();
                    let rausis_1_countdown = self.rausis_1 - seconds_app_up;
                    counter_label_1.text = Some(format!("{}", rausis_1_countdown));
                    if old_text.as_deref() != counter_label_1.text.as_deref() {
                        self.gui_renderer.needs_redraw = true;
                    }
                    self.last_check_time = loop_time.millis + 100;
                    if rausis_1_countdown == 0 {
                        self.vibrate(&vec![1000, 100, 1000, 100, 1000, 100, 1000, 100]);
                        self.alarm_state = 1;
                        self.app_start_time = millis() / 1000;
                    }
                } else if (self.alarm_state == 1) {
                    let mut counter_label_2: &mut Label = self.gui_renderer.elements[1]
                        .as_any()
                        .downcast_mut::<Label>()
                        .expect("Wasn't a label!");
                    let old_text = counter_label_2.text.clone();
                    let rausis_2_countdown = self.rausis_2 - seconds_app_up;
                    counter_label_2.text = Some(format!("{}", rausis_2_countdown));
                    if old_text.as_deref() != counter_label_2.text.as_deref() {
                        self.gui_renderer.needs_redraw = true;
                    }
                    self.last_check_time = loop_time.millis + 100;
                    if rausis_2_countdown == 0 {
                        self.vibrate(&vec![500, 1000, 500, 1000, 500, 1000, 500, 1000]);
                        self.alarm_state = 2;
                    }
                }
            }
        }
        if self.gui_renderer.will_redraw() {
            unsafe {
                fillScreen(0);
            }
        }
        self.gui_renderer.r#loop();
    }
}
