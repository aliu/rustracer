mod dielectric;
mod lambertian;
mod material;
mod metal;

use crate::vec3::Vec3;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::material::{Material, ScatterData};
pub use self::metal::Metal;

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}

fn refract(v: Vec3, normal: Vec3, f: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - f * f * (1.0 - dt * dt);

    if discriminant > 0.0 {
        return Some(f * (uv - dt * normal) - discriminant.sqrt() * normal);
    }

    None
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
