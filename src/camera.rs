use rand::Rng;

use crate::{
    ray::Ray,
    scenes::Scene,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(scene: &Scene, time0: f64, time1: f64) -> Self {
        //vertical fov in degrees
        let theta = std::f64::consts::PI / 180.0 * scene.vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = viewport_height * scene.aspect_ratio;

        let cw = (scene.lookfrom - scene.lookat).normalized();
        let cu = scene.vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let horizontal = scene.dist_to_focus * viewport_width * cu;
        let vertical = scene.dist_to_focus * viewport_height * cv;
        let lower_left_corner = scene.lookfrom
            - horizontal / 2.0
            - vertical / 2.0
            - scene.dist_to_focus * cw;

        Self {
            origin: scene.lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            cu,
            cv,
            lens_radius: scene.aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let lens = self.lens_radius * Vec3::random_in_unit_disk();
        let blur = self.cu * lens.x() + self.cv * lens.y();

        let mut rng = rand::thread_rng();

        let origin = self.origin + blur;
        let target =
            self.lower_left_corner + s * self.horizontal + t * self.vertical;

        Ray::new(
            self.origin + blur,
            target - origin,
            rng.gen_range(self.time0..=self.time1),
        )
    }
}
