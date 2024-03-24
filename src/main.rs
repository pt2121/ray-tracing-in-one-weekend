mod ray;
mod hittable;
mod hit_record;
mod sphere;

use cgmath::{Point3, Vector3};
use log::info;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::{hit, Sphere};

pub type Color = Vector3<f32>;

fn color(r: i32, g: i32, b: i32) -> Vector3<i32> {
    return Vector3::new(r, g, b);
}

fn write_color(color: Vector3<i32>) {
    println!("{} {} {}", color.x, color.y, color.z);
}

fn ray_color(ray: &Ray, world: &Vec<impl Hittable>) -> Vector3<i32> {
    if let Some(hit_record) = hit(world, ray, 0.0..=f32::MAX) {
        let m = 255.0 * 0.5 * (hit_record.normal + Vector3::new(1.0, 1.0, 1.0));
        return color(m.x as i32, m.y as i32, m.z as i32);
    }
    let unit_direction = ray.unit_dir();
    let a = 0.5 * (unit_direction.y + 1.0);
    let c = 255.0 * ((1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0));
    return color(c.x as i32, c.y as i32, c.z as i32);
}

// RUST_LOG=info cargo run > image.ppm
fn main() {
    env_logger::init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);

    // World
    let mut world_objects = Vec::new();
    world_objects.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world_objects.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0f32, 0.0, 0.0);

    // create the vectors representing the viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    // create delta vectors (pixel vectors)
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    // Find the center of the first pixel
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        info!("remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world_objects);
            write_color(color);
        }
    }
    info!("Done.")
}
