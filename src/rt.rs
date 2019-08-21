extern crate image;

use image::{ImageBuffer, Rgb};

use crate::config::Config;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let mut image = ImageBuffer::new(200, 100);
    let (width, height) = image.dimensions();

    let corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizonal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let y = height - y - 1; // invert y-axis
        let h = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let ray = Ray::new(origin, corner + (h * horizonal) + (v * vertical));
        *pixel = color(ray);
    }

    image.save(config.file).unwrap();

    Ok(())
}

fn color(ray: Ray) -> Rgb<u8> {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);

    let col = ((1.0 - t) * white) + (t * blue);
    Rgb(col.to_rgb())
}
