use std::rc::Rc;

use crate::basic_types::ray::Ray;

use crate::gfx::hittable::hittables::Hittable;
use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

pub trait HittableListTrait : Hittable {
    fn new() -> Self;
    fn clear(&mut self);
    fn add(&mut self, object: Rc<dyn Hittable>);
}

impl HittableListTrait for HittableList {
    fn new() -> Self {
        let objects = Vec::new();
        HittableList { objects }
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t();
            }
        }
        hit_anything
    }
}