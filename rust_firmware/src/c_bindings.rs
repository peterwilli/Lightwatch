/* automatically generated by rust-bindgen 0.58.1 */

pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bma4_accel {
    #[doc = " Accel X data"]
    pub x: i16,
    #[doc = " Accel Y data"]
    pub y: i16,
    #[doc = " Accel Z data"]
    pub z: i16,
}
#[test]
fn bindgen_test_layout_bma4_accel() {
    assert_eq!(
        ::core::mem::size_of::<bma4_accel>(),
        6usize,
        concat!("Size of: ", stringify!(bma4_accel))
    );
    assert_eq!(
        ::core::mem::align_of::<bma4_accel>(),
        2usize,
        concat!("Alignment of ", stringify!(bma4_accel))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<bma4_accel>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bma4_accel),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<bma4_accel>())).y as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(bma4_accel),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<bma4_accel>())).z as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bma4_accel),
            "::",
            stringify!(z)
        )
    );
}
pub type Accel = bma4_accel;
extern "C" {
    pub fn enableAccelerometer();
}
extern "C" {
    pub fn readAccelerometer(accel: *mut Accel) -> u8;
}
extern "C" {
    pub fn serialPrintln(text: *const cstr_core::c_char);
}
extern "C" {
    pub fn setBrightness(brightness: u8);
}
extern "C" {
    pub fn readIRQ() -> u8;
}
extern "C" {
    pub fn getPinAXP202() -> u8;
}
extern "C" {
    pub fn vibrate(duration: u8);
}
extern "C" {
    pub fn setTextSize(size: u8);
}
extern "C" {
    pub fn fillRect(x: i32, y: i32, w: i32, h: i32, color: u32);
}
extern "C" {
    pub fn fillScreen(color: u16);
}
extern "C" {
    pub fn setTextColor(c: u16);
}
extern "C" {
    pub fn drawString(string: *const cstr_core::c_char, x: i32, y: i32, font: u8) -> i16;
}
extern "C" {
    pub fn setTextDatum(datum: u8);
}
extern "C" {
    pub fn drawLine(xs: i32, ys: i32, xe: i32, ye: i32, color: u32);
}
extern "C" {
    pub fn pushImage(x0: i32, y0: i32, w: i32, h: i32, data: *mut u16);
}
extern "C" {
    pub fn getTouch(x: *mut i16, y: *mut i16) -> u8;
}