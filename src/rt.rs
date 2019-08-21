use crate::config::Config;
use crate::image::Image;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let (width, height) = (200, 100);

    let corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizonal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let image = Image::new(width, height, |x, y| {
        let h = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let ray = Ray::new(origin, corner + (h * horizonal) + (v * vertical));
        color(ray)
    });

    image.save(config.file)?;

    Ok(())
}

fn color(ray: Ray) -> [u8; 3] {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);

    let col = ((1.0 - t) * white) + (t * blue);
    col.to_rgb()
}
