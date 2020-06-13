mod math;
mod trace;

use crate::math::{random_float, random_in_range, Color, Point, Vec3};
use crate::trace::{
    get_ray_color, Camera, DiaelectriMaterial, HittableCollection, LambertianMaterial,
    MetalMaterial, Sphere, BLACK,
};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use trace::write_pixel;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 20;

fn main() -> std::io::Result<()> {
    let mut frame_buffer = vec![vec![BLACK; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];
    let world = generate_world();

    let render_timer = Instant::now();
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
    );

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = BLACK;

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_float()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_float()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += get_ray_color(&ray, &world, MAX_DEPTH);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            frame_buffer[j as usize][i as usize] = pixel_color;
        }
    }

    let io_timer = Instant::now();
    let file_name = format!("output_{}.ppm", 0);
    let mut output = BufWriter::new(File::create(&Path::new(&file_name))?);
    writeln!(&mut output, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            write_pixel(&mut output, &frame_buffer[j as usize][i as usize])?;
        }
    }
    output.flush()?;

    eprintln!(
        "Render time taken = {}ms\nFile IO time taken = {}ms",
        render_timer.elapsed().as_millis(),
        io_timer.elapsed().as_millis(),
    );

    Ok(())
}

fn generate_world() -> HittableCollection {
    let mut world = HittableCollection::new();

    let ground_material = Rc::new(LambertianMaterial {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        &Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random_float();
            let center = Point::new(
                a as f64 + 0.9 * random_float(),
                0.19,
                b as f64 + 0.9 * random_float(),
            );

            let is_visible = (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9;
            if !is_visible {
                continue;
            }

            if material_choice < 0.8 {
                // Diffuse
                let albedo = Color::random() * Color::random();
                let material = Rc::new(LambertianMaterial { albedo });
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            } else if material_choice < 0.95 {
                // Metal
                let albedo = Color::random_in_range(0.5, 1.0);
                let fuzziness = random_in_range(0.0, 0.5);
                let material = Rc::new(MetalMaterial { albedo, fuzziness });
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            } else {
                // Glass
                let material = Rc::new(DiaelectriMaterial::new(1.5));
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            }
        }
    }

    world.add(Box::new(Sphere::new(
        &Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(DiaelectriMaterial::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(LambertianMaterial {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(MetalMaterial {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        }),
    )));

    world
}
