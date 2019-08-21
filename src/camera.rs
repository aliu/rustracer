use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin,
            corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, h: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.corner + h * self.horizontal + v * self.vertical,
        )
    }
}
