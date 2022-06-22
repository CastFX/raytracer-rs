mod camera;
mod color;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;
use std::{
    io::{stderr, Write},
    rc::Rc,
    sync::Arc,
};

use crate::{
    color::write_color,
    hit::World,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};
use camera::Camera;
use hit::Hittable;
use rand::Rng;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec3::Color;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    //world
    // World
    let world = random_scene();
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{:?} {:?}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {:?}", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
}

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
            attenuation * ray_color(&scattered, &world, depth - 1)
        } else {
            Color::ZERO
        }
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
        // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        // let r = Ray::new(rec.p, target - rec.p);
        // 0.5 * ray_color(&r, world, depth - 1)
        // 0.5 * (rec.normal + Color::from_float(1.0))
    } else {
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::from_float(0.5)));
    let ground_sphere = Sphere::new(Point3::from_y(-1000.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            let sphere = match choose_mat {
                mat if mat < 0.8 => {
                    //diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let sphere_mat = Arc::new(Lambertian::new(albedo));
                    Sphere::new(center, 0.2, sphere_mat)
                }
                mat if mat < 0.95 => {
                    //metal
                    let albedo = Color::random(0.4..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    Sphere::new(center, 0.2, sphere_mat)
                }
                _ => {
                    //glass
                    let sphere_mat = Arc::new(Dielectric::new(1.5));
                    Sphere::new(center, 0.2, sphere_mat)
                }
            };

            world.push(Box::new(sphere));
        }
    }
    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}
