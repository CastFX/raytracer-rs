use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0))
                * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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
        let mut rec = HitRecord {
            t,
            u: 0.0,
            v: 0.0,
            p,
            normal: Vec3::ZERO,
            mat: self.material.clone(),
            front_face: false,
        };
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        rec.set_u_v(u, v);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let radius_vec = Vec3::from_float(self.radius);
        let box0 = AABB::new(
            self.center(time0) - radius_vec,
            self.center(time0) + radius_vec,
        );

        let box1 = AABB::new(
            self.center(time1) - radius_vec,
            self.center(time1) + radius_vec,
        );
        Some(box0.surrounding_box(&box1))
    }
}
