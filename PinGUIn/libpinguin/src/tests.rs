#[cfg(test)]
mod tests {
    use crate::alloc::string::ToString;
    use crate::common::Rect;
    use crate::elements::gui_element::GuiElement;
    use crate::elements::Button;
    use crate::println;
    use crate::rendering::GuiCanvas;
    use alloc::prelude::v1::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

    #[test]
    fn test_gui_button() {
        let mut button = Box::new(Button::new(Rect {
            x: 0,
            y: 0,
            w: 100,
            h: 30,
        }));
        button.text = Some("Button".to_string());
        button.on_tap = Some(Box::new(|| {
            println!("Button tap!");
        }));
        let mut gui_canvas = GuiCanvas::<i16>::new();
        gui_canvas.add_element(button);
        gui_canvas.tap(0, 0);
    }

    #[test]
    fn test_qtree() {
        // Instantiate a new quadtree which associates String values with u64
        // coordinates.
        let mut qt = Quadtree::<u64, String>::new(/*depth=*/ 10);

        // A depth of four means a square with width (and height) 2^4.
        assert_eq!(qt.width(), 1024);

        // Associate the value "foo" with a rectangle of size 2x1, anchored at (0, 0).
        let region_a = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((100, 10))
            .build()
            .unwrap();
        qt.insert(region_a, "foo".to_string());

        // Query over a region of size 2x2, anchored at (1, 0).
        let region_b = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((1, 1))
            .build()
            .unwrap();
        let mut query = qt.query(region_b);

        // The query region (region_b) intersects the region "foo" is associated with
        // (region_a), so the query iterator returns "foo" by reference.
        assert_eq!(query.next().unwrap().value_ref(), "foo");
    }
}
