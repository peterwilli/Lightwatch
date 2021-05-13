use alloc::vec::Vec;
use crate::gui::GuiWidget;
use crate::c_bindings::*;
use crate::non_official_c_bindings::*;
use alloc::prelude::v1::Box;

// TODO: https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
pub struct GUIRenderer {
    pub needs_redraw: bool,
    pub elements: Vec<Box<dyn GuiWidget>>
}

unsafe impl Send for GUIRenderer {}
unsafe impl Sync for GUIRenderer {}

static mut x:i16 = 0;
static mut y:i16 = 0;
static mut last_time_render: u32 = 0;
const redraw_time: u32 = 16;

impl GUIRenderer {
    pub fn new() -> Self where Self: Sized {
        return GUIRenderer {
            elements: Vec::<Box<dyn GuiWidget>>::new(),
            needs_redraw: true
        };
    }

    pub fn r#loop(&mut self) {
        let _is_touched = unsafe {
            getTouch(&mut x, &mut y) == 1
        };
        
        // Redraw if needed
        unsafe {
            let timestamp = millis();
            if self.needs_redraw && (timestamp - last_time_render) > redraw_time {
                for element in &self.elements {
                    element.draw();
                }               
                self.needs_redraw = false; 
                last_time_render = timestamp;
            }
        }
    }
}