#![no_std]
#![feature(default_alloc_error_handler)]
use panic_halt as _;
use cstr_core::{CString, CStr};
use cstr_core::c_char;

mod libc_alloc;
use libc_alloc::*;

mod c_bindings;
use c_bindings::*;

mod gui;
mod system_applications;

extern crate alloc;
use alloc::vec;
use alloc::format;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    unsafe {
        fillScreen(10);
        let hello = CString::new("Hello from rust").expect("CString::new failed");
        serialPrintln(hello.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn rust_bb_loop() {
    unsafe {
        let mut x:i16 = 0;
        let mut y:i16 = 0;
        let is_touched = getTouch(&mut x, &mut y) == 1;
        if is_touched {
            let hello = CString::new(format!("is_touched: {} {}", x, y)).expect("CString::new failed");
            // serialPrintln(hello.as_ptr());
            fillScreen(10);
            setTextColor(400);
            drawString(hello.as_ptr(), 10, 10, 7);
            drawLine(3,3, 200, 100, 99999);
        }
    }
}