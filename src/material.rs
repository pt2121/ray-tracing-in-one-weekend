use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use crate::camera::color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

// 1. Produce a scattered ray (or say it absorbed the incident ray).
// 2. If scattered, say how much the ray should be attenuated.
pub trait Material {
    // returns attenuation and scattered ray
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if is_near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

fn is_near_zero(v: Vector3<f32>) -> bool {
    let s = 1e-8;
    return v.x.abs() < s && v.y.abs() < s && v.z.abs() < s;
}

fn random_vec3_range(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
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

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Metal { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(ray.unit_dir(), hit_record.normal);
        let fuzzed = reflected + self.fuzz * random_unit_vector();
        let scattered = Ray::new(hit_record.p, fuzzed);
        if scattered.dir.dot(hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric { index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray.unit_dir();
        let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            // Must Reflect
            reflect(unit_direction, hit_record.normal)
        } else {
            // Can Refract
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.p, direction);
        let attenuation = color(1.0, 1.0, 1.0);
        return Some((scattered, attenuation));
    }
}

fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude2()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}
