use crate::materials::{Material, ScatterData};
use crate::objects::HitData;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_data: &HitData) -> Option<ScatterData> {
        let reflected = reflect(ray.direction.normalize(), hit_data.normal);
        let scattered = Ray::new(
            hit_data.p,
            reflected + self.fuzziness * util::unit_sphere_random(),
        );

        if scattered.direction.dot(hit_data.normal) > 0.0 {
            return Some(ScatterData {
                attenuation: self.albedo,
                scattered,
            });
        }

        None
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}
