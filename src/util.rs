use rand::Rng;

use crate::vec3::Vec3;

pub fn unit_disk_random() -> Vec3 {
    loop {
        let v = (2.0 * Vec3::new(random(), random(), 0.0)) - Vec3::new(1.0, 1.0, 0.0);
        if v.dot(v) < 1.0 {
            return v;
        }
    }
}

pub fn unit_sphere_random() -> Vec3 {
    loop {
        let v = (2.0 * Vec3::new(random(), random(), random())) - Vec3::new(1.0, 1.0, 1.0);
        if v.dot(v) < 1.0 {
            return v;
        }
    }
}

pub fn random() -> f64 {
    rand::thread_rng().gen()
}
