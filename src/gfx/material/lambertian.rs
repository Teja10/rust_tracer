use crate::gfx::material::Material;

use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;


use crate::basic_types::vec3::Color;
use crate::basic_types::vec3::Vec3Traits;

use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

pub struct Lambertian {
    albedo: Color
}

pub trait LambertianFn : Material {
    fn new(albedo: Color) -> Lambertian;
}


impl LambertianFn for Lambertian {
    fn new(albedo: Color) -> Lambertian {
        Lambertian{albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let scatter_direction = rec.normal() + Color::random_unit_vector();
        let scattered = Ray::new(rec.p(), scatter_direction);
        let attenuation = self.albedo;
        return (attenuation, scattered, true);
    }
}