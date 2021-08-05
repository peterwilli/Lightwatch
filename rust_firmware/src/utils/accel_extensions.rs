use crate::c_bindings::*;
use std::prelude::v1::*;

impl Accel {
    pub fn is_empty(&self) -> bool {
        return self.x == 0 && self.y == 0 && self.z == 0;
    }

    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
}
