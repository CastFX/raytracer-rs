mod aabb;
mod aarect;
mod bvh;
mod camera;
mod cube;
mod hit;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod scenes;
mod sphere;
mod texture;
mod vec3;

use crate::{hit::World, scenes::Scene};
use camera::Camera;
// use cube::Cube;
use hit::Hittable;

use rand::Rng;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec3::Color;

fn main() {
    let scene = Scene::new(0);

    let camera = Camera::new(&scene, 0.0, 1.0);

    println!(
        "P3\n{:?} {:?}\n255\n",
        scene.image_width, scene.image_height
    );

    for j in (0..scene.image_height).rev() {
        eprintln!("\rScanlines remaining: {:?}", j + 1);

        let scanline: Vec<Color> = (0..scene.image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..scene.samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u)
                        / ((scene.image_width - 1) as f64);
                    let v = ((j as f64) + random_v)
                        / ((scene.image_height - 1) as f64);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(
                        &r,
                        &scene.background,
                        &scene.world,
                        scene.max_depth,
                    );
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(scene.samples_per_pixel));
        }
    }
}

fn ray_color(
    ray: &Ray,
    background: &Color,
    world: &World,
    depth: u64,
) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = rec.mat.color_emitted(rec.u, rec.v, &rec.p);

        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
            emitted
                + attenuation
                    * ray_color(&scattered, background, &world, depth - 1)
        } else {
            emitted
        }
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
        // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        // let r = Ray::new(rec.p, target - rec.p);
        // 0.5 * ray_color(&r, world, depth - 1)
        // 0.5 * (rec.normal + Color::from_float(1.0))
    } else {
        // let unit_direction = ray.direction().normalized();
        // let t = 0.5 * (unit_direction.y() + 1.0);
        // (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
        *background
    }
}

// fn random_scene_cubes() -> World {
//     let mut rng = rand::thread_rng();
//     let mut world = World::new();

//     let ground_mat = Arc::new(Lambertian::new(Color::from_float(0.5)));
//     let ground_sphere = Sphere::new(Point3::from_y(-1000.0), 1000.0, ground_mat);

//     world.push(Box::new(ground_sphere));

//     //diffuse
//     let mut center = Point3::new(0.5, 0.2, 0.5);
//     let diffuse_albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
//     let diffuse_mat = Arc::new(Lambertian::new(diffuse_albedo));
//     let diffuse_cube = Cube::new(center, center + 1.0, diffuse_mat);
//     world.push(Box::new(diffuse_cube));

//     //metal
//     // center += Vec3::new(1.5, 0.0, 0.5);
//     // let metal_albedo = Color::random(0.4..1.0);
//     // let metal_fuzz = rng.gen_range(0.0..0.5);
//     // let metal_mat = Arc::new(Metal::new(metal_albedo, metal_fuzz));
//     // let metal_cube = Cube::new(center, center + 1.0, metal_mat);
//     // world.push(Box::new(metal_cube));

//     // //glass
//     // center += Vec3::new(1.5, 0.0, 0.5);
//     // let glass_mat = Arc::new(Dielectric::new(1.5));
//     // let glass_cube = Cube::new(center, center + 1.0, glass_mat);
//     // world.push(Box::new(glass_cube));

//     // for a in -2..2 {
//     //     for b in -2..2 {
//     //         let choose_mat = rng.gen::<f64>();
//     //         let center = Point3::new(
//     //             (a as f64) + rng.gen_range(0.0..0.9),
//     //             0.2,
//     //             (b as f64) + rng.gen_range(0.0..0.9),
//     //         );

//     //         let sphere = match choose_mat {
//     //             mat if mat < 0.8 => {
//     //                 //diffuse
//     //                 let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
//     //                 let sphere_mat = Arc::new(Lambertian::new(albedo));
//     //                 Cube::new(center, center + 0.5, sphere_mat)
//     //             }
//     //             mat if mat < 0.95 => {
//     //                 //metal
//     //                 let albedo = Color::random(0.4..1.0);
//     //                 let fuzz = rng.gen_range(0.0..0.5);
//     //                 let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
//     //                 Cube::new(center, center + 0.5, sphere_mat)
//     //             }
//     //             _ => {
//     //                 //glass
//     //                 let sphere_mat = Arc::new(Dielectric::new(1.5));
//     //                 Cube::new(center, center + 0.5, sphere_mat)
//     //             }
//     //         };

//     //         world.push(Box::new(sphere));
//     //     }
//     // }
//     let mat1 = Arc::new(Dielectric::new(1.5));
//     let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
//     let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

//     let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
//     let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
//     let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

//     world.push(Box::new(sphere1));
//     world.push(Box::new(sphere2));
//     world.push(Box::new(sphere3));

//     world
// }
