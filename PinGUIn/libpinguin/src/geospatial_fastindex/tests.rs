#[cfg(test)]
mod tests {
    use crate::common::Rect;
    use crate::geospatial_fastindex::GeoSpatialFastIndex;
    use crate::println;
    
    #[test]
    fn test_add_tile() {
        let mut fastindex = GeoSpatialFastIndex::<u8, u8, u8>::new(10, 10, 10, 10);
        fastindex.add(
            &Rect::<u8> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
            1,
        );
        fastindex.add(
            &Rect::<u8> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
            2,
        );
        for i in 0..2 {
            println!("Tryout {}", i);
            let result = fastindex.find(&Rect::<u8> {
                x: 1,
                y: 5,
                w: 1,
                h: 1,
            });
            assert_eq!(result[0], 1);
            assert_eq!(result[1], 2);
        }
        fastindex.add(
            &Rect::<u8> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
            3,
        );
        fastindex.add(
            &Rect::<u8> {
                x: 2,
                y: 0,
                w: 10,
                h: 10,
            },
            4,
        );
        let result = fastindex.find(&Rect {
            x: 1,
            y: 5,
            w: 1,
            h: 1,
        });
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
    }

    #[test]
    fn test_tile_converter() {
        let mut fastindex = GeoSpatialFastIndex::<u8, u8, u8>::new(10, 10, 10, 10);

        ///
        // Tile test 1
        ///
        let rect = Rect {
            x: 0,
            y: 0,
            w: 10,
            h: 10,
        };
        let tiles = fastindex.rect_to_tiles(&rect);
        println!("rect: {} tiles: {}", rect, tiles);
        assert_eq!(
            tiles,
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1
            }
        );

        ///
        // Tile test 2
        ///
        let rect = Rect {
            x: 5,
            y: 5,
            w: 20,
            h: 10,
        };
        let tiles = fastindex.rect_to_tiles(&rect);
        println!("rect: {} tiles: {}", rect, tiles);
        assert_eq!(
            tiles,
            Rect {
                x: 0,
                y: 0,
                w: 2,
                h: 1
            }
        );
    }
}
