mod math;
mod trace;

use crate::math::{Float, Point, Ray, Vec3};
use crate::trace::get_ray_color;
use std::io::{self, Write};
use std::time::Instant;
use trace::write_pixel;

const ASPECT_RATIO: Float = 16 as Float / 9 as Float;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as Float / ASPECT_RATIO) as i32;

fn main() -> std::io::Result<()> {
    let timer = Instant::now();
    let mut out: Box<dyn Write> = Box::new(io::stdout());
    writeln!(&mut out, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    let viewport_height = 2 as Float;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1 as Float;
    let origin = Point::new();
    let horizontal = Vec3::with_elements(viewport_width, 0 as Float, 0 as Float);
    let vertical = Vec3::with_elements(0 as Float, viewport_height, 0 as Float);
    let depth = Vec3::with_elements(0 as Float, 0 as Float, focal_length);
    let left_bottom_corner = origin - horizontal / 2 as Float - vertical / 2 as Float - depth;

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as Float / (IMAGE_WIDTH - 1) as Float;
            let v = j as Float / (IMAGE_HEIGHT - 1) as Float;
            let dir = left_bottom_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::with_data(origin, dir);
            let pixel_color = get_ray_color(&ray);
            write_pixel(&mut out, &pixel_color)?;
        }
    }
    eprintln!("Time taken = {}ms", timer.elapsed().as_millis());
    Ok(())
}
