use std::{f64::consts::PI, sync::Arc};

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    material::Material,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(
        center: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            u: 0.0,
            v: 0.0,
            t,
            p,
            normal,
            mat: self.material.clone(),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        let (u, v) = Self::get_sphere_uv(&outward_normal);
        rec.set_u_v(u, v);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::from_float(self.radius),
            self.center + Vec3::from_float(self.radius),
        ))
    }
}
