use crate::{aabb::AABB, hitable::*, ray::Ray};

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut hitable: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        unimplemented!()
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            match &self.tree {
                BVHNode::Leaf(leaf) => leaf.hit(ray, t_min, t_max),
                BVHNode::Branch { left, right } => {
                    let left = left.hit(ray, t_min, t_max);
                    if let Some(l) = &left {
                        t_max = l.t
                    };
                    let right = right.hit(ray, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
