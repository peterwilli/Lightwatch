use crate::common::Rect;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::hash::Hash;
use core::ops::{Add, Div, Range};
use no_std_compat::collections::HashMap;

pub struct GeoSpatialFastIndex<
    T: num::PrimInt + Default + Div<T>,
    G: num::PrimInt + Default + Hash,
    O: Default,
> {
    pub tile_width: T,
    pub tile_height: T,
    pub grid_width: G,
    pub grid_height: G,
    pub grid: HashMap<(G, G), Vec<O>>,
}

impl<
        T: num::PrimInt + std::ops::AddAssign + PartialOrd<T> + Default + Div<T>,
        G: num::PrimInt + Default + Hash + From<T>,
        O: Clone + Default,
    > GeoSpatialFastIndex<T, G, O>
{
    pub fn new(tile_width: T, tile_height: T, grid_width: G, grid_height: G) -> Self {
        return GeoSpatialFastIndex {
            tile_width: tile_width,
            tile_height: tile_height,
            grid_width: grid_width,
            grid_height: grid_height,
            grid: HashMap::new(),
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

    pub fn find(&mut self, rect: Rect<T>) -> Vec<O> {
        let rect_tiles = self.rect_to_tiles(&rect);
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
                items.append(&mut self.grid.get(&tuple).unwrap().clone());
                tile_x += T::one();
            }
            tile_y += T::one();
        }
        return items;
    }

    pub fn add(&mut self, rect: Rect<T>, object: O)
    where
        std::ops::Range<T>: Iterator,
    {
        let rect_tiles = self.rect_to_tiles(&rect);
        let mut tile_y = T::zero();
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
                    self.grid.get_mut(&tuple).unwrap().push(object.clone());
                } else {
                    self.grid.insert(tuple, vec![object.clone()]);
                }
                tile_x += T::one();
            }
            tile_y += T::one();
        }
    }
}
