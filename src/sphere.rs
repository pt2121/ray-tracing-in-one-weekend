use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

struct Sphere {
    center: Point3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(
        center: Point3<f32>,
        radius: f32,
    ) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}

fn is_front_face(ray: &Ray, outward_normal: Vector3<f32>) -> bool {
    // the parameter `outward_normal` is assumed to have unit length.
    return ray.dir.dot(outward_normal) < 0.0;
}

pub fn hit(objects: Vec<impl Hittable>, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
    let mut closest = ray_tmax;
    let mut hit_anything: Option<HitRecord> = None;
    for h in objects.iter() {
        if let Some(hit) = h.hit(ray, ray_tmin, closest) {
            closest = hit.t;
            hit_anything = Some(hit);
        }
    }
    hit_anything
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
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
            if root <= ray_tmin || root >= ray_tmax {
                root = (-half_b + sqrt_of_discriminant) / a;
                if root <= ray_tmin || root >= ray_tmax {
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
            })
        };
    }
}
