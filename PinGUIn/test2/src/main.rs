use core::ops::{AddAssign, Div};

trait GuiData {}

impl<T: AddAssign + PartialOrd<T> + Div<T> + Default> GuiData for T {}

struct Button<T: GuiData> {
    pub test: T,
}

fn main() {
    println!("Hello, world!");
}
