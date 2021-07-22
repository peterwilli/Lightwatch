use libpinguin::rendering::{GuiCanvas, GuiPixel};
use crate::c_bindings::*;

pub struct PinguinRenderer<'a> {
    gui_canvas: &'a GuiCanvas<i16, i16>,
    width: u16,
    height: u16,
    current_pixel: GuiPixel
}

impl PinguinRenderer<'_> {
    pub fn new(gui_canvas: &'static GuiCanvas<i16, i16>) -> Self {
        let mut x: u16 = 0;
        let mut y: u16 = 0;
        unsafe {
            getScreenSize(&mut x, &mut y);
        }
        return PinguinRenderer {
            gui_canvas: gui_canvas,
            width: x,
            height: y,
            current_pixel: GuiPixel::new()
        };
    }

    pub fn r#loop(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.gui_canvas.get_pixel(x as i16, y as i16, &mut self.current_pixel);
            }
        }
    }
}