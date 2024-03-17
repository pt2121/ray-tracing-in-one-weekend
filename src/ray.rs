use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>,
               dir: Vector3<f32>) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.origin + t * self.dir
    }

    pub fn unit_dir(&self) -> Vector3<f32> {
        self.dir.normalize()
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin.to_vec()
    }
}
