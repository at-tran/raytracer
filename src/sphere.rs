use crate::hit::{Hit, HitRecord};
use crate::point::Point;
use crate::ray::Ray;

struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = f64::sqrt(discriminant);
        let root =
            get_first_in_range([(-half_b - sqrtd) / a, (-half_b + sqrtd) / a], t_min, t_max)?;

        Some(HitRecord::new(
            r,
            (r.at(root) - self.center) / self.radius,
            root,
        ))
    }
}

fn get_first_in_range<T: PartialOrd, const L: usize>(vals: [T; L], min: T, max: T) -> Option<T> {
    for v in vals {
        if min <= v && v <= max {
            return Some(v);
        }
    }
    None
}
