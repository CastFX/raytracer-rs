use crate::{ray::Ray, vec3::Point3};

#[derive(Copy, Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = ray.inverse_direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            };

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 > t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(&self, other: &AABB) -> AABB {
        let min = Point3::new(
            self.min.x().min(other.min.x()),
            self.min.y().min(other.min.y()),
            self.min.z().min(other.min.z()),
        );

        let max = Point3::new(
            self.min.x().min(other.min.x()),
            self.min.y().min(other.min.y()),
            self.min.z().min(other.min.z()),
        );

        AABB { min, max }
    }
}
