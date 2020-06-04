use crate::math::{Float, Vec3};
use std::io::Write;

fn to_color_byte(c: Float) -> i32 {
    (c * (255.999 as Float)) as i32
}

pub fn write_pixel(out: &mut dyn Write, pixel_color: &Vec3) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {} {}",
        to_color_byte(pixel_color.x()),
        to_color_byte(pixel_color.y()),
        to_color_byte(pixel_color.z())
    )
}
