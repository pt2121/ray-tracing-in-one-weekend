use cgmath::{ElementWise, EuclideanSpace, InnerSpace, Point3, Vector3};
use log::info;
use rand::Rng;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::hit;
use crate::utils::random_in_unit_disk;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    pixel00_loc: Point3<f32>,
    samples_per_pixel: i32,
    max_depth: i32,
    defocus_angle: f32,
    defocus_disk_u: Vector3<f32>,
    defocus_disk_v: Vector3<f32>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f32, // Vertical view angle (field of view)
        look_from: Point3<f32>, // Point camera is looking from
        look_at: Point3<f32>, // Point camera is looking at
        vup: Vector3<f32>, // Camera-relative "up" direction
        defocus_angle: f32, // Variation angle of rays through each pixel
        focus_dist: f32, // Distance from camera lookfrom point to plane of perfect focus
    ) -> Self {
        let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);
        let center = look_from;
        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // create the vectors representing the viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        // create delta vectors (pixel vectors)
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        // Find the center of the first pixel
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world_objects: &Vec<Box<dyn Hittable>>) {
        // Render
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = color(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world_objects);
                }
                write_color(pixel_color, self.samples_per_pixel);
            }
        }
        info!("Done.")
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center = self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center.to_vec() + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin.to_vec();
        return Ray::new(ray_origin, ray_direction);
    }

    fn defocus_disk_sample(&self) -> Point3<f32> {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }

    fn pixel_sample_square(&self) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}

fn ray_color(ray: &Ray, depth: i32, world: &Vec<Box<dyn Hittable>>) -> Vector3<f32> {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hit(world, ray, 0.001..=f32::MAX) {
        return if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            attenuation.mul_element_wise(ray_color(&scattered, depth - 1, world))
        } else {
            color(0.0, 0.0, 0.0)
        };
    }

    let unit_direction = ray.unit_dir();
    let a = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
    return color(c.x, c.y, c.z);
}

pub fn color(r: f32, g: f32, b: f32) -> Vector3<f32> {
    return Vector3::new(r, g, b);
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    linear_component.sqrt()
}

fn write_color(color: Vector3<f32>, samples_per_pixel: i32) {
    let c = color.map(|x| {
        (linear_to_gamma(x / samples_per_pixel as f32)
            .clamp(0.0, 0.999) * 256.0) as i32
    });
    println!("{} {} {}", c.x, c.y, c.z);
}
