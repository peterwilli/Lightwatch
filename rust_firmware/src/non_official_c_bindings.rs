extern "C" {
    pub fn millis() -> u32;
    pub fn delay(ms: u32);
    pub fn digitalRead(pinNum: u8) -> u8;
    pub fn setCpuFrequencyMhz(mhz: u8);
    pub fn esp_random() -> u32;
    pub fn begin_tft_write();
    pub fn ceil(num: f64) -> f64;
    pub fn end_tft_write();
}
