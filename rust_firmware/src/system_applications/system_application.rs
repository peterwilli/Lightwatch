use alloc::string::String;
use alloc::vec::Vec;

pub enum Extra {
    BackgroundLoop
}

pub struct SystemApplicationInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub extras: Vec<Extra>
}

pub trait SystemApplication {
    fn new() -> Self where Self: Sized;
    fn get_info(&self) -> SystemApplicationInfo;
    fn init(&self);
    fn r#loop(&self);
    fn background_loop(&self) {}
}