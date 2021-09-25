use crate::c_bindings::*;
use crate::non_official_c_bindings::*;
use crate::utils::loop_time;

pub struct EnergyManager {
    last_interacted: u32,
    pub screen_off: bool,
}

impl EnergyManager {
    pub fn new() -> Self {
        return EnergyManager {
            last_interacted: 0,
            screen_off: false,
        };
    }

    pub fn wake(&mut self) {
        if self.screen_off {
            unsafe {
                displayWakeup();
            }
            self.screen_off = false;
        }
    }

    pub fn did_interact(&mut self) {
        unsafe {
            self.last_interacted = loop_time.millis;
        }
    }

    pub fn tick(&mut self) {
        if self.screen_off {
            unsafe {
                delay(100);
            }
            return;
        }
        if (unsafe { loop_time.millis } - self.last_interacted) > 5000 {
            unsafe {
                displaySleep();
            }
            self.screen_off = true;
        }
    }
}
