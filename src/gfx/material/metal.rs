use crate::basic_types::vec3::Color;
use crate::basic_types::vec3::Vec3Traits;

use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;
use crate::gfx::material::Material;

pub struct Metal {
    albedo: Color
}

pub trait MetalFn : Material {
    fn new(albedo: Color) -> Self;
}

impl MetalFn for Metal {
    fn new(albedo: Color) -> Self {
        Metal{ albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let reflected = r_in.direction().unitize().reflect(rec.normal());
        let scattered = Ray::new(rec.p(), reflected);
        let attenuation = self.albedo;
        (attenuation, scattered, scattered.direction().dot(rec.normal()) > 0.0)
    }
}