use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
}
