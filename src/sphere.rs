use std::ops::RangeInclusive;
use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere<M: Material> {
    center: Point3<f32>,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(
        center: Point3<f32>,
        radius: f32,
        material: M,
    ) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn is_front_face(ray: &Ray, outward_normal: Vector3<f32>) -> bool {
    // the parameter `outward_normal` is assumed to have unit length.
    return ray.dir.dot(outward_normal) < 0.0;
}

pub fn hit<'a>(objects: &'a Vec<Box<dyn Hittable>>, ray: &'a Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord<'a>> {
    let mut closest = ray_t.end().clone();
    let mut hit_anything: Option<HitRecord> = None;
    for h in objects.iter() {
        if let Some(hit) = h.hit(&ray, ray_t.start().clone()..=closest.clone()) {
            closest = hit.t;
            hit_anything = Some(hit);
        }
    }
    hit_anything
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord> {
        let oc = ray.origin() - self.center.to_vec();
        let a = ray.dir.magnitude2(); // squared length
        let half_b = oc.dot(ray.dir);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        return if discriminant < 0.0 {
            None
        } else {
            let sqrt_of_discriminant = discriminant.sqrt();
            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrt_of_discriminant) / a;
            if !ray_t.contains(&root) {
                root = (-half_b + sqrt_of_discriminant) / a;
                if !ray_t.contains(&root) {
                    return None;
                }
            }

            let point3 = ray.at(root);
            let outward_normal = (point3 - self.center) / self.radius;
            let normal = if is_front_face(ray, outward_normal) {
                outward_normal
            } else {
                -outward_normal
            };
            Some(HitRecord {
                t: root,
                p: point3,
                normal,
                material: &self.material,
            })
        };
    }
}
