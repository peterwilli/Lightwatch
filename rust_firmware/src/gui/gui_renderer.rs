use crate::c_bindings::*;
use crate::gui::GuiWidget;
use crate::input::*;
use crate::non_official_c_bindings::*;
use alloc::prelude::v1::Box;
use alloc::vec::Vec;

pub struct GUIRenderer {
    pub needs_redraw: bool,
    pub elements: Vec<Box<dyn GuiWidget>>,
}

unsafe impl Send for GUIRenderer {}
unsafe impl Sync for GUIRenderer {}

static mut last_time_render: u32 = 0;
static mut last_touch_input: TouchInput = TouchInput {
    x: 0,
    y: 0,
    is_touched: false,
};
const redraw_time: u32 = 16;

impl GUIRenderer {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        return GUIRenderer {
            elements: Vec::<Box<dyn GuiWidget>>::new(),
            needs_redraw: true,
        };
    }

    pub fn r#loop(&mut self) {
        unsafe {
            if !(last_touch_input.is_touched == touch_input.is_touched
                && last_touch_input.x == touch_input.x
                && last_touch_input.y == touch_input.y)
            {
                self.needs_redraw = true;
                last_touch_input.is_touched = touch_input.is_touched;
                last_touch_input.x = touch_input.x;
                last_touch_input.y = touch_input.y;
            }
        }
        // Redraw if needed
        unsafe {
            let timestamp = millis();
            if self.needs_redraw && (timestamp - last_time_render) > redraw_time {
                self.needs_redraw = false;
                for element in &mut self.elements {
                    element.r#loop(&mut self.needs_redraw);
                }
                last_time_render = timestamp;
            }
        }
    }
}
