use crate::input::*;

pub fn widget_is_tapped(widget_x: i16, widget_y: i16, widget_w: i16, widget_h: i16) -> bool {
    return unsafe {
        touch_input.is_touched
            && touch_input.x > widget_x
            && touch_input.y > widget_y
            && touch_input.x < (widget_w + touch_input.x)
            && touch_input.y < (widget_h + touch_input.y)
    };
}
