use crate::aabb::AABB;
use crate::hit::{Hit, HitList, HitRecord};
use crate::ray::Ray;

pub struct BVHNode<'a> {
    left: &'a (dyn Hit + Sync),
    right: &'a (dyn Hit + Sync),
    bounding_box: AABB,
}

impl<'a> BVHNode<'a> {
    pub fn new(list: &HitList, start_time: f64, end_time: f64) -> Self {
        Self::new_impl(&list.objects, 0, list.objects.len(), start_time, end_time)
    }

    fn new_impl(
        src_objects: &Vec<Box<dyn Hit + Sync>>,
        start: usize,
        end: usize,
        start_time: f64,
        end_time: f64,
    ) -> Self {
        todo!()
    }
}

impl Hit for BVHNode<'_> {
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

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
