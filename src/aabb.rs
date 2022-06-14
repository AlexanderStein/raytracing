use crate::ray::*;
use cgmath::Point3;

#[derive(Clone, Copy)]
pub struct AABB {
    minimum: Point3<f64>,
    maximum: Point3<f64>,
}

impl AABB {
    pub fn new(minimum: Point3<f64>, maximum: Point3<f64>) -> Self {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Point3<f64> {
        self.minimum
    }

    pub fn max(&self) -> Point3<f64> {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.minimum[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.minimum[a] - ray.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Point3 {
            x: box0.min().x.min(box1.min().x),
            y: box0.min().y.min(box1.min().y),
            z: box0.min().z.min(box1.min().z),
        };
        let big = Point3 {
            x: box0.max().x.max(box1.max().x),
            y: box0.max().y.max(box1.max().y),
            z: box0.max().z.max(box1.max().z),
        };
        AABB {
            minimum: small,
            maximum: big,
        }
    }
}
