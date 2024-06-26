extern crate vector3;

use vector3::Vector3;
use std::io::{self, Write};

fn linear_to_gamma(linear_component:f64)  -> f64 {
    if linear_component > 0.0 { return linear_component.sqrt(); }
    return 0.0;
}

fn clamp( x:f64, min:f64,max:f64) -> f64 {
    if x < min {return min};
    if x > max {return max};
    return x;
}
pub fn write_color(out: &mut impl Write, pixel_color: Vector3) -> io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let min =  0.0;
    let max = 1.0;
    let rbyte = ((clamp(r, min, max) * 256.0) as u8).to_be_bytes()[0];
    let gbyte = ((clamp(g, min, max) * 256.0) as u8).to_be_bytes()[0];
    let bbyte = ((clamp(b, min, max) * 256.0) as u8).to_be_bytes()[0];

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}

