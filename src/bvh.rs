use crate::{aabb::Aabb, hitable::*, ray::Ray};
use std::cmp::Ordering;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: Aabb,
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        fn box_compare(
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_box = a.bounding_box(time0, time1);
                let b_box = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_box, b_box) {
                    a.min()[axis].partial_cmp(&b.min()[axis]).unwrap()
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
        }

        fn axis_range(
            objects: &[Box<dyn Hittable>],
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> f64 {
            let (min, max) = objects
                .iter()
                .fold((f64::MIN, f64::MAX), |(bmin, bmax), hit| {
                    if let Some(bbox) = hit.bounding_box(time0, time1) {
                        (bmin.min(bbox.min()[axis]), bmin.max(bbox.max()[axis]))
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f64)> = (0..3)
            .map(|a| (a, axis_range(&objects, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let axis = axis_ranges[0].0;

        objects.sort_by(box_compare(time0, time1, axis));
        let len = objects.len();
        match len {
            0 => panic!("no elements in scene"),
            1 => {
                let leaf = objects.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
            _ => {
                let right = BVH::new(objects.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(objects, time0, time1);
                let bbox = Aabb::surrounding_box(&left.bbox, &right.bbox);

                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
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

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
