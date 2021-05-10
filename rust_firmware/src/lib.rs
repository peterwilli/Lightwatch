#![no_std]
#![feature(default_alloc_error_handler)]
#[macro_use]
extern crate lazy_static;
extern crate no_std_compat as std;
use std::prelude::v1::*;
use mutex_trait::Mutex;
use panic_halt as _;
use cstr_core::{CString, CStr};
use cstr_core::c_char;

mod libc_alloc;
use libc_alloc::*;

mod c_bindings;
use c_bindings::*;

mod gui;
mod system_applications;
use system_applications::*;

extern crate alloc;
use alloc::vec;
use alloc::format;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;

struct WatchState {
    currentApplication: &SystemApplication
}

lazy_static! {
    static ref WATCH_STATE: mutex_trait::Mutex<WatchState> = Mutex::new(WatchState {
        currentApplication: &EmptyApplication
    });
}

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    unsafe {
        fillScreen(10);
        let hello = CString::new("Hello from rust").expect("CString::new failed");
        serialPrintln(hello.as_ptr());
        setTextColor(400);
        drawLine(3,3, 200, 100, 99999);
        WATCH_STATE.lock().unwrap().currentApplication = HomeScreenApplication::new();
        WATCH_STATE.lock().unwrap().currentApplication.init();
    }
}

#[no_mangle]
pub extern "C" fn rust_bb_loop() {
    unsafe {
        let mut x:i16 = 0;
        let mut y:i16 = 0;
        let is_touched = getTouch(&mut x, &mut y) == 1;
        if is_touched {
            let hello = CString::new(format!("{} {}", x, y)).expect("CString::new failed");
            // serialPrintln(hello.as_ptr());
            fillScreen(10);
            drawString(hello.as_ptr(), 10, 10, 7);
        }

        CURRENT_APPLICATION.r#loop();
    }
}