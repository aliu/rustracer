use crate::objects::HitData;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterData {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Sync {
    fn scatter(&self, ray: Ray, hit_data: &HitData) -> Option<ScatterData>;
}
