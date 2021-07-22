use core::default::Default;
use core::ops::{AddAssign, Div, Sub};
use std::fmt;
use core::convert::TryInto;
use core::fmt::Display;

pub trait GuiNumber {}

impl<T: num::PrimInt + AddAssign + PartialOrd<T> + TryInto<usize> + Div<T> + Display + std::ops::Div<Output = T> + Default> GuiNumber for T where T::Error: std::fmt::Debug {}
