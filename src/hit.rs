use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::Scatter,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    // pub mat: Rc<dyn Scatter>,
    pub mat: Arc<dyn Scatter>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn set_u_v(&mut self, u: f64, v: f64) -> () {
        self.u = u;
        self.v = v;
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (rec, _) = self.iter().fold((None, t_max), |acc, object| {
            if let Some(rec) = object.hit(ray, t_min, acc.1) {
                let t = rec.t;
                (Some(rec), t)
            } else {
                acc
            }
        });
        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let (_, bounding_box) = self.iter().fold(
            (true, None::<AABB>),
            |(first_box, output_box), hittable| {
                if !first_box && output_box.is_none() {
                    (false, None)
                } else {
                    match (hittable.bounding_box(time0, time1), output_box) {
                        (Some(temp_box), None) => (false, Some(temp_box)),
                        (Some(temp_box), Some(output_box)) => {
                            (false, Some(output_box.surrounding_box(&temp_box)))
                        }
                        (None, _) => (false, None),
                    }
                }
            },
        );

        bounding_box
    }
}
