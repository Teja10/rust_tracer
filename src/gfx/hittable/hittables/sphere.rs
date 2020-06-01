use std::rc::Rc;

use crate::basic_types::vec3::Point3;
use crate::basic_types::vec3::Vec3Traits;

use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

use crate::gfx::hittable::HitRecord;
use crate::gfx::hittable::HitRecordTraits;
use crate::gfx::hittable::hittables::Hittable;

use crate::gfx::material::Material;

pub struct SphereData {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>
}

pub trait Sphere : Hittable {
    fn new(cen: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self;
}

impl Sphere for SphereData {
    fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        SphereData { center, radius, mat_ptr }
    }
}

impl Hittable for SphereData {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (Option<HitRecord>, bool) {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b*b - a * c;
        

        if discriminant >= 0.0 {
            let root = discriminant.sqrt();
            let temp = (-b - root) / a;
            let mut rec = HitRecord::new(self.mat_ptr.clone());
            if temp < t_max && temp > t_min {
                rec.set_t(temp.clone());
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.set_p(p);
                rec.set_mat_ptr(self.mat_ptr.clone());
                return (Some(rec), true);
            }

            let temp = (-b + root) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp.clone());
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.set_p(p);
                return (Some(rec), true);
            }
        }
        return (None, false);
    }
}