use cgmath::{InnerSpace, Vector3};
use rand::Rng;

pub fn is_near_zero(v: Vector3<f32>) -> bool {
    let s = 1e-8;
    return v.x.abs() < s && v.y.abs() < s && v.z.abs() < s;
}

pub fn random_vec3_range(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}

pub fn random_color() -> Vector3<f32> {
    random_vec3_range(-1.0, 1.0)
}

pub fn random_unit_vector() -> Vector3<f32> {
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

pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
