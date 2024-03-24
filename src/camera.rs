use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use log::info;
use rand::Rng;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::{hit, Sphere};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    pixel00_loc: Point3<f32>,
    samples_per_pixel: i32,
    max_depth: i32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = Point3::new(0.0f32, 0.0, 0.0);

        // create the vectors representing the viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        // create delta vectors (pixel vectors)
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        // Find the center of the first pixel
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world_objects: &Vec<Sphere>) {
        // Render
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
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
        let pixel_center = self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center.to_vec() + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin.to_vec();
        return Ray::new(ray_origin, ray_direction);
    }

    fn pixel_sample_square(&self) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}

fn ray_color(ray: &Ray, depth: i32, world: &Vec<impl Hittable>) -> Vector3<f32> {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = hit(world, ray, 0.001..=f32::MAX) {
        let direction = random_on_hemisphere(hit_record.normal);
        let m = ray_color(&Ray::new(hit_record.p, direction), depth - 1, world)
            .map(|u| {
                0.5 * u
            });
        return color(m.x, m.y, m.z);
    }
    let unit_direction = ray.unit_dir();
    let a = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
    return color(c.x, c.y, c.z);
}

fn color(r: f32, g: f32, b: f32) -> Vector3<f32> {
    return Vector3::new(r, g, b);
}

fn write_color(color: Vector3<f32>, samples_per_pixel: i32) {
    let c = color.map(|x| { x * 256.0 / samples_per_pixel as f32 });
    let r = c.x.clamp(0.000, 255.0);
    let g = c.y.clamp(0.000, 255.0);
    let b = c.z.clamp(0.000, 255.0);
    println!("{} {} {}", r as i32, g as i32, b as i32);
}

fn random_vec3_range(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}

fn random_on_hemisphere(normal: Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn random_unit_vector() -> Vector3<f32> {
    random_in_unit_sphere().normalize()
}

fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
