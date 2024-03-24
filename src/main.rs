mod ray;
mod hittable;
mod hit_record;
mod sphere;
mod camera;

use cgmath::Point3;
use crate::camera::Camera;
use crate::sphere::Sphere;

// RUST_LOG=info cargo run > image.ppm
fn main() {
    env_logger::init();

    // World
    let mut world_objects = Vec::new();
    world_objects.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world_objects.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400, 10);
    camera.render(&world_objects)
}
