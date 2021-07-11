use crate::common::Rect;
use core::convert::TryInto;
use core::ops::Div;
use no_std_compat::collections::HashMap;

pub struct GeoSpatialFastIndex<
    T: num::PrimInt + Default + Div<T>,
    G: num::PrimInt + Default,
    O: Default,
> {
    pub tile_width: T,
    pub tile_height: T,
    pub grid_width: G,
    pub grid_height: G,
    pub grid: HashMap<(G, G), O>,
}

impl<T: num::PrimInt + Default + Div<T>, G: num::PrimInt + Default, O: Default>
    GeoSpatialFastIndex<T, G, O>
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

    pub fn rect_to_tiles(&self, rect: Rect) -> Rect {
        return Rect {
            x: rect.x.try_into().unwrap() / self.tile_width,
            y: 0,
            w: 0,
            h: 0,
        };
    }

    pub fn add(&mut self, rect: Rect) {
        // Calculate rect in tiles
        // rect_tiles =
    }
}
