#![no_std]
use panic_halt as _;
mod c_bindings;
use c_bindings::*;

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    unsafe {
        fillScreen(10);
    }
}