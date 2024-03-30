use cgmath::Point3;

use crate::camera::{Camera, color};
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;

mod ray;
mod hittable;
mod hit_record;
mod sphere;
mod camera;
mod material;

// RUST_LOG=info cargo run > image.ppm
fn main() {
    env_logger::init();

    let material_ground = Lambertian::new(color(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(color(0.1, 0.2, 0.5));
    let material_right = Metal::new(color(0.8, 0.6, 0.2), 0.0);
    let world_objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5))),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, Dielectric::new(1.5))),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ];

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400, 10, 50);
    camera.render(&world_objects)
}
