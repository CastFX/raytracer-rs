use std::sync::Arc;

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    box3::Box3,
    bvh::BvhNode,
    constant_medium::ConstantMedium,
    hit::{Hittable, RotateY, Translate, World},
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
            7 => Self::cornell_smoke(),
            8 => Self::final_scene(),
            _ => Self::final_scene(),
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

    fn default_cornell() -> Self {
        Scene {
            aspect_ratio: 1.0,
            image_width: 600,
            image_height: 600,
            background: Color::ZERO,
            lookfrom: Point3::new(278.0, 278.0, -800.0),
            lookat: Point3::new(278.0, 278.0, 0.0),
            vfov: 40.0,
            samples_per_pixel: 200,
            ..Self::default()
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
                                Color::random(0.0..1.0)
                                    * Color::random(0.0..1.0),
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
            let red = Arc::new(Lambertian::from_solid_color(Color::new(
                0.65, 0.05, 0.05,
            )));
            let white = Arc::new(Lambertian::from_solid_color(Color::new(
                0.73, 0.73, 0.73,
            )));
            let green = Arc::new(Lambertian::from_solid_color(Color::new(
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

            let mut box1: Box<dyn Hittable> = Box::new(Box3::new(
                Point3::ZERO,
                Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            ));
            box1 = Box::new(RotateY::new(box1, 15.0));
            box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
            world.push(box1);

            let mut box2: Box<dyn Hittable> = Box::new(Box3::new(
                Point3::ZERO,
                Point3::from_float(165.0),
                white.clone(),
            ));
            box2 = Box::new(RotateY::new(box2, -18.0));
            box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
            world.push(box2);

            world
        };

        Scene {
            world,
            ..Self::default_cornell()
        }
    }

    fn cornell_smoke() -> Self {
        let world: World = {
            let mut world = World::new();
            let red = Arc::new(Lambertian::from_solid_color(Color::new(
                0.65, 0.05, 0.05,
            )));
            let white = Arc::new(Lambertian::from_solid_color(Color::new(
                0.73, 0.73, 0.73,
            )));
            let green = Arc::new(Lambertian::from_solid_color(Color::new(
                0.12, 0.45, 0.15,
            )));
            let light =
                Arc::new(DiffuseLight::from_color(Color::from_float(7.0)));

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
                113.0,
                443.0,
                127.0,
                432.0,
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

            let mut box1: Box<dyn Hittable> = Box::new(Box3::new(
                Point3::ZERO,
                Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            ));
            box1 = Box::new(RotateY::new(box1, 15.0));
            box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
            world.push(Box::new(ConstantMedium::from_color(
                box1,
                0.01,
                Color::ZERO,
            )));

            let mut box2: Box<dyn Hittable> = Box::new(Box3::new(
                Point3::ZERO,
                Point3::from_float(165.0),
                white.clone(),
            ));
            box2 = Box::new(RotateY::new(box2, -18.0));
            box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
            world.push(Box::new(ConstantMedium::from_color(
                box2,
                0.01,
                Color::ONE,
            )));

            world
        };

        Scene {
            world,
            ..Self::default_cornell()
        }
    }

    fn final_scene() -> Self {
        let world: World = {
            let mut world = World::new();

            let ground = Arc::new(Lambertian::from_solid_color(Color::new(
                0.48, 0.83, 0.53,
            )));
            let mut rng = rand::thread_rng();

            let boxes_per_side = 20;
            // let boxes1: World = (0..boxes_per_side * boxes_per_side)
            //     .map(|k| {
            //         let i = (k % boxes_per_side) as f64;
            //         let j = (k / boxes_per_side) as f64;
            //         let w = 100.0;
            //         let x0 = -1000.0 + (i * w);
            //         let y0 = 0.0;
            //         let z0 = -1000.0 + (j * w);
            //         let x1 = x0 + w;
            //         let y1 = rng.gen_range(1.0..101.0);
            //         let z1 = z0 + w;

            //         Box::new(Box3::new(
            //             Point3::new(x0, y0, z0),
            //             Point3::new(x1, y1, z1),
            //             ground.clone(),
            //         )) as Box<dyn Hittable>
            //     })
            //     .collect();
            let mut boxes1 = World::new();
            for i in 0..boxes_per_side {
                for j in 0..boxes_per_side {
                    let w = 100.0;
                    let x0 = -1000.0 + i as f64 * w;
                    let y0 = 0.0;
                    let z0 = -1000.0 + j as f64 * w;
                    let x1 = x0 + w;
                    let y1 = rng.gen_range(1.0..101.0);
                    let z1 = z0 + w;
                    boxes1.push(Box::new(Box3::new(
                        Point3::new(x0, y0, z0),
                        Point3::new(x1, y1, z1),
                        ground.clone(),
                    )));
                }
            }

            // world.extend(boxes1);
            world.push(Box::new(BvhNode::new(boxes1, 0.0, 1.0)));
            // world.push(Box::new(BvhNode::new(boxes1, 0.0, 1.0)));

            let light =
                Arc::new(DiffuseLight::from_color(Color::from_float(7.0)));
            world.push(Box::new(XZRect::new(
                123.0, 423.0, 147.0, 412.0, 554.0, light,
            )));

            let center1 = Point3::new(400.0, 400.0, 200.0);
            let center2 = center1 + Vec3::from_x(30.0);
            let moving_sphere_material = Arc::new(
                Lambertian::from_solid_color(Color::new(0.7, 0.3, 0.1)),
            );
            world.push(Box::new(MovingSphere::new(
                center1,
                center2,
                0.0,
                1.0,
                50.0,
                moving_sphere_material,
            )));

            world.push(Box::new(Sphere::new(
                Point3::new(260.0, 150.0, 45.0),
                50.0,
                Arc::new(Dielectric::new(1.5)),
            )));
            world.push(Box::new(Sphere::new(
                Point3::new(0.0, 150.0, 145.0),
                50.0,
                Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
            )));

            let boundary = Box::new(Sphere::new(
                Point3::new(360.0, 150.0, 145.0),
                70.0,
                Arc::new(Dielectric::new(1.5)),
            ));
            world.push(boundary.clone());

            world.push(Box::new(ConstantMedium::from_color(
                boundary.clone(),
                0.2,
                Color::new(0.2, 0.4, 0.9),
            )));

            let boundary = Box::new(Sphere::new(
                Point3::ZERO,
                5000.0,
                Arc::new(Dielectric::new(1.5)),
            ));
            world.push(Box::new(ConstantMedium::from_color(
                boundary,
                0.0001,
                Color::ONE,
            )));

            let earth_material = Arc::new(Lambertian::new(Arc::new(
                ImageTexture::from_filename(String::from("earthmap.jpg")),
            )));
            world.push(Box::new(Sphere::new(
                Point3::new(400.0, 200.0, 400.0),
                100.0,
                earth_material,
            )));

            let pertext = Arc::new(NoiseTexture::new(0.1));
            world.push(Box::new(Sphere::new(
                Point3::new(220.0, 280.0, 300.0),
                80.0,
                Arc::new(Lambertian::new(pertext)),
            )));

            let white =
                Arc::new(Lambertian::from_solid_color(Color::from_float(0.73)));
            let boxes2: World = (0..1000)
                .map(|_| {
                    Box::new(Sphere::new(
                        Point3::random(0.0..165.0),
                        10.0,
                        white.clone(),
                    )) as Box<dyn Hittable>
                })
                .collect();

            world.push(Box::new(Translate::new(
                Box::new(RotateY::new(
                    Box::new(BvhNode::new(boxes2, 0.0, 1.0)),
                    15.0,
                )),
                Vec3::new(-100.0, 270.0, 395.0),
            )));

            world
        };

        Scene {
            world,
            aspect_ratio: 1.0,
            image_width: 400,
            image_height: 400,
            samples_per_pixel: 10000,
            lookfrom: Point3::new(478.0, 278.0, -600.0),
            lookat: Point3::new(278.0, 278.0, 0.0),
            vfov: 40.0,
            ..Self::default_cornell()
        }
    }
}
