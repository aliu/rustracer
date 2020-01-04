use crate::materials::{Material, ScatterData};
use crate::objects::HitData;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, hit_data: &HitData) -> Option<ScatterData> {
        let target = hit_data.normal + util::unit_sphere_random();

        Some(ScatterData {
            attenuation: self.albedo,
            scattered: Ray::new(hit_data.p, target),
        })
    }
}
