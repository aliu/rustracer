use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitData {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Object {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitData>;
}
