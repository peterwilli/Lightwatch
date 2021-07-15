use core::default::Default;
use core::ops::{AddAssign, Div, Sub};

pub trait GuiNumber {}

impl<T: num::PrimInt + AddAssign + PartialOrd<T> + Div<T> + Default> GuiNumber for T {}
