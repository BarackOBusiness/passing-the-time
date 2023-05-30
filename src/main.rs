mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::{io::{stderr, Write}, rc::Rc};

use rand::prelude::*;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Hemispheral, Metal, Dielectric};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image data
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u64 = 1920; // For bonus spheres
    const IMAGE_WIDTH: u64 = 800;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // World
    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.8)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.4)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mat_back = Rc::new(Dielectric::new(1.5));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    let sphere_back = Sphere::new(Point3::new(0.0, -100.0 + f64::sin(std::f64::consts::FRAC_PI_3), -101.0 + f64::cos(std::f64::consts::FRAC_PI_3)), 0.5, mat_back.clone());
    let sphere_back_inner = Sphere::new(Point3::new(0.0, -100.0 + f64::sin(std::f64::consts::FRAC_PI_3), -101.0 + f64::cos(std::f64::consts::FRAC_PI_3)), 0.5, mat_back);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    world.push(Box::new(sphere_back));
    world.push(Box::new(sphere_back_inner));

    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let cam = Camera::new(lookfrom,
                                  lookat,
                                  vup,
                                  20.0,
                                  ASPECT_RATIO,
                                  aperture,
                                  dist_to_focus
                                );

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done.");
}
