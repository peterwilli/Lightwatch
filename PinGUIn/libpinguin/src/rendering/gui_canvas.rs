use crate::common::Rect;
use crate::elements::GuiRect;
use crate::elements::*;
use crate::geospatial_fastindex::GeoSpatialFastIndex;
use crate::println;
use alloc::prelude::v1::Box;
use crate::common::GuiNumber;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::hash::Hash;
use core::ops::{AddAssign, Div, Sub};
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

pub struct GuiCanvas<
    T: GuiNumber,
    G: GuiNumber
> {
    elements: Vec<Box<dyn GuiElement<T>>>,
    geospatial_fastindex: GeoSpatialFastIndex<T, G, u16>,
}

impl<
        T: 'static + GuiNumber + Copy + core::convert::From<T> + std::ops::Sub<Output = T> + std::ops::Div<Output = T> + num::Zero + num::One + std::cmp::PartialOrd + std::ops::AddAssign,
        G: GuiNumber + Hash + core::cmp::Eq + core::convert::From<T>,
    > GuiCanvas<T, G>
{
    pub fn new(tile_width: T, tile_height: T, grid_width: G, grid_height: G) -> Self {
        return GuiCanvas {
            elements: Vec::new(),
            geospatial_fastindex: GeoSpatialFastIndex::new(
                tile_width,
                tile_height,
                grid_width,
                grid_height,
            ),
        };
    }

    pub fn get_pixel(&mut self, x: T, y: T, output: &mut GuiPixel) {
        let mut pixel = GuiElementPixel::new();
        let mut has_changed = false;
        let result = self.geospatial_fastindex.find(Rect::<T> {
            x: x,
            y: y,
            w: T::one(),
            h: T::one()
        });
        for idx in result {
            has_changed = true;
            let idx = idx as usize;
            let element = &self.elements[idx];
            let bounds = element.get_bounds();
            let local_x: T = (x - bounds.x).try_into().unwrap();
            let local_y: T = (x - bounds.x).try_into().unwrap();
            element.get_pixel(local_x, local_y, &mut pixel);
            // TODO: take alpha channel into account
            output.r = pixel.r;
            output.g = pixel.g;
            output.b = pixel.b;
        }
        if !has_changed {
            output.reset();
        }
    }

    pub fn tap(&mut self, x: T, y: T) {
        let result = self.geospatial_fastindex.find(Rect::<T> {
            x: x,
            y: y,
            w: T::one(),
            h: T::one()
        });
        let button_index = result[0] as usize;
        let mut button: &mut Button<T> = self.elements[button_index]
            .as_any()
            .downcast_mut::<Button<T>>()
            .expect("Wasn't a label!");
        (button.on_tap.as_ref().unwrap())();
    }

    pub fn add_element(&mut self, element: Box<GuiElement<T>>) {
        self.geospatial_fastindex.add(element.get_bounds(), self.elements.len().try_into().unwrap());
        self.elements.push(element);
    }

    pub fn transform_element(&mut self, element_id: usize, new_rect: GuiRect) {
        // let mut element = self.elements[element_id].as_mut();
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
