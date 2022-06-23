use crate::{aabb::AABB, aarect::*, hitable::*, hitable_list::*, material::Material, ray::Ray};
use cgmath::Point3;

pub struct Cuboid {
    min: Point3<f64>,
    max: Point3<f64>,
    sides: HitableList,
}

impl Cuboid {
    pub fn new<M: Material + Clone + 'static>(
        p0: Point3<f64>,
        p1: Point3<f64>,
        material: M,
    ) -> Self {
        let mut sides = HitableList::new();

        sides.push(XYRect {
            material: material.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p1.z,
        });
        sides.push(XYRect {
            material: material.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p0.z,
        });

        sides.push(XZRect {
            material: material.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p1.y,
        });
        sides.push(XZRect {
            material: material.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p0.y,
        });

        sides.push(YZRect {
            material: material.clone(),
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p1.x,
        });
        sides.push(YZRect {
            material,
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p0.x,
        });

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
