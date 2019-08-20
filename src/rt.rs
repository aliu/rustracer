extern crate image;

use image::{ImageBuffer, Rgb};

use crate::config::Config;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let mut image = ImageBuffer::new(200, 100);
    let (width, height) = image.dimensions();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = Vec3::new(x as f64 / width as f64, y as f64 / height as f64, 0.2);
        *pixel = Rgb(color.to_rgb());
    }

    image.save(config.file).unwrap();

    Ok(())
}
