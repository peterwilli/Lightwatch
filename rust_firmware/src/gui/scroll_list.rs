use crate::gui::GuiWidget;

pub struct ScrollList {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub widget: dyn GuiWidget 
}