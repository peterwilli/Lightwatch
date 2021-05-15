use crate::touch_event::TouchEvent;

pub fn widget_is_tapped(
    widget_x: i16,
    widget_y: i16,
    widget_w: i16,
    widget_h: i16,
    touch_event: &TouchEvent,
) -> bool {
    return touch_event.is_touched
        && touch_event.x > widget_x
        && touch_event.y > widget_y
        && touch_event.x < (widget_w + touch_event.x)
        && touch_event.y < (widget_h + touch_event.y);
}
