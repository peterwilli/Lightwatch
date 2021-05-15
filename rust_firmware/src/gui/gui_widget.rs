use crate::input::*;

pub trait GuiWidget {
    fn new(x: i16, y: i16, w: i16, h: i16) -> Self
    where
        Self: Sized;
    fn r#loop(&mut self, needs_redraw: &mut bool);
}
