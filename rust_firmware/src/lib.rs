#![no_std]
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate lazy_static;

extern crate no_std_compat as std;
use std::prelude::v1::*;
use panic_halt as _;
mod libc_alloc;
use libc_alloc::*;
mod serial_logger;
use serial_logger::SerialLogger;
mod c_bindings;
mod non_official_c_bindings;
use crate::non_official_c_bindings::delay;
mod gui;
mod system_applications;
use system_applications::*;

extern crate alloc;

use alloc::sync::Arc;
use no_std_compat::sync::Mutex;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;

struct WatchState {
    current_application: Arc<Mutex<dyn SystemApplication>>
}

unsafe impl Send for WatchState {}
unsafe impl Sync for WatchState {}

lazy_static! {
    static ref WATCH_STATE: WatchState = WatchState {
        current_application: Arc::new(Mutex::new(HomeScreenApplication::new()))
    };
}

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    let mut current_app = WATCH_STATE.current_application.lock();
    current_app.init();

    SerialLogger::println("Test 1".to_string());
    unsafe {
        delay(1000);
    }
    SerialLogger::set_debug_in_memory(true);
    SerialLogger::println("Test 2".to_string());
    SerialLogger::println("Test 3".to_string());
    SerialLogger::set_debug_in_memory(false);

    SerialLogger::println("New Test 1".to_string());
    unsafe {
        delay(1000);
    }
    SerialLogger::set_debug_in_memory(true);
    SerialLogger::println("Test 2".to_string());
    SerialLogger::println("Test 3".to_string());
    SerialLogger::set_debug_in_memory(false);

    loop {
        current_app.r#loop();
    }
}