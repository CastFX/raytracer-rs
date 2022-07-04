use std::sync::Arc;

use rand::Rng;

use crate::{
    hit::{HitRecord, Hittable},
    material::{Isotropic, Material},
    texture::Texture,
    vec3::{Color, Vec3},
};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Box<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }

    pub fn from_color(
        boundary: Box<dyn Hittable>,
        density: f64,
        color: Color,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::HitRecord> {
        let enable_debug = false;
        let mut rng = rand::thread_rng();
        let debugging = enable_debug && rng.gen::<f64>() < 0.00001;

        match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY) {
            None => None,
            Some(mut hit1) => {
                match self.boundary.hit(ray, hit1.t + 0.0001, f64::INFINITY) {
                    None => None,
                    Some(mut hit2) => {
                        if debugging {
                            eprintln!(
                                "t_min: {:?}, t_max: {:?}",
                                hit1.t, hit2.t
                            );
                        }

                        if hit1.t < t_min {
                            hit1.t = t_min
                        };

                        if hit2.t > t_max {
                            hit2.t = t_max
                        };

                        if hit1.t >= hit2.t {
                            return None;
                        }

                        if hit1.t < 0.0 {
                            hit1.t = 0.0
                        };

                        let ray_length = ray.direction().length();
                        let distance_inside_boundary =
                            (hit2.t - hit1.t) * ray_length;
                        let hit_distance =
                            self.neg_inv_density * rng.gen::<f64>().ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let mut hit =
                            HitRecord::default(self.phase_function.clone());
                        hit.t = hit1.t + hit_distance / ray_length;
                        hit.p = ray.at(hit.t);

                        if debugging {
                            eprintln!(
                                "hit_distance: {:?}, hit.t: {:?}, hit.p: {:?}",
                                hit_distance, hit.t, hit.p
                            );
                        }

                        hit.normal = Vec3::from_x(1.0);
                        hit.front_face = true;

                        Some(hit)
                    }
                }
            }
        }
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
    ) -> Option<crate::aabb::AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
