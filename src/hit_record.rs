use cgmath::{Point3, Vector3};

use crate::material::Material;

pub struct HitRecord<'a> {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: &'a dyn Material,
    pub front_face: bool,
}
