use crate::basic_types::vec3::Color;
use crate::basic_types::ray::Ray;
use crate::gfx::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
}



pub mod lambertian;
pub mod metal;