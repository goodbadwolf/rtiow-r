mod math;
mod trace;

use crate::math::{random_float, Color, Float, Point};
use crate::trace::{
    get_ray_color, Camera, HittableCollection, LambertianMaterial, MetalMaterial, Sphere,
};
use std::io::{self, Write};
use std::rc::Rc;
use std::time::Instant;
use trace::write_pixel;

const ASPECT_RATIO: Float = 16 as Float / 9 as Float;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as Float / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 20;

fn main() -> std::io::Result<()> {
    let timer = Instant::now();
    let mut out: Box<dyn Write> = Box::new(io::stdout());
    writeln!(&mut out, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    let mut world = HittableCollection::new();
    world.add(Box::new(Sphere::new(
        &Point::with_elements(0 as Float, 0 as Float, -1 as Float),
        0.5 as Float,
        Rc::new(LambertianMaterial {
            albedo: Color::with_elements(0.7 as Float, 0.3 as Float, 0.3 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(0 as Float, -100.5 as Float, -1 as Float),
        100 as Float,
        Rc::new(LambertianMaterial {
            albedo: Color::with_elements(0.8 as Float, 0.8 as Float, 0.0 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(1 as Float, 0 as Float, -1 as Float),
        0.5 as Float,
        Rc::new(MetalMaterial {
            albedo: Color::with_elements(0.8 as Float, 0.6 as Float, 0.2 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(-1 as Float, 0 as Float, -1 as Float),
        0.5 as Float,
        Rc::new(MetalMaterial {
            albedo: Color::with_elements(0.8 as Float, 0.8 as Float, 0.8 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(-0.2 as Float, -0.45 as Float, -0.65 as Float),
        0.05 as Float,
        Rc::new(LambertianMaterial {
            albedo: Color::with_elements(0.8 as Float, 0.1 as Float, 0.1 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(0 as Float, -0.45 as Float, -0.65 as Float),
        0.05 as Float,
        Rc::new(LambertianMaterial {
            albedo: Color::with_elements(0.1 as Float, 0.8 as Float, 0.1 as Float),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::with_elements(0.2 as Float, -0.45 as Float, -0.65 as Float),
        0.05 as Float,
        Rc::new(LambertianMaterial {
            albedo: Color::with_elements(0.1 as Float, 0.1 as Float, 0.8 as Float),
        }),
    )));

    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as Float + random_float()) / (IMAGE_WIDTH - 1) as Float;
                let v = (j as Float + random_float()) / (IMAGE_HEIGHT - 1) as Float;
                let ray = camera.get_ray(u, v);
                pixel_color += get_ray_color(&ray, &world, MAX_DEPTH);
            }
            pixel_color /= SAMPLES_PER_PIXEL as Float;

            write_pixel(&mut out, &pixel_color)?;
        }
    }
    eprintln!("Time taken = {}ms", timer.elapsed().as_millis());
    Ok(())
}
