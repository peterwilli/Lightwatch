use crate::c_bindings::*;

pub trait Detector {
    fn new() -> Self
    where
        Self: Sized;

    fn push_accel(&mut self, accel: &Accel);
}
