use alloc::vec::Vec;
use crate::gui::GuiWidget;

pub struct GUIRenderer<'a> {
    elements: Vec<&'a GuiWidget>
}

impl GUIRenderer<'_> {
    pub fn new() -> Self where Self: Sized {
        return GUIRenderer {
            elements: Vec::new()
        };
    }

    pub fn draw(&self) {
        for element in &self.elements {
            element.draw();
        }
    }
}