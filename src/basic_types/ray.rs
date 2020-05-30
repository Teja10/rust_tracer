use crate::basic_types::vec3::Vec3;
use crate::basic_types::vec3::Point3;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

pub trait RayTraits {
    fn new(orig: Point3, 
            dir: Vec3) -> Self;
    
    fn origin(&self) -> Point3;
    fn direction(&self) -> Vec3;

    fn at(&self, t: f64) -> Point3;
}

impl RayTraits for Ray {
    fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir
        }
    }
    
    fn origin(&self) -> Point3 {
        self.orig
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }
    
    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}