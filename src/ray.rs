extern crate vector3;
use vector3::Vector3;

#[derive(Copy,Clone)]
pub struct Ray {
    pub orig: Vector3, // Assuming Vector3 is a struct with x, y, z fields of type f64
    pub dir: Vector3,    // Assuming Vector3 is a struct with x, y, z fields of type f64
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            orig: Vector3::zero(), // Assuming Vector3 has a default constructor
            dir: Vector3::zero(),    // Assuming Vector3 has a default constructor
        }
    }

    pub fn new_with_origin_and_direction(orig: Vector3, dir: Vector3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &Vector3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vector3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.orig +  self.dir  * t
    }
}

