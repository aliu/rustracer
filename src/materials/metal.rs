use crate::materials::{Material, ScatterData};
use crate::objects::HitData;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_data: &HitData) -> Option<ScatterData> {
        let reflected = reflect(ray.direction.normalize(), hit_data.normal);

        if reflected.dot(hit_data.normal) > 0.0 {
            Some(ScatterData {
                attenuation: self.albedo,
                scattered: Ray::new(hit_data.p, reflected),
            })
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}
