use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    inverse_direction: Vec3,
    time: f64,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point3::ZERO,
            direction: Vec3::ZERO,
            inverse_direction: Vec3::ZERO,
            time: 0.0,
        }
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            inverse_direction: 1.0 / direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn inverse_direction(&self) -> Vec3 {
        self.inverse_direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
