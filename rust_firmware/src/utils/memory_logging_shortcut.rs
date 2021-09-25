use crate::c_bindings::*;
use crate::input::*;
use crate::utils::loop_time;
use crate::utils::SerialLogger;
use alloc::format;
use cstr_core::CString;

static mut LAST_TRIGGER: bool = false;
static mut LAST_TIME_CURRENT_CHECK: u32 = 0;

pub fn memory_logging_shortcut_check() {
    let trigger = unsafe { touch_input.is_touched && button_input.is_pressed };
    if unsafe { LAST_TRIGGER } != trigger {
        if trigger {
            SerialLogger::set_debug_in_memory(!SerialLogger::get_debug_in_memory());
        }
        if !SerialLogger::get_debug_in_memory() {
            unsafe {
                fillRect(0, 0, 250, 30, 0);
            }
        }
        unsafe {
            LAST_TRIGGER = trigger;
        }
    }

    if SerialLogger::get_debug_in_memory() {
        let mut current: f32 = 0.0;
        unsafe {
            if (loop_time.millis - LAST_TIME_CURRENT_CHECK) > 1000 {
                LAST_TIME_CURRENT_CHECK = loop_time.millis;
                if isCharging() == 0 {
                    getBattDischargeCurrent(&mut current);
                } else {
                    getBattChargeCurrent(&mut current);
                }
                fillRect(0, 0, 250, 30, 3940);
                setTextColor(400);
                let c_str = CString::new(format!("IML ({} mA)", current).as_bytes())
                    .expect("CString::new failed");
                drawString(c_str.as_ptr(), 0, 0, 1);
                SerialLogger::println(format!("{} mA", current));
            }
        }
    }
}
