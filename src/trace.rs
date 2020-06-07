use crate::math::{dot_product, to_unit_vector, Color, Float, Point, Ray, Vec3};
use std::cmp::Ordering;
use std::io::Write;

const WHITE: Color = Color::with_elements(1.0 as Float, 1.0 as Float, 1.0 as Float);
const LIGHT_BLUE: Color = Color::with_elements(0.5 as Float, 0.7 as Float, 1.0 as Float);
const RED: Color = Color::with_elements(1 as Float, 0 as Float, 0 as Float);

const SPHERE_CENTER: Vec3 = Point::with_elements(0 as Float, 0 as Float, -1 as Float);

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
    if let Some(hit_normal) = hit_sphere(&SPHERE_CENTER, 0.5 as Float, &ray) {
        return Color::with_elements(
            hit_normal.x() + 1 as Float,
            hit_normal.y() + 1 as Float,
            hit_normal.z() + 1 as Float,
        ) * 0.5 as Float;
    }
    let unit_direction = to_unit_vector(&ray.direction);
    let t = (unit_direction.y() + (1.0 as Float)) * (0.5 as Float);

    WHITE * (1.0 as Float - t) + LIGHT_BLUE * t
}

pub fn hit_sphere(center: &Point, radius: Float, ray: &Ray) -> Option<Vec3> {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = dot_product(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    match discriminant.partial_cmp(&(0 as Float)).unwrap() {
        Ordering::Less => None,
        _ => {
            let t = (-half_b - discriminant.sqrt()) / a;
            let normal = ray.at(t) - *center;
            Some(to_unit_vector(&normal))
        }
    }
}

fn to_color_byte(c: Float) -> u8 {
    (c * (255.999 as Float)) as u8
}
