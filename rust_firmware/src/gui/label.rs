use crate::gui::GuiWidget;
use crate::c_bindings::*;

pub struct Label {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub mut text: Option<String>,
    pub mut font: u8 = 1
}

impl GuiWidget for Label {
    fn init(x: i16, y: i16, w: i16, h: i16) -> GuiWidget where Self: Sized {
        return Label {
            x: x,
            y: y,
            w: w,
            h: h,
            text: None
        };
    }

    fn draw(&self) {
        unsafe {
            let c_str = CString::new(self.text.unwrap()).expect("CString::new failed");
            drawString(c_str.as_ptr(), self.x, self.y, 0);
        }
    }
}