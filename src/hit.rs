use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    // pub mat: Rc<dyn Scatter>,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default(material: Arc<dyn Material>) -> Self {
        HitRecord {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            mat: material,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal =
            if self.front_face { outward_normal } else { -outward_normal }
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

pub struct Translate {
    hittable: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Box<dyn Hittable>, offset: Vec3) -> Self {
        Translate { hittable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray =
            Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut hit) = self.hittable.hit(&moved_ray, t_min, t_max) {
            hit.p += self.offset;
            hit.set_face_normal(&moved_ray, hit.normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self.hittable.bounding_box(time0, time1) {
            Some(output_box) => Some(AABB::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            )),
            _ => None,
        }
    }
}

pub struct RotateY {
    hittable: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bounding_box = match hittable.bounding_box(0.0, 1.0) {
            Some(bbox) => {
                let mut min = Point3::from_float(f64::INFINITY);
                let mut max = Point3::from_float(f64::NEG_INFINITY);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let i = i as f64;
                            let j = j as f64;
                            let k = k as f64;
                            let x =
                                i * bbox.max().x() + (1.0 - i) * bbox.min().x();
                            let y =
                                j * bbox.max().y() + (1.0 - j) * bbox.min().y();
                            let z =
                                k * bbox.max().z() + (1.0 - k) * bbox.min().z();

                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(new_x, y, new_z);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }

                Some(AABB::new(min, max))
            }
            None => None,
        };

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] =
            self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] =
            self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0]
            - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0]
            + self.cos_theta * ray.direction()[2];

        let rotated_ray = Ray::new(origin, direction, ray.time());

        match self.hittable.hit(&rotated_ray, t_min, t_max) {
            Some(hit) => {
                let HitRecord {
                    mut p, mut normal, ..
                } = hit;

                p[0] = self.cos_theta * hit.p[0] + self.sin_theta * hit.p[2];
                p[2] = -self.sin_theta * hit.p[0] + self.cos_theta * hit.p[2];

                normal[0] = self.cos_theta * hit.normal[0]
                    + self.sin_theta * hit.normal[2];
                normal[2] = -self.sin_theta * hit.normal[0]
                    + self.cos_theta * hit.normal[2];

                let mut hit = HitRecord { p, ..hit };
                hit.set_face_normal(&rotated_ray, normal);
                Some(hit)
            }
            None => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bounding_box
    }
}
