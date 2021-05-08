pub trait GuiWidget {
    fn init(x: i16, y: i16, w: i16, h: i16) -> GuiWidget where Self: Sized;
    fn draw(&self);
}