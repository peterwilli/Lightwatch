use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::input::touch_input;
use crate::input::*;
use crate::non_official_c_bindings::*;
use crate::system_applications::system_application::*;
use crate::utils::loop_time;
use crate::utils::ShakeDetector;
use crate::SerialLogger;
use alloc::format;
use alloc::sync::Arc;
use alloc::vec;
use core::convert::TryInto;
use core::ffi::c_void;
use cstr_core::CString;
use no_std_compat::sync::Mutex;
use std::cmp;
use std::prelude::v1::*;

static test: bool = false;

pub struct LucidDreamingApplication {
    gui_renderer: GUIRenderer,
    app_start_time: u32,
    last_check_time: u32,
    rausis_selected_hours: Arc<Mutex<u8>>,
    rausis2_selected_minutes: Arc<Mutex<u8>>,
    rausis_2: u32,
    alarm_state: u8,
    data_updated: Arc<Mutex<bool>>,
}

enum LDVibrationBreaker {
    Shake,
    Button,
    ButtonCount,
    AutoDismiss,
    ShakeAutoDismiss,
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

    fn random_vibration_pattern() -> Vec<u16> {
        // Silence not included
        let pattern_length = 2 + (unsafe { esp_random() } % 5);
        let mut result: Vec<u16> = Vec::new();
        for i in 0..pattern_length {
            let vibration_start = 200 + ((unsafe { esp_random() } % 700) as u16);
            let vibration_end = 200 + ((unsafe { esp_random() } % 700) as u16);
            result.push(vibration_start);
            result.push(vibration_end);
        }

        let vibration_starts: Vec<u16> = result.clone().into_iter()
            .enumerate().filter(|&(i, _)| i % 2 == 0)
            .map(|(_, v)| v)
            .collect();
        let max_diff_start = vibration_starts.iter().max().unwrap() - vibration_starts.iter().min().unwrap();

        let vibration_ends: Vec<u16> = result.clone().into_iter()
            .enumerate().filter(|&(i, _)| i % 2 == 1)
            .map(|(_, v)| v)
            .collect();
        let max_diff_end = vibration_ends.iter().max().unwrap() - vibration_ends.iter().min().unwrap();
        SerialLogger::println(format!("random_vibration_pattern() max_diff_end: {} max_diff_start: {}", max_diff_end, max_diff_start));
        if max_diff_end < 50 && max_diff_start < 50 {
            // Vibration is too static (makes it too much like the second alarm), trying again
            return Self::random_vibration_pattern();
        }
        return result;
    }

    fn vibrate_while(
        &self,
        pattern: &[u16],
        min_wait_ms: u32,
        breaker: LDVibrationBreaker,
    ) -> LDVibrationBreaker {
        let vibrate_start_time = unsafe { millis() };
        let mut last_vibrate_time = vibrate_start_time;
        let button_count = Mutex::new(0 as u8);
        let mut shake_detector = ShakeDetector::new(700);
        shake_detector.enable_slow_shake(900, 10000, 3);
        let mut check = |last_vibrate_time: u32| -> Option<LDVibrationBreaker> {
            if unsafe { millis() } - vibrate_start_time < min_wait_ms {
                if matches!(breaker, LDVibrationBreaker::ShakeAutoDismiss) {
                    let possible_shake_detect = shake_detector.detect();
                    if possible_shake_detect {
                        return Some(LDVibrationBreaker::Shake);
                    }
                }
                unsafe {
                    // To ignore any defered button presses
                    readIRQ();
                }
                return None;
            }
            if matches!(breaker, LDVibrationBreaker::Button) {
                if unsafe { readIRQ() == 1 } {
                    return Some(LDVibrationBreaker::Button);
                }
            } else if matches!(breaker, LDVibrationBreaker::Shake) {
                if shake_detector.detect() {
                    return Some(LDVibrationBreaker::Shake);
                }
            } else if matches!(breaker, LDVibrationBreaker::AutoDismiss)
                || matches!(breaker, LDVibrationBreaker::ShakeAutoDismiss)
            {
                return Some(LDVibrationBreaker::AutoDismiss);
            } else if matches!(breaker, LDVibrationBreaker::ButtonCount) {
                if unsafe { readIRQ() == 1 } {
                    let button_press_time = unsafe { millis() };
                    let button_vibration_diff = button_press_time - last_vibrate_time;
                    SerialLogger::println(format!(
                        "button_press_time: {} last_vibrate_time: {} diff: {}",
                        button_press_time, last_vibrate_time, button_vibration_diff
                    ));
                    if button_vibration_diff <= 200 {
                        let orig = *button_count.lock();
                        *button_count.lock() = orig + 1;
                        let score_tresh = if test { 1 } else { 10 };

                        if *button_count.lock() >= score_tresh {
                            return Some(LDVibrationBreaker::ButtonCount);
                        }
                    } else {
                        // Reset the score
                        // *button_count.lock() = 0;
                    }
                }
            }
            return None;
        };
        loop {
            let triggered_breaker = check(last_vibrate_time);
            if triggered_breaker.is_some() {
                return triggered_breaker.unwrap();
            }
            let divider: u8 = 10;
            for (i, pattern_piece) in pattern.iter().enumerate() {
                if i % 2 == 0 {
                    let pattern_piece_calculated = (pattern_piece / (divider as u16));
                    for i2 in 0..pattern_piece_calculated {
                        last_vibrate_time = unsafe { millis() };
                        unsafe {
                            vibrate(divider);
                            delay(divider.into());
                            let triggered_breaker = check(last_vibrate_time);
                            if triggered_breaker.is_some() {
                                // To give more precise timing
                                let triggered_breaker = triggered_breaker.unwrap();
                                if !matches!(triggered_breaker, LDVibrationBreaker::AutoDismiss) {
                                    return triggered_breaker;
                                }
                            }
                        }
                    }
                } else {
                    let pattern_piece_calculated = (pattern_piece / (divider as u16));
                    for i2 in 0..pattern_piece_calculated {
                        unsafe {
                            delay(divider.into());
                            let triggered_breaker = check(last_vibrate_time);
                            if triggered_breaker.is_some() {
                                // To give more precise timing
                                let triggered_breaker = triggered_breaker.unwrap();
                                if !matches!(triggered_breaker, LDVibrationBreaker::AutoDismiss) {
                                    return triggered_breaker;
                                }
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
                let deep_sleep_secs = if test {
                    1
                } else {
                    *rausis_selected_hours_lock.lock() as u32 * 60 * 60
                };
                if deep_sleep_secs > 0 {
                    SerialLogger::println(format!("deep sleep for {} seconds", deep_sleep_secs));
                    unsafe {
                        setRTCDataAtIndex(0, 1);
                        setRTCDataAtIndex(
                            1,
                            if test {
                                1
                            } else {
                                *rausis2_selected_minutes_lock.lock()
                            },
                        );
                        deepSleep(deep_sleep_secs * 1000);
                    }
                }
            }));
            self.gui_renderer.elements.push(button);
        } else if self.alarm_state == 1 {
            unsafe {
                setBrightness(0);
                enableVibrator();
            }
            let random_vibration_pattern = Self::random_vibration_pattern();
            self.vibrate_while(
                &random_vibration_pattern,
                0,
                LDVibrationBreaker::ButtonCount,
            );
            let preset_mins = unsafe { getRTCDataAtIndex(1) };
            if preset_mins == 0 {
                // TODO: add regular minutes to rausis_2 rather than handling seconds here..
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
                if test {
                    self.rausis_2 = 5;
                }
            }
            SerialLogger::println(format!("second alarm set to {} seconds", self.rausis_2));
            unsafe {
                setRTCDataAtIndex(0, 2);
                deepSleep(self.rausis_2 * 1000);
            }
        } else if self.alarm_state == 2 {
            unsafe {
                setBrightness(0);
                enableVibrator();
                enableAccelerometer();
            }
            let pre_second_end_trigger = self.vibrate_while(
                &vec![10, 5000],
                60 * 1000,
                LDVibrationBreaker::ShakeAutoDismiss,
            );
            if matches!(pre_second_end_trigger, LDVibrationBreaker::Shake) {
                SerialLogger::println("Shaken, so we need to retry again".to_string());
                // We also bump the second alarm with 1 minute.
                let current_mins = unsafe { getRTCDataAtIndex(1) };
                let to_add_mins: u8 = cmp::max(1, (current_mins + 1) / 2);
                let preset_mins = unsafe { getRTCDataAtIndex(1) } + to_add_mins;
                SerialLogger::println(format!(
                    "current_mins: {} new preset_mins: {} to_add_mins: {}",
                    current_mins, preset_mins, to_add_mins
                ));
                unsafe {
                    setRTCDataAtIndex(1, preset_mins);
                }
                unsafe {
                    setRTCDataAtIndex(0, 2);
                }
                self.rausis_2 = (preset_mins as u32) * 60;
                if test {
                    self.rausis_2 = 5;
                }
                SerialLogger::println(format!("second alarm set to {} seconds", self.rausis_2));
                unsafe {
                    deepSleep(self.rausis_2 * 1000);
                }
            }
            for i in 0..10 {
                let pattern = vec![(i + 1) * 10, 500];
                let last_trigger = self.vibrate_while(
                    &pattern,
                    if test { 1000 } else { 1000 * 10 },
                    LDVibrationBreaker::ShakeAutoDismiss,
                );
                if matches!(last_trigger, LDVibrationBreaker::Shake) {
                    unsafe {
                        deepSleep(60 * 60 * 24 * 1000);
                    }
                }
            }
            self.vibrate_while(&vec![100, 500], 0, LDVibrationBreaker::Shake);
            unsafe {
                deepSleep(60 * 60 * 24 * 1000);
            }
        }
    }

    fn r#loop(&mut self) {
        if self.alarm_state == 0 && unsafe { button_input.is_pressed } {
            unsafe {
                // Immediately go to the first alarm so you can set the second
                setRTCDataAtIndex(0, 1);
                deepSleep(1000);
                return;
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
