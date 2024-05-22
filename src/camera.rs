use std::io;

use rand::Rng;
use vector3::Vector3;

use crate::{color::write_color, hittable::{HitRecord, Hittable}, ray::Ray, vec3::random_unit_vector};
#[derive(Copy,Clone)]
pub struct  Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    image_height: u32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
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
        let samples_per_pixel=10;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let max_depth=10;
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,

        }
    }
    fn ray_color(r:Ray,depth:u32, world: &dyn Hittable) -> Vector3 {
        if depth <= 0 {return Vector3::new(0.0,0.0,0.0);}
        let mut rec:HitRecord=HitRecord::new();
        if world.hit(r, 0.001, 10000000.0, &mut rec) {
            let direction = rec.normal + random_unit_vector();
            return  Self::ray_color(Ray::new_with_origin_and_direction(rec.p, direction),depth-1, world) * 0.5;
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
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                    let  r = self.get_ray(i as u32, j as u32 );
                    pixel_color = pixel_color + Self::ray_color(r,self.max_depth, world);
            }


            // Write the pixel color to stdout
            write_color(&mut io::stdout(),  pixel_color * self.pixel_samples_scale )?;
        }
    }
    Ok(())
    }
    fn get_ray(self,i:u32, j:u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
                          + ( self.pixel_delta_u * ((i as f64 + offset.x)).into())
                          + ( self.pixel_delta_v * ((j as f64 + offset.y)).into() );

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new_with_origin_and_direction(ray_origin, ray_direction);
    }

    fn sample_square() -> Vector3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        let mut rng = rand::thread_rng();
        return Vector3::new(rng.gen_range(-0.5..0.5),rng.gen_range(-0.5..0.5), 0.0);
    }
}