use crate::aabb::AABB;
use crate::hit::{Hit, HitList, HitRecord};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hit + Sync + Send>,
    right: Arc<dyn Hit + Sync + Send>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(list: &HitList, start_time: f64, end_time: f64) -> Self {
        Self::new_impl(&list.objects, 0, list.objects.len(), start_time, end_time)
    }

    fn new_impl(
        src_objects: &Vec<Arc<dyn Hit + Sync + Send>>,
        start: usize,
        end: usize,
        start_time: f64,
        end_time: f64,
    ) -> Self {
        let mut objects = src_objects.clone();
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..=2);
        let comparator =
            |a: &Arc<dyn Hit + Sync + Send>, b: &Arc<dyn Hit + Sync + Send>| -> Ordering {
                let box_a = a
                    .bounding_box(0.0, 0.0)
                    .expect("No bounding box in BVHNode constructor.");
                let box_b = b
                    .bounding_box(0.0, 0.0)
                    .expect("No bounding box in BVHNode constructor.");

                box_a.minimum.0[axis]
                    .partial_cmp(&box_b.minimum.0[axis])
                    .unwrap()
            };

        let object_span = end - start;
        let left;
        let right;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                }
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                left = Arc::new(BVHNode::new_impl(
                    &objects, start, mid, start_time, end_time,
                ));
                right = Arc::new(BVHNode::new_impl(&objects, mid, end, start_time, end_time));
            }
        }

        let box_left = left
            .bounding_box(start_time, end_time)
            .expect("No bounding box in BVHNode constructor.");
        let box_right = right
            .bounding_box(start_time, end_time)
            .expect("No bounding box in BVHNode constructor.");

        BVHNode {
            left,
            right,
            bounding_box: AABB::surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hit for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            if let Some(rec_left) = &hit_left {
                rec_left.t
            } else {
                t_max
            },
        );

        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
