#[cfg(test)]
mod tests {
    use crate::alloc::string::ToString;
    use crate::common::Rect;
    use crate::elements::gui_element::GuiElement;
    use crate::elements::Button;
    use crate::println;
    use crate::rendering::{GuiCanvas, GuiPixel};
    use alloc::prelude::v1::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

    #[test]
    fn test_gui_button() {
        let mut button = Box::new(Button::new(Rect {
            x: 0,
            y: 0,
            w: 100,
            h: 30,
        }));
        button.set_text("Button".to_string());
        button.on_tap = Some(Box::new(|| {
            println!("Button tap!");
        }));
        let mut gui_canvas = GuiCanvas::<i16, i16>::new(10, 10, 10, 10);
        gui_canvas.add_element(button);
        gui_canvas.tap(10, 10);
    }

    #[test]
    fn test_graphic() {
        let mut button = Box::new(Button::new(Rect {
            x: 0,
            y: 0,
            w: 100,
            h: 30,
        }));
        button.set_text("Button".to_string());
        let mut gui_canvas = GuiCanvas::<i16, i16>::new(10, 10, 10, 10);
        let button_id = gui_canvas.add_element(button);
        gui_canvas.transform_element(
            button_id.into(),
            Rect {
                x: 20,
                y: 20,
                w: 50,
                h: 30,
            },
        );
        let mut img: RgbImage = ImageBuffer::new(512, 512);
        let (width, height) = img.dimensions();
        let mut current_pixel = GuiPixel::new();
        for y in 0..height {
            for x in 0..width {
                gui_canvas.get_pixel(x as i16, y as i16, &mut current_pixel);
                let pixel = image::Rgb([current_pixel.r, current_pixel.g, current_pixel.b]);
                img.put_pixel(x, y, pixel);
            }
        }
        img.save("test_gui_render.png").unwrap();
    }
}
