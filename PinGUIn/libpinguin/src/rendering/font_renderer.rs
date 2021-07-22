use fontdue::Font;
use std::prelude::v1::*;
use std::collections::HashMap;
use crate::common::GuiNumber;
use crate::elements::GuiElementPixel;
use crate::println;
use core::fmt::Display;

pub struct FontRenderer<T: GuiNumber> {
    fonts: HashMap<String, Font>,
    phantom: T
}

impl<T: GuiNumber + Display + num::Zero + core::convert::TryInto<usize>> FontRenderer<T> where T::Error: std::fmt::Debug {
    pub fn new() -> Self {
        return FontRenderer {
            fonts: HashMap::new(),
            phantom: T::zero()
        }
    }

    pub fn pixel_for_letter(&mut self, letter: char, font_name: String, font_size: f32, x: T, y: T, output: &mut GuiElementPixel) {
        if !self.fonts.contains_key(&font_name) {
            let font = include_bytes!("../resources/fonts/The Growqins.ttf") as &[u8];
            let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
            self.fonts.insert(font_name.clone(), font);
        }
        let (metrics, bitmap) = self.fonts.get(&font_name).unwrap().rasterize(letter, font_size);
        let idx: usize = (x.try_into().unwrap() % 10) + y.try_into().unwrap() * metrics.width;
        if bitmap.len() > idx {
            let pixel_value = bitmap[idx];
            output.r = 255;
            output.g = 255;
            output.b = 255;
            output.a = pixel_value;
        }
        else {
            output.a = 0;
        }
    }
}