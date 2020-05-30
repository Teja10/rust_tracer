use lib::basic_types::vec3::Color;
use lib::basic_types::vec3::Point3;
use lib::basic_types::vec3::Vec3Traits;

use lib::basic_types::ray::Ray;
use lib::basic_types::ray::RayTraits;

use lib::gfx::screen::Screen;
use lib::gfx::screen::ScreenTraits;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new((0.0, 0.0, -1.0)), 0.5, r) {
        return Color::new((1.0,0.0,0.0));
    }
    let unit_direction = r.direction().unitize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new((1.0, 1.0, 1.0)) + 
        t*Color::new((0.5, 0.7, 1.0))

}

fn main() {
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: i32 = 384;
    // const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as i32;

    // println!("P3");
    // println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    // println!("255");

    // let viewport_height: f64 = 2.0;
    // let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    // let focal_length = 1.0;

    // let origin = Point3::new((0.0, 0.0, 0.0));
    // let horizontal = Vec3::new((viewport_width, 0.0, 0.0));
    // let vertical = Vec3::new((0.0, viewport_height, 0.0));
    // let lower_left_corner = origin - horizontal / 2.0 - vertical  / 2.0 - Vec3::new((0.0, 0.0, focal_length));


    // for j in (0..IMAGE_WIDTH).rev() {
    //     eprintln!("\rScanlines Remaining: {} ", j);
    //     for i in 0..IMAGE_HEIGHT {
    //         let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
    //         let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    //         let temp = lower_left_corner + u * horizontal + v*vertical - origin;
    //         let r = Ray::new(origin, temp);
    //         let pixel = ray_color(&r);
    //         println!("{}", pixel.write_color());
    //     }
    // }
    // eprintln!("\nDone.");
    let image_height: i32 = 128;
    let image_width: i32 = 256;
    let mut s = Screen::empty_screen(image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            s.update_pixel(i as usize, j as usize, (r, g, b));
        }
    }
    
    s.write_to_ppm("image1".to_string());
}
