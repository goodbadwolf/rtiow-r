const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            eprintln!("Scanlines remaining: {}", j);
            let r = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b = 0.25f32;

            let r = (r * 255.999f32) as i32;
            let g = (g * 255.999f32) as i32;
            let b = (b * 255.999f32) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
    eprintln!("Done!");
}
