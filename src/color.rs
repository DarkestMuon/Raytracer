extern crate vector3;

use vector3::Vector3;
use std::io::{self, Write};

pub fn write_color(out: &mut impl Write, pixel_color: Vector3) -> io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = ((r * 255.999) as u8).to_be_bytes()[0];
    let gbyte = ((g * 255.999) as u8).to_be_bytes()[0];
    let bbyte = ((b * 255.999) as u8).to_be_bytes()[0];

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}

