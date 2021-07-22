use libpinguin::rendering::{GuiCanvas, GuiPixel};
use crate::c_bindings::*;
use std::prelude::v1::*;
use crate::SerialLogger;

pub struct PinguinRenderer {
    width: u16,
    height: u16,
    current_pixel: GuiPixel
}

impl PinguinRenderer {
    pub fn new() -> Self {
        let mut x: u16 = 0;
        let mut y: u16 = 0;
        unsafe {
            getScreenSize(&mut x, &mut y);
        }
        return PinguinRenderer {
            width: x,
            height: y,
            current_pixel: GuiPixel::new()
        };
    }

    pub fn r#loop(&mut self, gui_canvas: &GuiCanvas<i16, i16>) {
        unsafe {
            tft_setAddrWindow(0, 0, self.width.into(), self.height.into());
            tft_startWrite();
        }
        for y in 0..self.height {
            for x in 0..self.width {
                gui_canvas.get_pixel(x as i16, y as i16, &mut self.current_pixel);
                let color = unsafe { color565(self.current_pixel.r, self.current_pixel.g, self.current_pixel.b) };
                unsafe {
                    tft_pushColor(color);
                }
            }
        }
        unsafe {
            tft_endWrite();
        }
    }
}