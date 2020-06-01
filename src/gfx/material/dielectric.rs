use rand::Rng;
use crate::gfx::material::Material;
use crate::gfx::material::schlick;

use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;

use crate::basic_types::vec3::Color;
use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;
use crate::basic_types::vec3::Vec3Traits;

pub struct Dielectric {
    ref_idx: f64
}

pub trait DielectricFn : Material {
    fn new(ref_idx: f64) -> Self;
}

impl DielectricFn for Dielectric {
    fn new(ref_idx: f64) -> Self {
        Dielectric{ref_idx}
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let attenuation = Color::new((1.0, 1.0, 1.0));
        let etai_over_etat = if rec.front() {1.0 / self.ref_idx} else {self.ref_idx};

        let unit_direction = r_in.direction().unitize();
        let val = -unit_direction.dot(rec.normal());
        let cos_theta = if val < 1.0 {val} else {1.0};
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.p(), reflected);
            return (attenuation, scattered, true);
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let mut rng = rand::thread_rng()    ;
        if rng.gen_range(0.0, 1.0) < reflect_prob {
            let reflected = unit_direction.reflect(rec.normal());
            let scattered = Ray::new(rec.p(), reflected);
            return (attenuation, scattered, true);
        }
        let refracted = unit_direction.refract(rec.normal(), etai_over_etat);
        let scattered = Ray::new(rec.p(), refracted);
        (attenuation, scattered, true)
    }
}