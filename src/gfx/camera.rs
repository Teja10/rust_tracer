use crate::basic_types::vec3::Point3;
use crate::basic_types::vec3::Vec3;
use crate::basic_types::vec3::Vec3Traits;
use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;
use crate::degrees_to_radians;
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3
}

pub trait CameraTraits {
    fn new(lookfrom: Point3, lookat: Point3, 
        vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self;

    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

impl CameraTraits for Camera {
    fn new(lookfrom: Point3, lookat: Point3,
        vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;


        let w = (lookfrom - lookat).unitize();
        let u = vup.cross(w).unitize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera{origin, lower_left_corner, vertical, horizontal}
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v*self.vertical);
    }
}