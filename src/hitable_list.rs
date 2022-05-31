use crate::{hitable::{HitRecord,Hittable}, ray::*};
use std::option::Option;

pub struct HitableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object))
    }
}
impl Hittable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(temp_rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }
}
