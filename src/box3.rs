use std::sync::Arc;

use crate::{
    aabb::AABB,
    aarect::{XYRect, XZRect, YZRect},
    hit::{Hittable, World},
    material::Material,
    vec3::Point3,
};

pub struct Box3 {
    min: Point3,
    max: Point3,
    sides: World,
}

impl Box3 {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = World::new();
        sides.push(Box::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material.clone(),
        )));
        sides.push(Box::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material.clone(),
        )));

        sides.push(Box::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material.clone(),
        )));
        sides.push(Box::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material.clone(),
        )));

        sides.push(Box::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material.clone(),
        )));
        sides.push(Box::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material.clone(),
        )));

        Box3 { min, max, sides }
    }
}

impl Hittable for Box3 {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
    ) -> Option<crate::aabb::AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
