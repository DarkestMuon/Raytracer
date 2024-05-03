use std::io;

use vector3::Vector3;

use crate::{color::write_color, hittable::{HitRecord, Hittable}, ray::Ray};

pub struct  Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    image_height: u32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height: u32 = (image_width as f64 / aspect_ratio).ceil() as u32;
        image_height = if image_height < 1 { 1 } else { image_height };
        // Camera
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let center = Vector3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - Vector3::new(0.0, 0.0, focal_length) - viewport_u/2.0- viewport_v/2.0;
        let pixel00_loc = viewport_upper_left +  (pixel_delta_u + pixel_delta_v)*0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    fn ray_color(r:Ray, world: &dyn Hittable) -> Vector3 {
        let mut rec:HitRecord=HitRecord::new();
        if world.hit(r, 0.0, 10000000.0, &mut rec) {
            return (rec.normal + Vector3::new(1.0,1.0,1.0))* 0.5  ;
        }
        let unit_direction = *r.direction() /  (r.direction().x * r.direction().x +r.direction().y * r.direction().y +r.direction().z * r.direction().z).sqrt() ;
        let a = 0.5 * (unit_direction.y + 1.0);
        return Vector3::new(1.0, 1.0, 1.0)  * (1.0 - a)  + Vector3::new(0.5, 0.7, 1.0) * a;
    }
    pub fn render(&self,  world: &dyn Hittable) -> io::Result<()> {
         // Render
    println!("P3\n{} {}\n255", self.image_width, self.image_height);

    for j in 0..self.image_height {
        for i in 0..self.image_width as u128 {
            let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64 ) + (self.pixel_delta_v * j as f64);
            let ray_direction = pixel_center - self.center;
            let r = Ray::new_with_origin_and_direction(self.center, ray_direction);
            let pixel_color = Self::ray_color(r,world);

            // Write the pixel color to stdout
            write_color(&mut io::stdout(), pixel_color)?;
        }
    }
    Ok(())
    }
}