extern crate vector3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;
use std::{io, sync::Arc};
use vector3::Vector3;
use crate::{hittablelist::HittableList, sphere::Sphere};
mod camera;
mod vec3;


fn main() -> io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;


    let mut world = HittableList::new();

    // Sphere
    let sphere1 = Arc::new(Sphere { center: Vector3::new(0.0, 0.0, -1.0), radius: 0.5 });
    let sphere2 = Arc::new(Sphere { center: Vector3::new(0.0, -100.5, -1.0), radius: 100.0 });


    // Add the Sphere to the HittableList
    world.add(sphere1);
    world.add(sphere2);
    let cam = camera::Camera::new(aspect_ratio, image_width as f64); 
    let _ = cam.render(&world);
    Ok(())
}
