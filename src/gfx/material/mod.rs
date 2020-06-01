use crate::basic_types::vec3::Color;
use crate::basic_types::ray::Ray;
use crate::gfx::hittable::HitRecord;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
}



pub mod lambertian;
pub mod metal;
pub mod dielectric;