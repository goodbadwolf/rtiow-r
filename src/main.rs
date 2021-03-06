mod math;
mod trace;

use crate::math::{linspace, random_float, random_in_range, Color, Point, Vec3};
use crate::trace::{
    get_ray_color, Camera, DiaelectriMaterial, HittableCollection, LambertianMaterial,
    MetalMaterial, Sphere, BLACK,
};
use rayon::prelude::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use trace::write_pixel;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = (TILE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 20;

fn main() -> std::io::Result<()> {
    let world = Arc::new(generate_world());
    let camera_locus_radius = 13.34;

    let mut render_stats = vec![];

    let mut total_render_time = 0u128;
    let num_steps = 240;
    for (step_idx, camera_locus_angle) in linspace(0.0, 2.0 * PI, num_steps).into_iter().enumerate()
    {
        let render_timer = Instant::now();
        let look_from = Point::new(
            camera_locus_radius * camera_locus_angle.cos(),
            7.0,
            camera_locus_radius * camera_locus_angle.sin(),
        );
        let look_at = Point::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let distance_to_focus = 10.0;
        let aperture = 0.1;

        let camera = Arc::new(Camera::new(
            &look_from,
            &look_at,
            &vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            distance_to_focus,
        ));
        let world = world.clone();

        let num_tiles = (IMAGE_WIDTH * IMAGE_HEIGHT) / (TILE_WIDTH * TILE_HEIGHT);
        let tiles_per_row = IMAGE_WIDTH / TILE_WIDTH;

        let tile_results = (0..num_tiles)
            .into_par_iter()
            .map(move |tile_idx| {
                let col_start = (tile_idx % tiles_per_row) * TILE_WIDTH;
                let col_end = col_start + TILE_WIDTH;
                let row_start = (tile_idx / tiles_per_row) * TILE_HEIGHT;
                let row_end = row_start + TILE_HEIGHT;

                let mut tile_buffer = vec![vec![BLACK; TILE_WIDTH as usize]; TILE_HEIGHT as usize];

                for j in row_start..row_end {
                    for i in col_start..col_end {
                        let mut pixel_color = BLACK;

                        for _s in 0..SAMPLES_PER_PIXEL {
                            let u = (i as f64 + random_float()) / (IMAGE_WIDTH - 1) as f64;
                            let v = (j as f64 + random_float()) / (IMAGE_HEIGHT - 1) as f64;
                            let ray = camera.get_ray(u, v);
                            pixel_color += get_ray_color(&ray, &world, MAX_DEPTH);
                        }
                        pixel_color /= SAMPLES_PER_PIXEL as f64;
                        let tile_j = j - row_start;
                        let tile_i = i - col_start;
                        tile_buffer[tile_j as usize][tile_i as usize] = pixel_color;
                    }
                }

                (tile_idx, tile_buffer)
            })
            .collect::<Vec<_>>();

        let mut frame_buffer = vec![vec![BLACK; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];
        for (tile_idx, tile_buffer) in tile_results {
            for j in 0..TILE_HEIGHT {
                for i in 0..TILE_WIDTH {
                    let frame_buffer_i = (tile_idx % tiles_per_row) * TILE_WIDTH + i;
                    let frame_buffer_j = (tile_idx / tiles_per_row) * TILE_HEIGHT + j;
                    frame_buffer[frame_buffer_j as usize][frame_buffer_i as usize] =
                        tile_buffer[j as usize][i as usize];
                }
            }
        }

        let render_time = render_timer.elapsed().as_millis();
        render_stats.push((step_idx, render_time));
        total_render_time += render_time;
        let avg_render_time = total_render_time / (step_idx as u128 + 1);
        let frames_left = num_steps - step_idx as u32 - 1;
        let est_time_left = avg_render_time * frames_left as u128;
        let est_time_left_mins = est_time_left / 60000;
        eprintln!(
            "Step {:03} done in {} ms. Est time left = {} mins",
            step_idx + 1,
            render_time,
            est_time_left_mins
        );

        let file_name = format!("output_{:03}.ppm", step_idx);
        let mut output = BufWriter::new(File::create(&Path::new(&file_name))?);
        writeln!(&mut output, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                write_pixel(&mut output, &frame_buffer[j as usize][i as usize])?;
            }
        }
        output.flush()?;
    }

    let mut stats_writer = csv::Writer::from_path(&Path::new("output_stats.csv"))?;
    stats_writer.write_record(&["Step_Idx", "Time_In_Ms"])?;
    for (i, time) in render_stats.into_iter() {
        stats_writer.write_record(&[format!("{}", i), format!("{}", time)])?;
    }
    stats_writer.flush()?;

    Ok(())
}

fn generate_world() -> HittableCollection {
    let mut world = HittableCollection::new();

    let ground_material = Arc::new(LambertianMaterial {
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
                let material = Arc::new(LambertianMaterial { albedo });
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            } else if material_choice < 0.95 {
                // Metal
                let albedo = Color::random_in_range(0.5, 1.0);
                let fuzziness = random_in_range(0.0, 0.5);
                let material = Arc::new(MetalMaterial { albedo, fuzziness });
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            } else {
                // Glass
                let material = Arc::new(DiaelectriMaterial::new(1.5));
                let sphere = Box::new(Sphere::new(&center, 0.2, material));
                world.add(sphere);
            }
        }
    }

    world.add(Box::new(Sphere::new(
        &Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(DiaelectriMaterial::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(LambertianMaterial {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(MetalMaterial {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        }),
    )));

    world
}
