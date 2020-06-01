use crate::basic_types::vec3::Point3;
use crate::basic_types::vec3::Vec3;
use crate::basic_types::vec3::Vec3Traits;
use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3
}

pub trait CameraTraits {
    fn new() -> Self;

    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

impl CameraTraits for Camera {
    fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new((0.0, 0.0, 0.0));
        let horizontal = Vec3::new((viewport_width, 0.0, 0.0));
        let vertical = Vec3::new((0.0, viewport_height, 0.0));
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new((0.0, 0.0, focal_length));
        Camera{origin, lower_left_corner, vertical, horizontal}
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v*self.vertical);
    }
}