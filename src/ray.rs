use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub start: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(start: Vec3, direction: Vec3) -> Ray {
        Ray { start, direction }
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.start + (t * self.direction)
    }
}
