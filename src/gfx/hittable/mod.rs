use crate::basic_types::vec3::Point3;
use crate::basic_types::vec3::Vec3;
use crate::basic_types::vec3::Vec3Traits;

use crate::basic_types::ray::Ray;
use crate::basic_types::ray::RayTraits;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    front_face: bool,
    t: f64
}

pub trait HitRecordTraits {
    fn new() -> Self;
    fn p(&self) -> Point3;
    fn normal(&self) -> Vec3;
    fn front(&self) -> bool;
    fn t(&self) -> f64;

    fn set_p(&mut self, p: Point3);
    fn set_t(&mut self, t: f64);
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3);
}

impl HitRecordTraits for HitRecord {
    fn new() -> Self {
        let p = Point3::new((0.0,0.0,0.0));
        let normal = Vec3::new((0.0, 0.0, 0.0));
        HitRecord { p, normal, front_face: false, t: 0.0 }
    }

    fn p(&self) -> Point3 {
        self.p
    }

    fn normal(&self) -> Vec3 {
        self.normal
    }

    fn front(&self) -> bool {
        self.front_face
    }

    fn t(&self) -> f64 {
        self.t
    }

    fn set_p(&mut self, p: Point3) {
        self.p = p
    }

    fn set_t(&mut self, t: f64) {
        self.t = t
    }

    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}


pub mod hittables;


