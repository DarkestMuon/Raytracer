extern crate vector3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;
use std::{io, sync::Arc};
// use hittable::HitRecord;
// use hittable::Hittable;
use vector3::Vector3;
use crate::{hittablelist::HittableList, sphere::Sphere};
// use ray::Ray;
mod camera;



// fn ray_color(r:Ray, world: &dyn Hittable) -> Vector3 {
//     let mut rec:HitRecord=HitRecord::new();
//     if (world.hit(r, 0.0, 10000000.0, &mut rec)) {
//         return (rec.normal + Vector3::new(1.0,1.0,1.0))* 0.5  ;
//     }
//     let unit_direction = *r.direction() /  (r.direction().x * r.direction().x +r.direction().y * r.direction().y +r.direction().z * r.direction().z).sqrt() ;
//     let a = 0.5 * (unit_direction.y + 1.0);
//     return Vector3::new(1.0, 1.0, 1.0)  * (1.0 - a)  + Vector3::new(0.5, 0.7, 1.0) * a;
// }
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
