extern crate vector3;
use vector3::Vector3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().dot(r.direction());
        let b = -2.0 * r.direction().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / (2.0 * a);
        if root <= t_min || t_max <= root {
            root = (-b + sqrtd) / (2.0 * a);
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal:Vector3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);


        true
    }
}
