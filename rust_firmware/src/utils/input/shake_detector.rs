use crate::SerialLogger;
use crate::non_official_c_bindings::*;
use alloc::format;
use std::prelude::v1::*;
use crate::c_bindings::*;

pub struct ShakeDetector {
    last_accel: Accel,
    treshold: i16,
    slow_treshold: i16,
    delay: u8,
    slow_shake_time: u32,
    slow_debounce: u32,
    slow_shake_min_shakes: u8,
    slow_shake_count: u8,
    slow_min_gap: u16
}

impl ShakeDetector {
    pub fn new(treshold: i16) -> Self {
        return ShakeDetector {
            last_accel: Accel { x: 0, y: 0, z: 0 },
            treshold: treshold,
            delay: 50,
            slow_treshold: 0,
            slow_shake_time: 0,
            slow_shake_count: 0,
            slow_shake_min_shakes: 0,
            slow_debounce: 0,
            slow_min_gap: 0
        };
    }

    pub fn enable_slow_shake(&mut self, slow_treshold: i16, slow_min_gap: u16, slow_debounce: u32, slow_shake_min_shakes: u8) {
        self.slow_treshold = slow_treshold;
        self.slow_debounce = slow_debounce;
        self.slow_shake_min_shakes = slow_shake_min_shakes;
        self.slow_min_gap = slow_min_gap;
    }

    pub fn detect(&mut self) -> bool {
        let mut accel = Accel { x: 0, y: 0, z: 0 };
        unsafe { readAccelerometer(&mut accel) };
        let is_shaken = if self.delay == 0 {
            let dX = (self.last_accel.x - accel.x).abs();
            let dY = (self.last_accel.y - accel.y).abs();
            let dZ = (self.last_accel.z - accel.z).abs();
            let dA = (dX + dY + dZ);
            SerialLogger::println(format!("dA: {}", dA));
            // false
            if self.slow_treshold > 0 && dA > self.slow_treshold.into() {
                let mut should_add_count = true;
                if self.slow_shake_time > 0 {
                    if unsafe { millis() } - self.slow_shake_time < self.slow_min_gap.into() {
                        should_add_count = false;
                    }
                }
                if should_add_count {
                    self.slow_shake_time = unsafe { millis() };
                    if self.slow_shake_count < u8::MAX {
                        self.slow_shake_count += 1;
                        SerialLogger::println(format!("slow_shake_count up: {}", self.slow_shake_count));
                    }
                }
            }
            let is_slow_shake = if self.slow_treshold > 0 && ((unsafe { millis() } - self.slow_shake_time) > self.slow_debounce) {
                let to_return = if self.slow_shake_count >= self.slow_shake_min_shakes {
                    true
                }
                else {
                    false
                };
                self.slow_shake_time = 0;
                self.slow_shake_count = 0;
                to_return
            } else {
                false
            };
            if is_slow_shake {
                SerialLogger::println("Slow shake triggered!".to_string());
            }
            is_slow_shake || dA > self.treshold.into()
        }
        else {
            self.delay -= 1;
            false   
        };

        // Copy over last known values
        self.last_accel.x = accel.x;
        self.last_accel.y = accel.y;
        self.last_accel.z = accel.z;

        return is_shaken;
    }
}