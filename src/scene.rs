use crate::object::{HitData, Object};
use crate::ray::Ray;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add_object<T: 'static + Object>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Object for Scene {
    fn hit(&self, ray: Ray, t_min: f64, mut t_max: f64) -> Option<HitData> {
        let mut res = None;

        for object in self.objects.iter() {
            if let Some(hit_data) = object.hit(ray, t_min, t_max) {
                t_max = hit_data.t;
                res = Some(hit_data);
            }
        }

        res
    }
}
