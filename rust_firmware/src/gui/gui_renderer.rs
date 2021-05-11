use alloc::vec::Vec;
use crate::gui::GuiWidget;
use crate::c_bindings::*;

pub struct GUIRenderer<'a> {
    pub needs_redraw: bool,
    pub elements: Vec<&'a GuiWidget>
}

unsafe impl Send for GUIRenderer<'_> {}
unsafe impl Sync for GUIRenderer<'_> {}

static mut x:i16 = 0;
static mut y:i16 = 0;
static mut last_time_render: u32 = 0;
const redraw_time: u32 = 16;

impl GUIRenderer<'_> {
    pub fn new() -> Self where Self: Sized {
        return GUIRenderer {
            elements: Vec::new(),
            needs_redraw: true
        };
    }

    pub fn r#loop(&self) {
        let is_touched = unsafe {
            getTouch(&mut x, &mut y) == 1
        };
        
        // Redraw if needed
        unsafe {
            let timestamp = millis();
            if self.needs_redraw && (timestamp - last_time_render) > redraw_time {
                for element in &self.elements {
                    element.draw();
                }
            }
        }
    }
}