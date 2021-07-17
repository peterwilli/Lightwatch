use core::default::Default;
use core::ops::{AddAssign, Div, Sub};
use std::fmt;

pub trait GuiNumber {}

impl<T: num::PrimInt + AddAssign + PartialOrd<T> + Div<T> + core::fmt::Display + Default> GuiNumber for T {}
