use alloc::vec::Vec;
use crate::gui::GuiWidget;
use crate::c_bindings::*;
use crate::non_official_c_bindings::*;
use alloc::prelude::v1::Box;
use crate::gui::touch_event::TouchEvent;

pub struct GUIRenderer {
    pub needs_redraw: bool,
    pub elements: Vec<Box<dyn GuiWidget>>
}

unsafe impl Send for GUIRenderer {}
unsafe impl Sync for GUIRenderer {}

static mut last_time_render: u32 = 0;
static mut touch_event: TouchEvent = TouchEvent {
    x: 0,
    y: 0,
    is_touched: false
};
static mut last_touch_event: TouchEvent = TouchEvent {
    x: 0,
    y: 0,
    is_touched: false
};
const redraw_time: u32 = 16;

impl GUIRenderer {
    pub fn new() -> Self where Self: Sized {
        return GUIRenderer {
            elements: Vec::<Box<dyn GuiWidget>>::new(),
            needs_redraw: true
        };
    }

    pub fn r#loop(&mut self) {
        let is_touched = unsafe {
            getTouch(&mut touch_event.x, &mut touch_event.y) == 1
        };
        unsafe {
            touch_event.is_touched = is_touched;
            if !(last_touch_event.is_touched == touch_event.is_touched && last_touch_event.x == touch_event.x && last_touch_event.y == touch_event.y) {
                self.needs_redraw = true;
                last_touch_event.is_touched = touch_event.is_touched;
                last_touch_event.x = touch_event.x;
                last_touch_event.y = touch_event.y;
            }
        }
        
        // Redraw if needed
        unsafe {
            let timestamp = millis();
            if self.needs_redraw && (timestamp - last_time_render) > redraw_time {
                self.needs_redraw = false; 
                for element in &mut self.elements {
                    element.r#loop(&touch_event, &mut self.needs_redraw);
                }
                last_time_render = timestamp;
            }
        }
    }
}