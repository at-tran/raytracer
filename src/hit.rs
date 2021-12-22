use std::sync::Arc;
use crate::aabb::AABB;
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB>;
}

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: &'a (dyn Material + Sync),
}

impl HitRecord<'_> {
    pub fn new<'a>(
        r: &Ray,
        outward_normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        mat: &'a (dyn Material + Sync),
    ) -> HitRecord<'a> {
        let p = r.at(t);
        let front_face = r.direction().dot(&outward_normal) < 0.0;
        HitRecord {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            u,
            v,
            front_face,
            mat,
        }
    }
}

pub struct HitList {
    pub objects: Vec<Arc<dyn Hit + Sync + Send>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: impl Hit + Sync + Send + 'static) {
        self.objects.push(Arc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest) {
                closest = rec.t;
                res = Some(rec);
            }
        }

        res
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB> {
        let mut res = None;

        for object in self.objects.iter() {
            let this_box = object.bounding_box(start_time, end_time)?;
            res = match res {
                None => Some(this_box),
                Some(res_box) => Some(AABB::surrounding_box(&res_box, &this_box)),
            }
        }

        res
    }
}
