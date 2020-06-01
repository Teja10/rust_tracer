use std::rc::Rc;


use rand::Rng;

use lib::basic_types::vec3::Color;
use lib::basic_types::vec3::Point3;
use lib::basic_types::vec3::Vec3Traits;

use lib::basic_types::ray::Ray;
use lib::basic_types::ray::RayTraits;


use lib::gfx::screen::Screen;
use lib::gfx::screen::ScreenTraits;

use lib::gfx::hittable::HitRecord;
use lib::gfx::hittable::HitRecordTraits;
use lib::gfx::hittable::hittables::Hittable;
use lib::gfx::hittable::hittables::hittable_list::HittableList;
use lib::gfx::hittable::hittables::hittable_list::HittableListTrait;
use lib::gfx::hittable::hittables::sphere::Sphere;
use lib::gfx::hittable::hittables::sphere::SphereData;

use lib::gfx::camera::Camera;
use lib::gfx::camera::CameraTraits;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal() + Color::new((1.0,1.0,1.0)))
    }
    // Normalize ray direction 
    let unit_direction = r.direction().unitize();
    // transform y coordinate to viewport coordinate system then scale to range [0,1]
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new((1.0, 1.0, 1.0)) + 
        t*Color::new((0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    let cam = Camera::new();
    let mut s = Screen::empty_screen(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut world: HittableList = HittableList::new();
    let sp = SphereData::new(Point3::new((0.0, 0.0, -1.0)), 0.5);
    world.add(Rc::new(sp));
    let sp = SphereData::new(Point3::new((0.0, -100.5, -1.0)), 100.0);
    world.add(Rc::new(sp));

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new((0.0,0.0,0.0));
            for s in 0..SAMPLES_PER_PIXEL {
                let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
                let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            }
            
            let r = cam.get_ray(u, v);
            let pixel_color = ray_color(r, &world);
            s.color_pixel(i as usize, j as usize, pixel_color);
        }
    }
    
    s.write_to_ppm("image".to_string());
}
