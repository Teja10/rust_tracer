use lib::basic_types::vec3;
use lib::basic_types::vec3::Vec3Traits;
use lib::basic_types::vec3::ColorTraits;


fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    for j in (0..IMAGE_WIDTH).rev() {
        eprintln!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_HEIGHT {
            let r = i as f64 / ((IMAGE_WIDTH - 1) as f64);
            let g = j as f64 / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let pixel = vec3::Color::new(r, g, b);

            println!("{}", pixel.write_color());
        }
    }
}
