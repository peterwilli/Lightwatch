#![no_std]
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate lazy_static;

extern crate no_std_compat as std;
use std::prelude::v1::*;
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
use alloc::sync::Arc;
use no_std_compat::sync::Mutex;

#[global_allocator]
static A: LibcAllocator = LibcAllocator;

struct WatchState {
    currentApplication: Arc<Mutex<SystemApplication>>
}

unsafe impl Send for WatchState {}
unsafe impl Sync for WatchState {}

lazy_static! {
    static ref WATCH_STATE: WatchState = WatchState {
        currentApplication: Arc::new(Mutex::new(HomeScreenApplication::new()))
    };
}

#[no_mangle]
pub extern "C" fn rust_bb_init() {
    let mut currentApp = WATCH_STATE.currentApplication.lock();
    currentApp.init();
    loop {
        currentApp.r#loop();
    }
}