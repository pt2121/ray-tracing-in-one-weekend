use std::f32::consts::PI;

use cgmath::Point3;

use crate::camera::{Camera, color};
use crate::hittable::Hittable;
use crate::material::Lambertian;
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

    let r = (PI / 4.0).cos();
    let material_left = Lambertian::new(color(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(color(1.0, 0.0, 0.0));
    let world_objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left)),
        Box::new(Sphere::new(Point3::new(r, 0.0, -1.0), r, material_right)),
    ];

    // Camera
    let camera = Camera::new(
        16.0 / 9.0,
        400,
        10,
        50,
        90.0,
    );
    camera.render(&world_objects)
}
