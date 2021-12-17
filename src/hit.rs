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
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(r: &Ray, outward_normal: Vec3, t: f64) -> HitRecord {
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
            front_face,
        }
    }
}

pub struct HitList<T: Hit> {
    pub objects: Vec<T>,
}

impl<T: Hit> HitList<T> {
    pub fn new() -> HitList<T> {
        HitList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hit> Hit for HitList<T> {
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
}
