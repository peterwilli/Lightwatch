use crate::common::to_qtree_region;
use crate::elements::*;
use crate::println;
use alloc::prelude::v1::Box;
use alloc::vec::Vec;
use core::convert::TryInto;
use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

pub struct GuiPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct GuiCanvas<T: From<i16> + num::PrimInt + Default> {
    elements: Vec<Box<dyn GuiElement>>,
    quadtree: Quadtree<T, u16>,
}

impl<T: From<i16> + num::PrimInt + Default> GuiCanvas<T> {
    pub fn new() -> Self {
        let mut qt = Quadtree::<T, u16>::new(/*depth=*/ 9);

        return GuiCanvas {
            elements: Vec::new(),
            quadtree: qt,
        };
    }

    pub fn get_pixel(&self, x: T, y: T, output: &mut GuiPixel) {
        let area = AreaBuilder::default()
            .anchor(Point { x: x, y: y })
            .dimensions((1.into(), 1.into()))
            .build()
            .unwrap();
        let mut query = self.quadtree.query(area);
        for idx in query {
            println!("idx {}", idx.value_ref());
        }
    }

    pub fn tap(&mut self, x: T, y: T) {
        let area = AreaBuilder::default()
            .anchor(Point { x: x, y: y })
            .dimensions((1.into(), 1.into()))
            .build()
            .unwrap();
        let mut query = self.quadtree.query(area);
        let button_index = query.next().expect("No element found").value_ref();
        let button_index = (*button_index as usize) - 1;
        let mut button: &mut Button = self.elements[button_index]
            .as_any()
            .downcast_mut::<Button>()
            .expect("Wasn't a label!");
        (button.on_tap.as_ref().unwrap())();
    }

    pub fn add_element(&mut self, element: Box<GuiElement>) {
        let rect = element.get_bounds();
        let area = rect.to_qtree_area::<T>();
        self.elements.push(element);
        self.quadtree
            .insert(area, self.elements.len().try_into().unwrap());
    }
}
