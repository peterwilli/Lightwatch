pub struct LoopTime {
    pub millis: u32,
}

impl LoopTime {
    pub fn secs(&self) -> u32 {
        self.millis / 1000
    }
}

pub static mut loop_time: LoopTime = LoopTime { millis: 0 };
