use crate::basic_types::ray::Ray;
use crate::gfx::hittable::HitRecord;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (Option<HitRecord>, bool);
}

pub mod hittable_list;
pub mod sphere;