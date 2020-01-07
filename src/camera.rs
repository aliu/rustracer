use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect: f64) -> Camera {
        let y = (fov / 2.0).tan();
        let x = aspect * y;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            corner: look_from - x * u - y * v - w,
            horizontal: 2.0 * x * u,
            vertical: 2.0 * y * v,
        }
    }

    pub fn ray(&self, h: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.corner + h * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
