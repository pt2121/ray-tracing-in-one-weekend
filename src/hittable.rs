use std::ops::RangeInclusive;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord>;
}
