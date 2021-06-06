use crate::c_bindings::*;
use crate::input::*;
use crate::utils::SerialLogger;
use cstr_core::CString;

static mut LAST_TRIGGER: bool = false;

pub fn memory_logging_shortcut_check() {
    let trigger = unsafe { touch_input.is_touched && button_input.is_pressed };
    if unsafe { LAST_TRIGGER } != trigger {
        if trigger {
            SerialLogger::set_debug_in_memory(!SerialLogger::get_debug_in_memory());
            if SerialLogger::get_debug_in_memory() {
                unsafe {
                    setTextColor(400);
                    let c_str =
                        CString::new("In-memory logging".as_bytes()).expect("CString::new failed");
                    drawString(c_str.as_ptr(), 0, 0, 1);
                }
            } else {
                unsafe {
                    fillRect(0, 0, 200, 20, 3940);
                }
            }
        }
        unsafe {
            LAST_TRIGGER = trigger;
        }
    }
}
