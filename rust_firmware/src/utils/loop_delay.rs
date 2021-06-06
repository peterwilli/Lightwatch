use crate::utils::loop_time;

pub struct LoopDelay {
    start_delay_time: u32,
    delay_time: u32,
}

impl LoopDelay {
    pub fn new() -> Self {
        return LoopDelay {
            start_delay_time: 0,
            delay_time: 0,
        };
    }

    pub fn delay(&mut self, ms: u32) -> bool {
        if self.delay_time == 0 {
            self.delay_time = ms;
            self.start_delay_time = unsafe { loop_time.millis };
        } else {
            if (unsafe { loop_time.millis } - self.start_delay_time) > self.delay_time {
                self.delay_time = 0;
                return true;
            }
        }

        return false;
    }
}
