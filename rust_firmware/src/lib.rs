#![no_std]
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate lazy_static;

extern crate no_std_compat as std;
// use nn::{HaltCondition, NN};
use panic_halt as _;
use std::prelude::v1::*;
mod libc_alloc;
use libc_alloc::*;
mod utils;
use utils::SerialLogger;
mod c_bindings;
mod pinguin_renderer;
use c_bindings::*;
mod energy_manager;
mod non_official_c_bindings;
use non_official_c_bindings::*;
mod input;
use input::button_input;
use input::touch_input;

mod gui;
mod system_applications;
use system_applications::*;

use utils::loop_time;
use utils::memory_logging_shortcut;

extern crate alloc;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;
#[no_mangle]
pub extern "C" fn rust_bb_init() {
    let mut current_app = HomeScreenApplication::new();
    current_app.init();
    // test_stuff();
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

/*fn test_stuff() {
    let nn = NN::new(&[2, 3, 1]);
    let mut results = nn.make_vec_for_inplace(&[0.0, 0.0]);
    SerialLogger::println(format!("results: {:?}", results));
    loop {
        unsafe {
            let start_time = millis();
            let pixels_amount = 100;
            for i in 0..pixels_amount {
                let results = nn.run_inplace(&[i.into(), i.into()], &mut results);
            }
            let end_time = millis();
            SerialLogger::println(format!(
                "result time {}px: {}ms",
                pixels_amount,
                end_time - start_time
            ));
            delay(1000);
        }
    }
}*/
