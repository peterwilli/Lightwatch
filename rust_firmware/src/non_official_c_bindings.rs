extern "C" {
    pub fn millis() -> u32;
    pub fn delay(ms: u32);
    pub fn digitalRead(pinNum: u8) -> u8;
    pub fn esp_random() -> u32;
}
