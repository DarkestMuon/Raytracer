extern crate vector3;
use vector3::Vector3;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face:true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        let dot_product = r.direction().dot(outward_normal);
        self.front_face = dot_product < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.clone() * -1.0
        };
    }
}
