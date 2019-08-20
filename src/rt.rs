extern crate image;

use image::{ImageBuffer, Rgb};

use crate::config::Config;

pub fn render(config: Config) -> Result<(), String> {
    let mut image = ImageBuffer::new(200, 100);
    let (width, height) = image.dimensions();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = x as f32 / width as f32;
        let g = y as f32 / height as f32;
        let b = 0.2;
        *pixel = color(r, g, b);
    }

    image.save(config.file).unwrap();

    Ok(())
}

fn color(r: f32, g: f32, b: f32) -> Rgb<u8> {
    fn transform(n: f32) -> u8 {
        (n * 256.) as u8
    }
    Rgb([transform(r), transform(g), transform(b)])
}
