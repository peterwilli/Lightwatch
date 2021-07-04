extern crate num;

use crate::common::Rect;
use num::PrimInt;
use quadtree_rs::{area::Area, area::AreaBuilder, point::Point, Quadtree};

impl Rect {
    pub fn to_qtree_area<T>(&self) -> Area<T>
    where
        T: Default + PrimInt + From<i16>,
    {
        let area = AreaBuilder::<T>::default()
            .anchor(Point {
                x: self.x.into(),
                y: self.y.into(),
            })
            .dimensions((self.w.into(), self.h.into()))
            .build()
            .unwrap();
        return area;
    }
}
