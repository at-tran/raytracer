use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a (dyn Material + Sync),
}

impl HitRecord<'_> {
    pub fn new<'a>(
        r: &Ray,
        outward_normal: Vec3,
        t: f64,
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
            front_face,
            mat,
        }
    }
}

pub struct HitList<'a> {
    pub objects: Vec<Box<dyn Hit + Sync + 'a>>,
}

impl<'a> HitList<'a> {
    pub fn new() -> HitList<'a> {
        HitList {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: impl Hit + Sync + 'a) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HitList<'_> {
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
