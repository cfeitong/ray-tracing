use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use image::{Rgb, Pixel};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn new<T: Into<f32>>(x: T, y: T, z: T) -> Self {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(self) -> Vec3 {
        self / self.len()
    }

    pub fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn len2(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn proj_to(self, rhs: Self) -> Self {
        rhs.normalize() * self.dot(rhs.normalize())
    }

    pub fn distance(self, rhs: Self) -> f32 {
        let v = self - rhs;
        let Self { x, y, z } = v;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn mid_vec(self, rhs: Self) -> Self {
        (self.normalize() + rhs.normalize()).normalize()
    }
}

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::utils::Vec3::new($x as f32, $y as f32, $z as f32)
    };
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Into<f32>> Mul<T> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let v = rhs.into();
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}

impl<T: Into<f32>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, rhs: T) {
        let v = rhs.into();
        self.x *= v;
        self.y *= v;
        self.z *= v;
    }
}

impl<T: Into<f32>> Div<T> for Vec3 {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let v = rhs.into();
        Vec3 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}

impl<T: Into<f32>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, rhs: T) {
        let v = rhs.into();
        self.x /= v;
        self.y /= v;
        self.z /= v;
    }
}

macro_rules! max {
    ($a:expr) => {$a};
    ($a:expr, $($b:expr)+) => {{
        let t = max!($($b),*);
        if $a > t {
            $a
        } else {
            t
        }
    }}
}

macro_rules! min {
    ($a:expr) => {$a};
    ($a:expr, $($b:expr)+) => {{
        let t = min!($($b),*);
        if $a < t {
            $a
        } else {
            t
        }
    }}
}


pub type Color = Vec3;

pub fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255. * max!(0., min!(1., c.x))) as u8;
    let g = (255. * max!(0., min!(1., c.y))) as u8;
    let b = (255. * max!(0., min!(1., c.z))) as u8;
    *Rgb::from_slice(&[r, g, b])
}
