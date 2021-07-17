use core::default::Default;
use core::ops::{AddAssign, Div, Sub};
use std::fmt;

pub trait GuiNumber {}

impl<T: num::PrimInt + AddAssign + PartialOrd<T> + Div<T> + std::ops::Div<Output = T> + Default> GuiNumber for T {}
