use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}
