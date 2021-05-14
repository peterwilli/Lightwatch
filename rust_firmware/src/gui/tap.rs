pub trait Tap {
    fn set_callback(&mut self, c: impl FnMut() + 'static);
}