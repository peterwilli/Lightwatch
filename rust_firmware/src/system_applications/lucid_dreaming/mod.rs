use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::input::*;
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

enum LDVibrationBreaker {
    Shake,
    Button,
}

impl LucidDreamingApplication {
    fn vibrate_while(&self, pattern: &[u16], min_wait_ms: u32, breaker: LDVibrationBreaker) {
        let vibrate_start_time = unsafe { millis() };
        let check = || -> bool {
            if unsafe { millis() } - vibrate_start_time < (min_wait_ms) {
                unsafe {
                    // To ignore any defered button presses
                    readIRQ();
                }
                return false;
            }
            if matches!(breaker, LDVibrationBreaker::Button) {
                return unsafe { readIRQ() == 1 };
            } else if matches!(breaker, LDVibrationBreaker::Shake) {
                let mut accel = Accel { x: 0, y: 0, z: 0 };
                let _x = unsafe { readAccelerometer(&mut accel) };
                let accel_avg = ((accel.x + accel.y + accel.z) / 3);
                SerialLogger::println(format!(
                    "accel: {}x{}x{} [{}]",
                    accel.x, accel.y, accel.z, accel_avg
                ));
                return accel_avg > 200;
            }
            return false;
        };
        loop {
            if check() {
                break;
            }
            for (i, pattern_piece) in pattern.iter().enumerate() {
                if i % 2 == 0 {
                    let pattern_piece_calculated = (pattern_piece / 100);
                    for i2 in 0..pattern_piece_calculated {
                        unsafe {
                            vibrate(100);
                            delay(100);
                            if check() {
                                return;
                            }
                        }
                    }
                } else {
                    let pattern_piece_calculated = (pattern_piece / 100);
                    for i2 in 0..pattern_piece_calculated {
                        unsafe {
                            delay(100);
                            if check() {
                                return;
                            }
                        }
                    }
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
                rausis_2: 0,
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
                self.vibrate_while(
                    &vec![1000, 100, 1000, 100, 1000, 100, 1000, 100],
                    25 * 1000,
                    LDVibrationBreaker::Button,
                );
                'outer: loop {
                    self.vibrate_while(&vec![100, 1000], 0, LDVibrationBreaker::Button);
                    let vibrate_end_time = unsafe { millis() };
                    loop {
                        let now_time = unsafe { millis() };
                        if now_time - vibrate_end_time < 1000 {
                            if readIRQ() == 1 {
                                break 'outer;
                            }
                        } else {
                            break;
                        }
                    }
                    // Add one minute
                    self.rausis_2 += 60;
                    SerialLogger::println("added one minute to second alarm".to_string());
                }
                SerialLogger::println(format!("second alarm set to {} seconds", self.rausis_2));
                setRTCDataAtIndex(0, 2);
                deepSleep(self.rausis_2 * 1000);
            } else if self.alarm_state == 2 {
                enableVibrator();
                enableAccelerometer();
                self.vibrate_while(&vec![500, 1000], 25 * 1000, LDVibrationBreaker::Shake);
                deepSleep(60 * 60 * 24 * 1000);
            }
        }
    }

    fn r#loop(&mut self) {}
}
