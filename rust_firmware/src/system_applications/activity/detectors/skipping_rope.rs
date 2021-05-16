use super::Detector;
use crate::c_bindings::*;

pub enum JumpState {
    High,
    Low,
}

pub struct SkippingRopeDetector {
    pub jump_state: JumpState,
    pub jump_count: u16,
}

impl Detector for SkippingRopeDetector {
    fn new() -> Self {
        return SkippingRopeDetector {
            jump_state: JumpState::Low,
            jump_count: 0,
        };
    }

    fn push_accel(&mut self, accel: &Accel) {
        let xy_diff = (accel.x - accel.y).abs();
        if xy_diff > 3000 {
            self.jump_count = self.jump_count + 1;
        }
    }
}
