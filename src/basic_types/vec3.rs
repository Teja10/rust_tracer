use std::ops;
use std::fmt;
use num::clamp;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

pub trait Vec3Traits {
    fn new(tup: (f64, f64, f64)) -> Self;
    fn random() -> Self;
    fn random_from_range(min: f64, max: f64) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_unit_vector() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn length_squared(&self) -> f64;
    fn length(&self) -> f64;
    fn dot(&self, other: Self) -> f64;
    fn cross(&self, other: Self) -> Self;
    fn unitize(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn reflect(&self, n: Vec3) -> Self;
    fn refract(&self, n: Vec3, i_over_t: f64) -> Self;
}
        
impl Vec3Traits for Vec3 {
    fn new(tup: (f64, f64, f64)) -> Vec3 {
        let (x,y,z) = tup;
        Vec3{x, y, z}
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vec3{x: rng.gen_range(0.0, 1.0), 
             y: rng.gen_range(0.0, 1.0), 
             z: rng.gen_range(0.0, 1.0)}
    }

    fn random_from_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3{x: rng.gen_range(min, max),
             y: rng.gen_range(min, max),
             z: rng.gen_range(min, max)}
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_from_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }   
    }

    fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, 2.0*std::f64::consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = 1.0 - z*z;
        Vec3 { x: r * a.cos(),
             y: r * a.sin(),
             z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new((self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x))
    }

    fn unitize(&self) -> Vec3 {
        let v = *self;
        v / v.length()
    }

    fn sqrt(&self) -> Self {
        Vec3{x: self.x.sqrt(), y: self.y.sqrt(), z: self.z.sqrt()}
    }

    fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    fn refract(&self, n: Vec3, i_over_t: f64) -> Self {
        let cos_theta = -self.dot(n);
        let r_out_parallel: Vec3 = (*self + cos_theta * n) * i_over_t;
        let r_out_perp = (1.0 - r_out_parallel.length_squared()).sqrt() * n;
        r_out_parallel + r_out_perp
    }
}
    
// Negation
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, 
                y: -self.y, 
                z: -self.z }
    }
}


// Addition w/ constant
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        *self = Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        *self *= 1f64/_rhs
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}. y: {}, z: {}", self.x, self.y, self.z)
    }
}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new((self.x + other.x, 
            self.y + other.y, 
            self.z + other.z))
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Self::Output {
        Vec3::new((self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z))
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new((self.x * other.x, 
            self.y * other.y, 
            self.z * other.z))
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, num: f64) -> Self::Output {
        Vec3::new((self.x * num, 
            self.y * num, 
            self.z * num))
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new((self * other.x, 
            self * other.y, 
            self * other.z))
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, num: f64) -> Self::Output {
        (1f64 / num) * self
    }
}

pub trait ColorTraits: Vec3Traits {
    fn write_color(&self) -> String;
}

pub trait Point3Traits: Vec3Traits {}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl ColorTraits for Color {
    fn write_color(&self) -> String {
        format!("{} {} {}", 
                (256.0 * clamp(self.x, 0.0, 0.999)) as i32, 
                (256.0 * clamp(self.y, 0.0, 0.999)) as i32, 
                (256.0 * clamp(self.z, 0.0, 0.999)) as i32)
    }
}

impl Point3Traits for Point3{}
