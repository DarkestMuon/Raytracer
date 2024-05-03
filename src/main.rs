extern crate vector3;
mod color;
use std::io;
use vector3::Vector3;
use crate::color::write_color;
fn main() -> io::Result<()> {
    // Image dimensions
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
        // Create a Vector3<f64> object representing the pixel color
        let pixel_color = Vector3::new(
            i as f64/ (image_width - 1) as f64,
            j as f64 / (image_height - 1) as f64,
            0.0,
        );

        // Write the pixel color to stdout
        write_color(&mut io::stdout(), pixel_color)?;
        }
    }
    Ok(())
}
