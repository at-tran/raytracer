use crate::aabb::AABB;
use crate::hit::{Hit, HitRecord};
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center_start: Point,
    center_end: Point,
    time_start: f64,
    time_end: f64,
    radius: f64,
    mat: Box<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: impl Material + Sync + Send + 'static) -> Sphere {
        Sphere {
            center_start: center,
            center_end: center,
            time_start: 0.0,
            time_end: 1.0,
            radius,
            mat: Box::new(mat),
        }
    }

    pub fn new_moving(
        center_start: Point,
        center_end: Point,
        time_start: f64,
        time_end: f64,
        radius: f64,
        mat: impl Material + Sync + Send + 'static,
    ) -> Sphere {
        Sphere {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            mat: Box::new(mat),
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center(r.time());
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
            (r.at(root) - self.center(r.time())) / self.radius,
            root,
            self.mat.as_ref(),
        ))
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(start_time) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(start_time) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.center(end_time) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(end_time) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(&box0, &box1))
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
