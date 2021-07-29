use crate::SerialLogger;
use alloc::format;
use crate::c_bindings::*;

pub struct ShakeDetector {
    last_accel: Accel,
    treshold: i16,
    delay: u8
}

impl ShakeDetector {
    pub fn new(treshold: i16) -> Self {
        return ShakeDetector {
            last_accel: Accel { x: 0, y: 0, z: 0 },
            treshold: treshold,
            delay: 50
        };
    }

    pub fn detect(&mut self) -> bool {
        let mut accel = Accel { x: 0, y: 0, z: 0 };
        unsafe { readAccelerometer(&mut accel) };
        let is_shaken = if self.delay == 0 {
            let dX = (self.last_accel.x - accel.x).abs();
            let dY = (self.last_accel.y - accel.y).abs();
            let dZ = (self.last_accel.z - accel.z).abs();
            let dA = (dX + dY + dZ);
            // SerialLogger::println(format!("dA: {}", dA));
            // false
            dA > self.treshold.into()
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