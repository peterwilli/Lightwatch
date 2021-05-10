use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use crate::gui::*;
use alloc::vec;

pub struct EmptyApplication;

impl SystemApplication for EmptyApplication {
    fn new() -> Self {
        return {
            EmptyApplication {

            }
        }
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.empty".to_string(),
            name: "Empty".to_string(),
            description: "Does nothing".to_string(),
            extras: vec![]
        }
    }

    fn init(&self) {

    }

    fn r#loop(&self) {

    }
}