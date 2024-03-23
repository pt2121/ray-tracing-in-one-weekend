use cgmath::{Point3, Vector3};

pub struct HitRecord {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
}
