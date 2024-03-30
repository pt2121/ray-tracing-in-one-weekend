use cgmath::{InnerSpace, Vector3};
use rand::Rng;
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
