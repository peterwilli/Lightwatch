#![no_std]
#![feature(default_alloc_error_handler)]
extern crate no_std_compat as std;
use panic_halt as _;
mod libc_alloc;
use libc_alloc::*;
mod c_bindings;
use c_bindings::*;
extern crate alloc;
use std::prelude::v1::*;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    unsafe {
        fillScreen(10);
    }
    let fun = vec!["sfsdf"];
}