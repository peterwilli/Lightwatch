use crate::c_bindings::*;
use crate::non_official_c_bindings::*;
use crate::SerialLogger;
use alloc::format;
use std::prelude::v1::*;

pub struct ShakeDetector {
    last_accel: Accel,
    last_accel_slow: Accel,
    treshold: i16,
    slow_treshold: i16,
    delay: u8,
    slow_shake_time: u32,
    slow_max_time: u32,
    slow_shake_min_shakes: u8,
    slow_shake_count: u8,
}

impl ShakeDetector {
    pub fn new(treshold: i16) -> Self {
        return ShakeDetector {
            last_accel: Accel { x: 0, y: 0, z: 0 },
            last_accel_slow: Accel { x: 0, y: 0, z: 0 },
            treshold: treshold,
            delay: 50,
            slow_treshold: 0,
            slow_shake_time: 0,
            slow_shake_count: 0,
            slow_shake_min_shakes: 0,
            slow_max_time: 0,
        };
    }

    pub fn enable_slow_shake(
        &mut self,
        slow_treshold: i16,
        slow_max_time: u32,
        slow_shake_min_shakes: u8,
    ) {
        self.slow_treshold = slow_treshold;
        self.slow_max_time = slow_max_time;
        self.slow_shake_min_shakes = slow_shake_min_shakes;
    }

    fn has_slow_shake(&self) -> bool {
        return self.slow_max_time > 0;
    }

    pub fn detect(&mut self) -> bool {
        let mut accel = Accel { x: 0, y: 0, z: 0 };
        unsafe { readAccelerometer(&mut accel) };
        let is_shaken = if self.delay == 0 {
            let mut is_slow_shake = false;
            if self.has_slow_shake() {
                if self.last_accel_slow.is_empty() {
                    self.last_accel_slow.x = accel.x;
                    self.last_accel_slow.y = accel.y;
                    self.last_accel_slow.z = accel.z;
                    self.slow_shake_time = unsafe { millis() };
                } else if (unsafe { millis() } - self.slow_shake_time) > self.slow_max_time {
                    self.last_accel_slow.clear();
                    self.slow_shake_count = 0;
                } else {
                    let dX = (self.last_accel_slow.x - accel.x).abs();
                    let dY = (self.last_accel_slow.y - accel.y).abs();
                    let dZ = (self.last_accel_slow.z - accel.z).abs();
                    let dA = (dX + dY + dZ);
                    let is_individual_slow_shake = dA > self.slow_treshold.into();
                    if is_individual_slow_shake {
                        self.slow_shake_count += 1;
                        self.last_accel_slow.clear();
                    }
                    // SerialLogger::println(format!("slow_shake_count: {}", self.slow_shake_count));
                    if self.slow_shake_count >= self.slow_shake_min_shakes {
                        is_slow_shake = true;
                    }
                }
            }
            if is_slow_shake {
                SerialLogger::println("Slow shake triggered!".to_string());
            }
            let dX = (self.last_accel.x - accel.x).abs();
            let dY = (self.last_accel.y - accel.y).abs();
            let dZ = (self.last_accel.z - accel.z).abs();
            let dA = (dX + dY + dZ);
            // SerialLogger::println(format!("dA: {}", dA));
            is_slow_shake || dA > self.treshold.into()
        } else {
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
