use std::sync::Arc;

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    hit::World,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    vec3::{Color, Point3, Vec3},
};
use rand::Rng;

pub struct Scene {
    pub world: World,
    pub background: Vec3,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vfov: f64,
    pub vup: Vec3,
    pub aperture: f64,
    pub samples_per_pixel: u64,
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub image_height: u64,
    pub max_depth: u64,
    pub dist_to_focus: f64,
}

impl Scene {
    pub fn new(n: usize) -> Self {
        match n {
            1 => Self::random_scene(),
            2 => Self::two_spheres(),
            3 => Self::two_perlin_spheres(),
            4 => Self::earth(),
            5 => Self::simple_light(),
            6 => Self::cornell_box(),
            _ => Self::cornell_box(),
        }
    }

    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width: u64 = 400;
        let image_height = ((image_width as f64) / aspect_ratio) as u64;

        Scene {
            world: World::new(),
            background: Color::new(0.70, 0.80, 1.00),
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::ZERO,
            vfov: 20.0,
            vup: Vec3::from_y(1.0),
            aperture: 0.1,
            samples_per_pixel: 100,
            aspect_ratio,
            image_width,
            image_height,
            max_depth: 50,
            dist_to_focus: 10.0,
        }
    }

    fn random_scene() -> Self {
        let world: World = {
            let mut rng = rand::thread_rng();
            let mut world = World::new();

            let checker = Arc::new(CheckerTexture::from_colors(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            ));

            let ground_mat = Arc::new(Lambertian::new(checker));
            let ground_sphere =
                Sphere::new(Point3::from_y(-1000.0), 1000.0, ground_mat);

            world.push(Box::new(ground_sphere));

            for a in -11..11 {
                for b in -11..11 {
                    let choose_mat = rng.gen::<f64>();
                    let center = Point3::new(
                        (a as f64) + rng.gen_range(0.0..0.9),
                        0.2,
                        (b as f64) + rng.gen_range(0.0..0.9),
                    );

                    match choose_mat {
                        mat if mat < 0.8 => {
                            //diffuse
                            let sphere_mat = Lambertian::from_solid_color(
                                &(Color::random(0.0..1.0)
                                    * Color::random(0.0..1.0)),
                            );

                            let center2 =
                                center + Vec3::from_y(rng.gen_range(0.0..0.5));
                            let moving_sphere = MovingSphere::new(
                                center,
                                center2,
                                0.0,
                                1.0,
                                0.2,
                                Arc::new(sphere_mat),
                            );

                            world.push(Box::new(moving_sphere));
                        }
                        mat if mat < 0.95 => {
                            //metal
                            let albedo = Color::random(0.4..1.0);
                            let fuzz = rng.gen_range(0.0..0.5);
                            let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                            let sphere = Sphere::new(center, 0.2, sphere_mat);
                            world.push(Box::new(sphere));
                        }
                        _ => {
                            //glass
                            let sphere_mat = Arc::new(Dielectric::new(1.5));
                            let sphere = Sphere::new(center, 0.2, sphere_mat);
                            world.push(Box::new(sphere));
                        }
                    };
                }
            }
            let mat1 = Arc::new(Dielectric::new(1.5));
            let mat2 = Arc::new(Lambertian::new(Arc::new(SolidColor::new(
                Color::new(0.4, 0.2, 0.1),
            ))));
            let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

            let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
            let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
            let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

            world.push(Box::new(sphere1));
            world.push(Box::new(sphere2));
            world.push(Box::new(sphere3));

            world
        };

        Scene {
            world,
            ..Self::default()
        }
    }

    fn two_spheres() -> Self {
        let world: World = {
            let mut objects = World::new();
            let checker = CheckerTexture::from_colors(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            );
            let checker2 = CheckerTexture::from_colors(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            );

            objects.push(Box::new(Sphere::new(
                Point3::new(0.0, -10.0, 0.0),
                10.0,
                Arc::new(Lambertian::new(Arc::new(checker))),
            )));
            objects.push(Box::new(Sphere::new(
                Point3::new(0.0, 10.0, 0.0),
                10.0,
                Arc::new(Lambertian::new(Arc::new(checker2))),
            )));

            objects
        };

        Scene {
            world,
            ..Self::default()
        }
    }

    fn two_perlin_spheres() -> Scene {
        let world: World = {
            let mut objects = World::new();

            let pertext = Arc::new(NoiseTexture::new(4.0));

            objects.push(Box::new(Sphere::new(
                Point3::new(0.0, -1000.0, 0.0),
                1000.0,
                Arc::new(Lambertian::new(pertext.clone())),
            )));
            objects.push(Box::new(Sphere::new(
                Point3::new(0.0, 2.0, 0.0),
                2.0,
                Arc::new(Lambertian::new(pertext.clone())),
            )));

            objects
        };

        Scene {
            world,
            ..Self::default()
        }
    }

    fn earth() -> Scene {
        let world: World = {
            let earth_texture = Arc::new(ImageTexture::from_filename(
                String::from("earthmap.jpg"),
            ));

            let earth_surface = Arc::new(Lambertian::new(earth_texture));
            let globe = Box::new(Sphere::new(Point3::ZERO, 2.0, earth_surface));

            vec![globe]
        };

        Scene {
            world,
            ..Self::default()
        }
    }

    fn simple_light() -> Self {
        let world: World = {
            let mut world = World::new();
            let pertext = Arc::new(NoiseTexture::new(4.0));
            world.push(Box::new(Sphere::new(
                Point3::from_y(-1000.0),
                1000.0,
                Arc::new(Lambertian::new(pertext.clone())),
            )));
            world.push(Box::new(Sphere::new(
                Point3::from_y(2.0),
                2.0,
                Arc::new(Lambertian::new(pertext.clone())),
            )));

            let diffuse_light =
                Arc::new(DiffuseLight::from_color(Color::from_float(4.0)));
            world.push(Box::new(XYRect::new(
                3.0,
                5.0,
                1.0,
                3.0,
                -2.0,
                diffuse_light,
            )));

            world
        };

        Scene {
            world,
            samples_per_pixel: 400,
            background: Color::ZERO,
            lookfrom: Point3::new(26.0, 3.0, 26.0),
            lookat: Point3::from_y(2.0),
            vfov: 20.0,
            ..Self::default()
        }
    }

    fn cornell_box() -> Self {
        let world: World = {
            let mut world = World::new();
            let red = Arc::new(Lambertian::from_solid_color(&Color::new(
                0.65, 0.05, 0.05,
            )));
            let white = Arc::new(Lambertian::from_solid_color(&Color::new(
                0.73, 0.73, 0.73,
            )));
            let green = Arc::new(Lambertian::from_solid_color(&Color::new(
                0.12, 0.45, 0.15,
            )));
            let light =
                Arc::new(DiffuseLight::from_color(Color::from_float(15.0)));

            world.push(Box::new(YZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                555.0,
                green.clone(),
            )));
            world.push(Box::new(YZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                0.0,
                red.clone(),
            )));
            world.push(Box::new(XZRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                light.clone(),
            )));
            world.push(Box::new(XZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                0.0,
                white.clone(),
            )));
            world.push(Box::new(XZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                555.0,
                white.clone(),
            )));
            world.push(Box::new(XYRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                555.0,
                white.clone(),
            )));

            world
        };

        Scene {
            world,
            aspect_ratio: 1.0,
            image_width: 600,
            image_height: 600,
            background: Color::ZERO,
            lookfrom: Point3::new(278.0, 278.0, -800.0),
            lookat: Point3::new(278.0, 278.0, 0.0),
            vfov: 40.0,
            ..Self::default()
        }
    }
}
