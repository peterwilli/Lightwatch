#[cfg(test)]
mod tests {
    #[test]
    fn test_qtree() {
        use crate::alloc::string::ToString;
        use alloc::string::String;
        use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

        // Instantiate a new quadtree which associates String values with u64
        // coordinates.
        let mut qt = Quadtree::<u64, String>::new(/*depth=*/ 4);

        // A depth of four means a square with width (and height) 2^4.
        assert_eq!(qt.width(), 16);

        // Associate the value "foo" with a rectangle of size 2x1, anchored at (0, 0).
        let region_a = AreaBuilder::default()
            .anchor(Point { x: 10, y: 0 })
            .dimensions((2, 1))
            .build()
            .unwrap();
        qt.insert(region_a, "foo".to_string());

        // Query over a region of size 2x2, anchored at (1, 0).
        let region_b = AreaBuilder::default()
            .anchor(Point { x: 11, y: 0 })
            .dimensions((2, 2))
            .build()
            .unwrap();
        let mut query = qt.query(region_b);

        // The query region (region_b) intersects the region "foo" is associated with
        // (region_a), so the query iterator returns "foo" by reference.
        assert_eq!(query.next().unwrap().value_ref(), "foo");
    }
}
