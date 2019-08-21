use std::f64;

use rand::Rng;

use crate::camera::Camera;
use crate::config::Config;
use crate::image::Image;
use crate::objects::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let (width, height) = (200, 100);
    let samples = 100;

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
        (0..samples)
            .map(|_| {
                let h = (f64::from(x) + rand()) / f64::from(width);
                let v = (f64::from(y) + rand()) / f64::from(height);
                let ray = camera.ray(h, v);

                color(ray, &scene)
            })
            .fold(Vec3::new(0.0, 0.0, 0.0), |sum, x| sum + x)
            / f64::from(samples)
    });

    image.save(config.file)?;

    Ok(())
}

fn color(ray: Ray, scene: &Scene) -> Vec3 {
    if let Some(hit_data) = scene.hit(ray, 0.0, f64::MAX) {
        return 0.5 * (hit_data.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);

    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

fn rand() -> f64 {
    rand::thread_rng().gen()
}
