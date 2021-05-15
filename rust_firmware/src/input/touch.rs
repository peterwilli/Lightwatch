pub struct TouchInput {
    pub x: i16,
    pub y: i16,
    pub is_touched: bool,
}

pub static mut touch_input: TouchInput = TouchInput {
    x: 0,
    y: 0,
    is_touched: false,
};
