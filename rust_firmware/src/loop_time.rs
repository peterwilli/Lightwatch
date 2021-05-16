pub struct LoopTime {
    pub millis: u32,
}

pub static mut loop_time: LoopTime = LoopTime { millis: 0 };
