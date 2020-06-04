mod math;
mod trace;

use math::{Float, Vec3};
use std::io::{self, Write};
use trace::write_pixel;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() -> std::io::Result<()> {
    let mut out: Box<dyn Write> = Box::new(io::stdout());

    writeln!(&mut out, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Vec3::with_elements(
                (i as Float) / ((IMAGE_WIDTH - 1) as Float),
                (j as Float) / ((IMAGE_HEIGHT - 1) as Float),
                0.25 as Float,
            );
            write_pixel(&mut out, &pixel_color)?;
        }
    }
    eprintln!("Done!");
    Ok(())
}
