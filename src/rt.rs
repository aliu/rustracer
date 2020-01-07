use std::f64;

use crate::camera::Camera;
use crate::config::Config;
use crate::image::Image;
use crate::materials::*;
use crate::objects::*;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

pub fn render(config: Config) -> Result<(), String> {
    let (width, height) = (200, 100);
    let samples = 100;

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        f64::consts::PI / 2.0,
        width as f64 / height as f64,
    );

    let mut scene = Scene::new();
    scene.add_object(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.5),
    ));
    scene.add_object(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Dielectric::new(1.5),
    ));
    scene.add_object(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Vec3::new(0.1, 0.2, 0.5)),
    ));
    scene.add_object(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3),
    ));
    scene.add_object(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
    ));

    let image = Image::new(width, height, |x, y| {
        let avg = (0..samples)
            .map(|_| {
                let h = (x as f64 + util::random()) / width as f64;
                let v = (y as f64 + util::random()) / height as f64;
                let ray = camera.ray(h, v);

                color(ray, &scene, 0)
            })
            .fold(Vec3::new(0.0, 0.0, 0.0), |sum, x| sum + x)
            / samples as f64;

        // gamma correction
        avg.map(|x| x.sqrt())
    });

    image.save(config.file)?;

    Ok(())
}

fn color(ray: Ray, scene: &Scene, depth: u32) -> Vec3 {
    if let Some(hit_data) = scene.hit(ray, 0.001, f64::MAX) {
        if depth < 50 {
            if let Some(scatter_data) = hit_data.material.scatter(ray, &hit_data) {
                return scatter_data.attenuation * color(scatter_data.scattered, scene, depth + 1);
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);

    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}
