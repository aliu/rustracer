use crate::objects::{HitData, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let center = ray.start - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = center.dot(ray.direction);
        let c = center.dot(center) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let d = discriminant.sqrt();
            let roots = [(-b - d) / a, (-b + d) / a];

            for root in roots.iter() {
                if (t_min..t_max).contains(root) {
                    let t = *root;
                    let p = ray.point_at(t);
                    let normal = (p - self.center) / self.radius;
                    return Some(HitData { t, p, normal });
                }
            }
        }

        None
    }
}
