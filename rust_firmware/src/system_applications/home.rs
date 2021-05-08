use crate::system_applications::system_application::*;
use crate::alloc::string::ToString;
use crate::gui::*;
use alloc::vec;

pub struct HomeScreenApplication;

impl SystemApplication for ActivityApplication {
    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.home".to_string(),
            name: "Home".to_string(),
            description: "Home screen".to_string(),
            extras: vec![Extra::BackgroundLoop]
        }
    }

    fn init(&self) {

    }

    fn r#loop(&self) {

    }
}