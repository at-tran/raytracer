use crate::point::Point;
use crate::ray::Ray;

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
            let invD = 1.0 / r.direction()[i];
            let mut t0 = (self.minimum.0[i] - r.origin().0[i]) * invD;
            let mut t1 = (self.maximum.0[i] - r.origin().0[i]) * invD;
            if invD < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if f64::min(t1, t_max) <= f64::max(t0, t_min) {
                return false;
            }
        }
        true
    }
}
