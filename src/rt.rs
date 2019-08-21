use std::f64;

use crate::camera::Camera;
use crate::config::Config;
use crate::image::Image;
use crate::objects::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let (width, height) = (200, 100);

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let mut scene = Scene::new();
    scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add_object(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let image = Image::new(width, height, |x, y| {
        let h = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let ray = camera.ray(h, v);
        color(ray, &scene)
    });

    image.save(config.file)?;

    Ok(())
}

fn color(ray: Ray, scene: &Scene) -> [u8; 3] {
    if let Some(hit_data) = scene.hit(ray, 0.0, f64::MAX) {
        return (0.5 * (hit_data.normal + Vec3::new(1.0, 1.0, 1.0))).to_rgb();
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    let col = ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0));
    col.to_rgb()
}
