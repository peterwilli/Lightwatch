#![no_std]
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate lazy_static;

extern crate no_std_compat as std;
use panic_halt as _;
use std::prelude::v1::*;
mod libc_alloc;
use libc_alloc::*;
mod serial_logger;
use serial_logger::SerialLogger;
mod c_bindings;
use c_bindings::*;
mod non_official_c_bindings;
use non_official_c_bindings::*;
mod input;
use input::button_input;
use input::touch_input;

mod memory_logging_shortcut;

mod gui;
mod system_applications;
use system_applications::*;

mod loop_time;
use loop_time::loop_time;

extern crate alloc;

use alloc::sync::Arc;
use no_std_compat::sync::Mutex;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;
#[no_mangle]
pub extern "C" fn rust_bb_init() {
    unsafe {
        setBrightness(150);
    }
    let mut current_app = HomeScreenApplication::new();
    current_app.init();
    loop {
        unsafe {
            loop_time.millis = millis();
            let is_touched = getTouch(&mut touch_input.x, &mut touch_input.y) == 1;
            touch_input.is_touched = is_touched;
            button_input.is_pressed = readIRQ() == 1;
        }
        memory_logging_shortcut::memory_logging_shortcut_check();
        current_app.r#loop();
    }
}
