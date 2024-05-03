extern crate vector3;
mod color;
mod ray;
use std::io;
use vector3::Vector3;
use crate::color::write_color;
use ray::Ray;

fn ray_color(r:Ray) -> Vector3 {
    let unit_direction = *r.direction() /  (r.direction().x * r.direction().x +r.direction().y * r.direction().y +r.direction().z * r.direction().z).sqrt() ;
    let a = 0.5 * (unit_direction.y + 1.0);
    return Vector3::new(1.0, 1.0, 1.0)  * (1.0 - a)  + Vector3::new(0.5, 0.7, 1.0) * a;
}
fn main() -> io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height: u32 = (image_width as f64 / aspect_ratio).ceil() as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vector3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left +  (pixel_delta_u + pixel_delta_v)*0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64 ) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new_with_origin_and_direction(camera_center, ray_direction);
            let pixel_color = ray_color(r);

            // Write the pixel color to stdout
            write_color(&mut io::stdout(), pixel_color)?;
        }
    }
    Ok(())
}
