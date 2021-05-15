use crate::c_bindings::serialPrintln;
use alloc::string::String;
use alloc::vec::Vec;
use cstr_core::CString;
use no_std_compat::sync::Mutex;

lazy_static! {
    static ref DEBUG_IN_MEMORY: Mutex<bool> = Mutex::new(false);
    static ref LIST_OF_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());
}

pub struct SerialLogger;

impl SerialLogger {
    pub fn println(text: String) {
        if *DEBUG_IN_MEMORY.lock() {
            LIST_OF_LOGS.lock().push(text);
        } else {
            let c_str = CString::new(text.as_bytes()).expect("CString::new failed");
            unsafe {
                serialPrintln(c_str.as_ptr());
            }
        }
    }

    pub fn get_debug_in_memory() -> bool {
        return *DEBUG_IN_MEMORY.lock();
    }

    pub fn set_debug_in_memory(debug_in_memory: bool) {
        let mut debug_in_memory_ = DEBUG_IN_MEMORY.lock();
        if *debug_in_memory_ != debug_in_memory {
            // The toggle updated
            let mut logs = LIST_OF_LOGS.lock();
            for line in &*logs {
                let c_str = CString::new(line.as_bytes()).expect("CString::new failed");
                unsafe {
                    serialPrintln(c_str.as_ptr());
                }
            }
            logs.clear();
        }
        *debug_in_memory_ = debug_in_memory;
    }
}
