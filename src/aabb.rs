use crate::point::Point;
use crate::ray::Ray;

#[derive(Clone)]
pub struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.minimum.0[i] - r.origin().0[i]) * inv_d;
            let mut t1 = (self.maximum.0[i] - r.origin().0[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if f64::min(t1, t_max) <= f64::max(t0, t_min) {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point::new(
            f64::min(box0.minimum.0[0], box1.minimum.0[0]),
            f64::min(box0.minimum.0[1], box1.minimum.0[1]),
            f64::min(box0.minimum.0[2], box1.minimum.0[2]),
        );

        let big = Point::new(
            f64::max(box0.maximum.0[0], box1.maximum.0[0]),
            f64::max(box0.maximum.0[1], box1.maximum.0[1]),
            f64::max(box0.maximum.0[2], box1.maximum.0[2]),
        );

        AABB::new(small, big)
    }
}
