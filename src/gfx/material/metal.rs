use crate::basic_types::vec3::Color;
use crate::basic_types::vec3::Vec3Traits;

use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;
use crate::gfx::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

pub trait MetalFn : Material {
    fn new(albedo: Color, fuzz: f64) -> Self;
}

impl MetalFn for Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        Metal{ albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let reflected = r_in.direction().unitize().reflect(rec.normal());
        let scattered = Ray::new(rec.p(), reflected + self.fuzz * Color::random_in_unit_sphere());
        let attenuation = self.albedo;
        (attenuation, scattered, scattered.direction().dot(rec.normal()) > 0.0)
    }
}