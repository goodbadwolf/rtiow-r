use crate::math::{to_unit_vector, Color, Float, Ray, Vec3};
use std::io::Write;

const WHITE: Color = Color::with_elements(1.0 as Float, 1.0 as Float, 1.0 as Float);
const LIGHT_BLUE: Color = Color::with_elements(0.5 as Float, 0.7 as Float, 1.0 as Float);

pub fn write_pixel(out: &mut dyn Write, pixel_color: &Vec3) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {} {}",
        to_color_byte(pixel_color.x()),
        to_color_byte(pixel_color.y()),
        to_color_byte(pixel_color.z())
    )
}

pub fn get_ray_color(ray: &Ray) -> Color {
    let unit_direction = to_unit_vector(&ray.direction);
    let t = (unit_direction.y() + (1.0 as Float)) * (0.5 as Float);

    WHITE * (1.0 as Float - t) + LIGHT_BLUE * t
}

fn to_color_byte(c: Float) -> i32 {
    (c * (255.999 as Float)) as i32
}
