use std::{
    f64, fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use rand::Rng;

pub(crate) const EPS: f64 = 1e-3;
pub(crate) const PI: f64 = f64::consts::PI;

// TODO: replace with tuple
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
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

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn len(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn len2(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn proj_to(self, rhs: Self) -> Self {
        let n = rhs.unit();
        n * self.dot(n)
    }

    pub fn distance(self, rhs: Self) -> f64 {
        let v = self - rhs;
        let Self { x, y, z } = v;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn mid_vec(self, rhs: Self) -> Self {
        (self.unit() + rhs.unit()).unit()
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        abs_diff_eq!(self.dot(rhs).abs(), 1.)
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::util::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
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

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x + self,
            y: rhs.y + self,
            z: rhs.z + self,
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

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Sub<Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
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

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
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

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl<T: Into<f64>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, rhs: T) {
        let v = rhs.into();
        self.x *= v;
        self.y *= v;
        self.z *= v;
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
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

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, rhs: T) {
        let v = rhs.into();
        self.x /= v;
        self.y /= v;
        self.z /= v;
    }
}

impl<T: Into<f64>> From<(T, T, T)> for Vec3 {
    fn from(v: (T, T, T)) -> Self {
        vec3!(v.0.into(), v.1.into(), v.2.into())
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        EPS
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon)
            && f64::abs_diff_eq(&self.y, &other.y, epsilon)
            && f64::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl RelativeEq for Vec3 {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        f64::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && f64::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && f64::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        f64::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && f64::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && f64::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        iter.fold((0., 0., 0.).into(), |acc, cur| acc + cur)
    }
}

#[macro_export]
macro_rules! max {
    ($a:expr) => {$a};
    ($a:expr $(,$b:expr)+) => {{
        let t = max!($($b),*);
        if $a > t {
            $a
        } else {
            t
        }
    }}
}

#[macro_export]
macro_rules! min {
    ($a:expr) => {$a};
    ($a:expr $(,$b:expr)+) => {{
        let t = min!($($b),*);
        if $a < t {
            $a
        } else {
            t
        }
    }}
}

pub type Color = Vec3;

pub(crate) fn gen_point_in_sphere(radius: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let r = radius;
    let theta: f64 = rng.gen_range(0., 2. * PI);
    let phi: f64 = rng.gen_range(-f64::consts::FRAC_PI_2, f64::consts::FRAC_PI_2);

    vec3!(
        r * phi.sin() * theta.cos(),
        r * phi.sin() * theta.sin(),
        r * phi.cos()
    )
}

pub(crate) fn gen_point_in_disk(radius: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0., PI);
    let r = rng.gen_range(0., 1.);
    radius * r * vec3!(theta.cos(), theta.sin(), 0.)
}

pub trait ChunkIter<T, I: Iterator<Item=T>> {
    fn chunks(self, size: usize) -> Chunks<T, I>;
}

impl<T, I: Iterator<Item=T>> ChunkIter<T, I> for I {
    fn chunks(self, size: usize) -> Chunks<T, I> {
        Chunks {
            size,
            iter: self,
        }
    }
}

pub struct Chunks<T, I: Iterator<Item=T>> {
    size: usize,
    iter: I,
}

impl<T, I: Iterator<Item=T>> Iterator for Chunks<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let storage: Vec<_> = (0..self.size).into_iter().filter_map(|_| self.iter.next()).collect();
        if storage.is_empty() {
            None
        } else {
            Some(storage)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec3() {
        assert_abs_diff_eq!(vec3!(1, 2, 3).dot(vec3!(5, 40, 200)), 685.);

        assert_abs_diff_eq!(vec3!(1, 0, 0).cross(vec3!(0, 1, 0)), vec3!(0, 0, 1));
        assert_abs_diff_eq!(vec3!(0, 1, 0).cross(vec3!(0, 0, 1)), vec3!(1, 0, 0));
        assert_abs_diff_eq!(vec3!(0, 0, 1).cross(vec3!(1, 0, 0)), vec3!(0, 1, 0));
        assert_abs_diff_eq!(vec3!(0, 1, 0).cross(vec3!(1, 0, 0)), vec3!(0, 0, -1));
        assert_abs_diff_eq!(vec3!(0, 0, 1).cross(vec3!(0, 1, 0)), vec3!(-1, 0, 0));
        assert_abs_diff_eq!(vec3!(1, 0, 0).cross(vec3!(0, 0, 1)), vec3!(0, -1, 0));

        assert_abs_diff_eq!(vec3!(1, 2, 3) + vec3!(10, 100, 1000), vec3!(11, 102, 1003));
        assert_abs_diff_eq!(vec3!(1, 2, 3) + 10., vec3!(11, 12, 13));
        assert_abs_diff_eq!(vec3!(10, 100, 1000) + vec3!(1, 2, 3), vec3!(11, 102, 1003));
        assert_abs_diff_eq!(10. + vec3!(1, 2, 3), vec3!(11, 12, 13));
        assert_abs_diff_eq!(vec3!(1, 2, 3) - vec3!(10, 100, 1000), vec3!(-9, -98, -997));
        assert_abs_diff_eq!(vec3!(1, 2, 3) - 10., vec3!(-9, -8, -7));
        assert_abs_diff_eq!(vec3!(10, 100, 1000) - vec3!(1, 2, 3), -vec3!(-9, -98, -997));
        assert_abs_diff_eq!(10. - vec3!(1, 2, 3), -vec3!(-9, -8, -7));

        assert_abs_diff_eq!(vec3!(5, 6, 9) * vec3!(1, 2, 3), vec3!(5, 12, 27));
        assert_abs_diff_eq!(vec3!(1, 1, 1) / vec3!(1, 2, 3), vec3!(1, 0.5, 1. / 3.));

        assert_abs_diff_eq!(vec3!(1, 2, 3) * 5., vec3!(5, 10, 15));
        assert_abs_diff_eq!(5. * vec3!(1, 2, 3), vec3!(5, 10, 15));
        assert_abs_diff_eq!(vec3!(10, 20, 30) / 5., vec3!(2, 4, 6));
        assert_abs_diff_eq!(24. / vec3!(1, 2, 3), vec3!(24, 12, 8));

        let mut v = vec3!(1, 2, 3);
        v -= 10.;
        assert_abs_diff_eq!(v, vec3!(-9, -8, -7));
        let mut v = vec3!(1, 2, 3);
        v += 10.;
        assert_abs_diff_eq!(v, vec3!(11, 12, 13));
        let mut v = vec3!(1, 2, 3);
        v *= 10.;
        assert_abs_diff_eq!(v, vec3!(10, 20, 30));
        let mut v = vec3!(1, 2, 3);
        v /= 10.;
        assert_abs_diff_eq!(v, vec3!(0.1, 0.2, 0.3));

        assert_abs_diff_eq!(
            vec![vec3!(1, 2, 3), vec3!(10, 20, 30), vec3!(100, 200, 300)]
                .into_iter()
                .sum(),
            vec3!(111, 222, 333)
        );
    }

    #[test]
    fn test_min_max() {
        assert_abs_diff_eq!(5., min!(max!(0., 10., 20., 30.), min!(6., 9., 7.), 5.));
    }

    #[test]
    fn test_gen_point_in_sphere() {
        (0..100000).for_each(|_| {
            let o = gen_point_in_sphere(5.);
            assert!(o.dot(o) <= 25. + EPS);
        })
    }
}
