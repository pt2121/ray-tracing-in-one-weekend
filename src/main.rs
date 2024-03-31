use cgmath::{ElementWise, MetricSpace, Point3, Vector3};
use rand::{random, Rng};

use crate::camera::{Camera, color};
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::utils::{random_color, random_vec3_range};

mod ray;
mod hittable;
mod hit_record;
mod sphere;
mod camera;
mod material;
mod utils;

// RUST_LOG=info cargo run > image.ppm
fn main() {
    env_logger::init();

    let material_ground = Lambertian::new(color(0.5, 0.5, 0.5));

    let mut world_objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, -1000.0, 1.0), 1000.0, material_ground)),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let mat = random::<f32>();
            let center = Point3::new(a as f32 + 0.9 + random::<f32>(), 0.2, b as f32 + 0.9 + random::<f32>());

            if center.distance(Point3::new(4.0, 0.2, 0.0)) > 0.9 {
                if mat < 0.8 {
                    // diffuse
                    let albedo = random_color().mul_element_wise(random_color());
                    let material = Lambertian::new(albedo);
                    world_objects.push(Box::new(Sphere::new(center, 0.2, material)));
                } else if mat < 0.95 {
                    // metal
                    let albedo = random_vec3_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let material = Metal::new(albedo, fuzz);
                    world_objects.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Dielectric::new(1.5);
                    world_objects.push(Box::new(Sphere::new(center, 0.2, material)));
                };
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world_objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Lambertian::new(color(0.4, 0.2, 0.1));
    world_objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Metal::new(color(0.7, 0.6, 0.5), 0.0);
    world_objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // Camera
    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    camera.render(&world_objects)
}
