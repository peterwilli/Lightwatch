use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use alloc::vec;

pub struct ActivityApplication;

impl SystemApplication for ActivityApplication {
    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.activity".to_string(),
            name: "Activity".to_string(),
            description: "Fitness activity application".to_string(),
            extras: vec![Extra::BackgroundLoop]
        }
    }

    fn init(&self) {

    }

    fn r#loop(&self) {

    }
}