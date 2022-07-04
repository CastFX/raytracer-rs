use std::sync::Arc;

use crate::{material::Material, vec3::Point3};

pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub center: Point3,
    pub size: f64,
    pub material: Arc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Cube {
        Cube {
            min,
            max,
            material,
            center: (max + min) / 2.0,
            size: (min.x() - max.x()).abs(),
        }
    }

    pub fn bounds(&self, index: bool) -> Point3 {
        if index {
            self.max
        } else {
            self.min
        }
    }
}

// impl Hittable for Cube {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let mut tx_min = (self.bounds(ray.direction_signs()[0]).x() - ray.origin().x())
//             * ray.inverse_direction().x();

//         let mut tx_max = (self.bounds(!ray.direction_signs()[0]).x() - ray.origin().x())
//             * ray.inverse_direction().x();

//         let ty_min = (self.bounds(ray.direction_signs()[1]).y() - ray.origin().y())
//             * ray.inverse_direction().y();

//         let ty_max = (self.bounds(!ray.direction_signs()[1]).y() - ray.origin().y())
//             * ray.inverse_direction().y();

//         if tx_min > ty_max || ty_min > tx_max {
//             return None;
//         }

//         if ty_min > tx_min {
//             tx_min = ty_min
//         }

//         if ty_max > tx_max {
//             tx_max = ty_max
//         }

//         let tz_min = (self.bounds(ray.direction_signs()[2]).z() - ray.origin().z())
//             * ray.inverse_direction().z();

//         let tz_max = (self.bounds(!ray.direction_signs()[2]).z() - ray.origin().z())
//             * ray.inverse_direction().z();

//         if tx_min > tz_max || tz_min > tx_max {
//             return None;
//         }

//         if tz_min > tx_min {
//             tx_min = tz_min
//         }

//         if tz_max > tx_max {
//             tx_max = tz_max
//         }

//         if (tx_min < t_min || tx_min > t_max) && (tx_max < t_min || tx_max > t_max) {
//             return None;
//         }

//         let (p, t, normal) = if tx_min < tx_max {
//             let p = ray.at(tx_min);
//             let normal = (2.0 * p - self.min.cross(p)).normalized();
//             (p, tx_min, normal)
//         } else {
//             let p = ray.at(tx_max);
//             let normal = (2.0 * p - self.max.cross(p)).normalized();
//             (p, tx_max, normal)
//         };

//         let mut rec = HitRecord {
//             t,
//             p,
//             normal,
//             mat: self.material.clone(),
//             front_face: false,
//         };
//         // let outward_normal = (rec.p - self.center) / self.side;
//         rec.set_face_normal(ray, normal);
//         Some(rec)
//     }

//     // fn normal_at(&self, p: Point3) -> Vec3 {
//     //     let localPoint = p - self.center;

//     //     let distance = self.
//     // }
// }
