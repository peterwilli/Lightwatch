use crate::common::Rect;
use crate::common::GuiNumber;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::hash::Hash;
use core::ops::{Add, Div, Range};
use crate::println;
use std::prelude::v1::*;
use std::collections::HashMap;

pub struct ObjectHolder<T: GuiNumber, O> {
    rect: Rect<T>,
    object: O
}

pub struct GeoSpatialFastIndex<
    T: GuiNumber,
    G: GuiNumber,
    O,
> {
    pub tile_width: T,
    pub tile_height: T,
    pub grid_width: G,
    pub grid_height: G,
    pub added_objects: Vec<ObjectHolder<T, O>>,
    pub grid: HashMap<(G, G), Vec<usize>>,
}

impl<
        T: GuiNumber + Copy + core::convert::From<T> + std::ops::Div<Output = T> + num::Zero + num::One + std::cmp::PartialOrd + std::ops::AddAssign,
        G: GuiNumber + Hash + core::cmp::Eq + core::convert::From<T>,
        O: core::marker::Copy,
    > GeoSpatialFastIndex<T, G, O>
{
    pub fn new(tile_width: T, tile_height: T, grid_width: G, grid_height: G) -> Self {
        return GeoSpatialFastIndex {
            tile_width: tile_width,
            tile_height: tile_height,
            grid_width: grid_width,
            grid_height: grid_height,
            grid: HashMap::new(),
            added_objects: Vec::new()
        };
    }

    pub fn rect_to_tiles(&self, rect: &Rect<T>) -> Rect<T> {
        return Rect {
            x: rect.x / self.tile_width,
            y: rect.y / self.tile_height,
            w: rect.w / self.tile_width,
            h: rect.h / self.tile_height,
        };
    }

    pub fn find(&mut self, rect: &Rect<T>) -> Vec<O> {
        let rect_tiles = self.rect_to_tiles(rect);
        let mut tile_y = T::zero();
        let mut items: Vec<O> = vec![];
        loop {
            if tile_y > rect_tiles.h {
                break;
            }
            let mut tile_x = T::zero();
            loop {
                if tile_x > rect_tiles.w {
                    break;
                }
                let tuple = (
                    (rect_tiles.x + tile_x).into(),
                    (rect_tiles.y + tile_y).into(),
                );
                let possible_tile = self.grid.get(&tuple);
                if possible_tile.is_some() {
                    for held_object_index in possible_tile.unwrap() {
                        let held_object = self.added_objects.get(*held_object_index).unwrap();
                        if rect.is_inside(&held_object.rect) {
                            items.push(held_object.object);
                        }
                    }
                }
                tile_x += T::one();
            }
            tile_y += T::one();
        }

        return items;
    }

    pub fn add(&mut self, rect: Rect<T>, object: O)
    {
        let rect_tiles = self.rect_to_tiles(&rect);
        let mut tile_y = T::zero();
        let held_object = ObjectHolder::<T, O> {
            rect: rect,
            object: object
        };
        self.added_objects.push(held_object);
        let held_object_index = self.added_objects.len() - 1;

        //... Seriously? How to you do range loops on type generics?!
        loop {
            if tile_y > rect_tiles.h {
                break;
            }
            let mut tile_x = T::zero();
            loop {
                if tile_x > rect_tiles.w {
                    break;
                }
                let tuple = (
                    (rect_tiles.x + tile_x).into(),
                    (rect_tiles.y + tile_y).into(),
                );
                if self.grid.contains_key(&tuple) {
                    self.grid.get_mut(&tuple).unwrap().push(held_object_index);
                } else {
                    self.grid.insert(tuple, vec![held_object_index]);
                }
                tile_x += T::one();
            }
            tile_y += T::one();
        }
    }
}
