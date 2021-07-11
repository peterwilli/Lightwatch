use crate::common::Rect;
use crate::elements::GuiRect;
use crate::elements::*;
use crate::geospatial_fastindex::GeoSpatialFastIndex;
use crate::println;
use alloc::prelude::v1::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::ops::Sub;
use std::fmt;

pub struct GuiPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for GuiPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b:{}", self.r, self.g, self.b)
    }
}

impl GuiPixel {
    pub fn new() -> Self {
        return GuiPixel { r: 0, g: 0, b: 0 };
    }

    pub fn reset(&mut self) {
        self.r = 0;
        self.g = 0;
        self.b = 0;
    }
}

pub struct GuiCanvas<T: From<i16> + Sub<T> + num::PrimInt + Default> {
    elements: Vec<Box<dyn GuiElement>>,
    geospatial_fastindex: GeoSpatialFastIndex<u8, u8, u16>,
}

impl<T: From<i16> + Into<i16> + num::PrimInt + Default> GuiCanvas<T> {
    pub fn new() -> Self {
        return GuiCanvas {
            elements: Vec::new(),
            geospatial_fastindex: GeoSpatialFastIndex::new(32, 32, 8, 8),
        };
    }

    pub fn get_pixel(&mut self, x: T, y: T, output: &mut GuiPixel) {
        // let mut pixel = GuiElementPixel::new();
        // let mut has_changed = false;
        // for idx in query {
        //     has_changed = true;
        //     let idx = *idx.value_ref() as usize;
        //     let element = &self.elements[idx];
        //     let bounds = element.get_bounds();
        //     let local_x: u16 = (x.into() - bounds.x).try_into().unwrap();
        //     let local_y: u16 = (x.into() - bounds.x).try_into().unwrap();
        //     element.get_pixel(local_x, local_y, &mut pixel);
        //     // TODO: take alpha channel into account
        //     output.r = pixel.r;
        //     output.g = pixel.g;
        //     output.b = pixel.b;
        // }
        // if !has_changed {
        //     output.reset();
        // }
    }

    pub fn rect_safe(&self, gui_rect: GuiRect) -> Option<Rect<u8>> {
        if ((gui_rect.x + gui_rect.w) < 0 || (gui_rect.y + gui_rect.h) < 0) {
            // there's no point in adding this
            return None;
        } else {
            let mut width = gui_rect.w;
            let mut height = gui_rect.h;
            let mut x = gui_rect.x;
            let mut y = gui_rect.y;
            if x < 0 {
                width -= x;
                x = 0;
            }
            if y < 0 {
                height -= x;
                y = 0;
            }
            return Rect {
                x: x,
                y: y,
                w: width,
                h: height,
            };
        }
    }

    pub fn tap(&mut self, x: T, y: T) {
        // let area = AreaBuilder::default()
        //     .anchor(Point { x: x, y: y })
        //     .dimensions((1.into(), 1.into()))
        //     .build()
        //     .unwrap();
        // let mut query = self.quadtree.query(area);
        // let button_index = query.next().expect("No element found").value_ref();
        // let button_index = *button_index as usize;
        // let mut button: &mut Button = self.elements[button_index]
        //     .as_any()
        //     .downcast_mut::<Button>()
        //     .expect("Wasn't a label!");
        // (button.on_tap.as_ref().unwrap())();
    }

    pub fn add_element(&mut self, element: Box<GuiElement>) {
        let rect = element.get_bounds();
        self.geospatial_fastindex.add(rect);
        // let area = rect.to_qtree_area::<T>();
        // let handle = self
        //     .quadtree
        //     .insert(area, self.elements.len().try_into().unwrap())
        //     .expect("quadtree insert failed!");
        // println!("handle: {}", handle);
        // self.elements.push(element);
    }

    pub fn transform_element(&mut self, element_id: usize, new_rect: GuiRect) {
        let mut element = self.elements[element_id].as_mut();
        // let area = new_rect.to_qtree_area::<T>();
        // element.transform(new_rect);
        // self.quadtree
        //     .delete_by_handle(element_id.try_into().unwrap());
        // let handle = self
        //     .quadtree
        //     .insert(area, self.elements.len().try_into().unwrap())
        //     .expect("quadtree insert failed!");
        // println!("[transform_element] handle: {}", handle);
    }
}
