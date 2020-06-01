use std::rc::Rc;

use crate::basic_types::ray::Ray;
use crate::basic_types::vec3::Color;
use crate::basic_types::vec3::Vec3Traits;

use crate::gfx::hittable::hittables::Hittable;
use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;

use crate::gfx::material::metal::Metal;
use crate::gfx::material::metal::MetalFn;
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (Option<HitRecord>, bool) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::new(Rc::new(Metal::new(Color::new((0.0,0.0,0.0)))));
        for object in &self.objects {
            let (temp_rec, flag) = object.hit(r, t_min, closest_so_far);
            if flag {
                hit_anything = true;
                closest_so_far = temp_rec.clone().unwrap().t();
                rec = temp_rec.unwrap();
                
            }
        }

        if hit_anything {
            return (Some(rec), hit_anything)
        }
        else {
            return (None, hit_anything)
        }
    }
}