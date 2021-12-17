use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: impl Material + Sync + 'static) -> Sphere {
        Sphere {
            center,
            radius,
            mat: Box::new(mat),
        }
    }

    pub fn random_in_unit_sphere() -> Point {
        loop {
            let mut rng = rand::thread_rng();
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if p.length() <= 1.0 {
                return Point(p);
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().0.unit_vector()
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
            self.mat.as_ref(),
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
