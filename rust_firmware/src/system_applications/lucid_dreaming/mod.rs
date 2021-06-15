use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::input::*;
use crate::non_official_c_bindings::*;
use crate::system_applications::system_application::*;
use crate::utils::loop_time;
use crate::SerialLogger;
use alloc::format;
use alloc::sync::Arc;
use alloc::vec;
use core::ffi::c_void;
use cstr_core::CString;
use no_std_compat::sync::Mutex;
use std::prelude::v1::*;

pub struct LucidDreamingApplication {
    gui_renderer: GUIRenderer,
    app_start_time: u32,
    last_check_time: u32,
    rausis_selected_hours: Arc<Mutex<u8>>,
    rausis2_selected_minutes: Arc<Mutex<u8>>,
    rausis_1: u32,
    rausis_2: u32,
    alarm_state: u8,
    data_updated: Arc<Mutex<bool>>,
}

enum LDVibrationBreaker {
    Shake,
    Button,
}

impl LucidDreamingApplication {
    fn update_rausis1(
        rausis_selected_hours_lock: &Arc<Mutex<u8>>,
        data_updated_lock: &Arc<Mutex<bool>>,
        new_hours: u8,
    ) {
        SerialLogger::println(format!("new_hours: {}", new_hours));
        *rausis_selected_hours_lock.lock() = new_hours;
        *data_updated_lock.lock() = true;
    }

    fn update_rausis2(
        rausis2_selected_minutes_lock: &Arc<Mutex<u8>>,
        data_updated_lock: &Arc<Mutex<bool>>,
        new_minutes: u8,
    ) {
        SerialLogger::println(format!("new_minutes: {}", new_minutes));
        *rausis2_selected_minutes_lock.lock() = new_minutes;
        *data_updated_lock.lock() = true;
    }

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
                return accel_avg > 50;
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

    fn add_1_hours_selector(&mut self) {
        let mut label = Box::new(Label::new(50, 50, 70, 20));
        label.font_size = 1;
        label.text = Some("Hours".to_string());
        self.gui_renderer.elements.push(label);

        let mut button = Box::new(Button::new(50, 10, 40, 40));
        button.text = Some("Up".to_string());
        let rausis_selected_hours_lock = self.rausis_selected_hours.clone();
        let data_updated_lock = self.data_updated.clone();
        button.on_tap = Some(Box::new(move || {
            let new_hours = *rausis_selected_hours_lock.lock() + 1;
            LucidDreamingApplication::update_rausis1(
                &rausis_selected_hours_lock,
                &data_updated_lock,
                new_hours,
            )
        }));
        self.gui_renderer.elements.push(button);

        let mut button = Box::new(Button::new(50, 80, 40, 40));
        button.text = Some("Down".to_string());
        let rausis_selected_hours_lock = self.rausis_selected_hours.clone();
        let data_updated_lock = self.data_updated.clone();
        button.on_tap = Some(Box::new(move || {
            if *rausis_selected_hours_lock.lock() > 0 {
                let new_hours = *rausis_selected_hours_lock.lock() - 1;
                LucidDreamingApplication::update_rausis1(
                    &rausis_selected_hours_lock,
                    &data_updated_lock,
                    new_hours,
                )
            }
        }));
        self.gui_renderer.elements.push(button);
    }

    fn add_2_minutes_selector(&mut self) {
        let start_x = 150;
        let mut label = Box::new(Label::new(start_x, 50, 70, 20));
        label.font_size = 1;
        label.text = Some("Hours".to_string());
        self.gui_renderer.elements.push(label);
        let mut button = Box::new(Button::new(start_x, 10, 40, 40));
        button.text = Some("Up".to_string());
        let rausis2_selected_minutes_lock = self.rausis2_selected_minutes.clone();
        let data_updated_lock = self.data_updated.clone();
        button.on_tap = Some(Box::new(move || {
            let new_minutes = *rausis2_selected_minutes_lock.lock() + 1;
            LucidDreamingApplication::update_rausis2(
                &rausis2_selected_minutes_lock,
                &data_updated_lock,
                new_minutes,
            )
        }));
        self.gui_renderer.elements.push(button);

        let mut button = Box::new(Button::new(start_x, 80, 40, 40));
        button.text = Some("Down".to_string());
        let rausis2_selected_minutes_lock = self.rausis2_selected_minutes.clone();
        let data_updated_lock = self.data_updated.clone();
        button.on_tap = Some(Box::new(move || {
            if *rausis2_selected_minutes_lock.lock() > 0 {
                let new_minutes = *rausis2_selected_minutes_lock.lock() - 1;
                LucidDreamingApplication::update_rausis2(
                    &rausis2_selected_minutes_lock,
                    &data_updated_lock,
                    new_minutes,
                )
            }
        }));
        self.gui_renderer.elements.push(button);
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
                rausis_selected_hours: Arc::new(Mutex::new(0)),
                rausis2_selected_minutes: Arc::new(Mutex::new(0)),
                alarm_state: unsafe { getRTCDataAtIndex(0) },
                data_updated: Arc::new(Mutex::new(false)),
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
            setBrightness(50);
        }
        self.app_start_time = unsafe { loop_time.secs() };
        unsafe {
            fillScreen(0);
            setTextColor(400);
        }
        if self.alarm_state == 0 {
            let mut x: u16 = 0;
            let mut y: u16 = 0;
            unsafe {
                getScreenSize(&mut x, &mut y);
            }
            // Buttons
            self.add_1_hours_selector();
            self.add_2_minutes_selector();
            let mut button = Box::new(Button::new(50, 200, 40, 40));
            button.text = Some("Start".to_string());
            let rausis_selected_hours_lock = self.rausis_selected_hours.clone();
            let rausis2_selected_minutes_lock = self.rausis2_selected_minutes.clone();
            button.on_tap = Some(Box::new(move || {
                let deep_sleep_secs = *rausis_selected_hours_lock.lock() as u32 * 60 * 60;
                // let deep_sleep_secs = 10;
                if deep_sleep_secs > 0 {
                    SerialLogger::println(format!("deep sleep for {} seconds", deep_sleep_secs));
                    unsafe {
                        setRTCDataAtIndex(0, 1);
                        setRTCDataAtIndex(1, *rausis2_selected_minutes_lock.lock());
                        deepSleep(deep_sleep_secs * 1000);
                    }
                }
            }));
            self.gui_renderer.elements.push(button);
        } else if self.alarm_state == 1 {
            unsafe {
                enableVibrator();
            }
            self.vibrate_while(
                &vec![1000, 100, 1000, 100, 1000, 100, 1000, 100],
                1 * 1000,
                LDVibrationBreaker::Button,
            );
            let preset_mins = unsafe { getRTCDataAtIndex(1) };
            if preset_mins == 0 {
                'outer: loop {
                    self.vibrate_while(&vec![100, 1000], 0, LDVibrationBreaker::Button);
                    let vibrate_end_time = unsafe { millis() };
                    loop {
                        let now_time = unsafe { millis() };
                        if now_time - vibrate_end_time < 1000 {
                            if unsafe { readIRQ() } == 1 {
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
            } else {
                self.rausis_2 = (preset_mins as u32) * 60;
            }
            SerialLogger::println(format!("second alarm set to {} seconds", self.rausis_2));
            unsafe {
                setRTCDataAtIndex(0, 2);
                deepSleep(self.rausis_2 * 1000);
            }
        } else if self.alarm_state == 2 {
            unsafe {
                enableVibrator();
                enableAccelerometer();
            }
            self.vibrate_while(&vec![500, 1000], 1 * 1000, LDVibrationBreaker::Shake);
            unsafe {
                deepSleep(60 * 60 * 24 * 1000);
            }
        }
    }

    fn r#loop(&mut self) {
        if self.alarm_state == 0 && unsafe { readIRQ() == 1 } {
            unsafe {
                // Immediately go to the first alarm so you can set the second
                setRTCDataAtIndex(0, 1);
                deepSleep(1000);
            }
        }
        if self.gui_renderer.will_redraw() {
            unsafe {
                fillScreen(0);
            }
        }
        self.gui_renderer.r#loop();
        if *self.data_updated.lock() {
            *self.data_updated.lock() = false;
            let mut counter_label: &mut Label = self.gui_renderer.elements[0]
                .as_any()
                .downcast_mut::<Label>()
                .expect("Wasn't a label!");
            let old_text = counter_label.text.clone();
            counter_label.text = Some(format!("{} hours", *self.rausis_selected_hours.lock()));
            if old_text.as_deref() != counter_label.text.as_deref() {
                self.gui_renderer.needs_redraw = true;
            }

            let mut counter_label: &mut Label = self.gui_renderer.elements[3]
                .as_any()
                .downcast_mut::<Label>()
                .expect("Wasn't a label!");
            let old_text = counter_label.text.clone();
            counter_label.text = Some(format!("{} minutes", *self.rausis2_selected_minutes.lock()));
            if old_text.as_deref() != counter_label.text.as_deref() {
                self.gui_renderer.needs_redraw = true;
            }
        }
    }
}
