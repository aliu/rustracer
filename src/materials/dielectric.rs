use crate::materials::{Material, ScatterData};
use crate::objects::HitData;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_data: &HitData) -> Option<ScatterData> {
        let reflected = super::reflect(ray.direction, hit_data.normal);

        let (normal, f, cosine) = if ray.direction.dot(hit_data.normal) > 0.0 {
            (
                -hit_data.normal,
                self.refractive_index,
                self.refractive_index * ray.direction.dot(hit_data.normal) / ray.direction.norm(),
            )
        } else {
            (
                hit_data.normal,
                1.0 / self.refractive_index,
                -ray.direction.dot(hit_data.normal) / ray.direction.norm(),
            )
        };

        let scattered = match super::refract(ray.direction, normal, f) {
            Some(refracted) if util::random() >= super::schlick(cosine, self.refractive_index) => {
                Ray::new(hit_data.p, refracted)
            }
            _ => Ray::new(hit_data.p, reflected),
        };

        Some(ScatterData {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered,
        })
    }
}
