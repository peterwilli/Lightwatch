use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq)]
pub enum Extra {
    BackgroundLoop,
    NoThrottling,
}

pub struct SystemApplicationInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub extras: Vec<Extra>,
}

pub trait SystemApplication {
    fn new() -> Self
    where
        Self: Sized;
    fn get_info(&self) -> SystemApplicationInfo;
    fn init(&mut self);
    fn r#loop(&mut self);
    fn background_loop(&self) {}
}
