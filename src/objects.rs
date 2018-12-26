use std::cmp;
use std::rc::Rc;

use light::LightSource;
use ray::{HitRecord, Ray};
use ray::HitInfo;
use utils::{EPS, Vec3};

pub trait Hittable {
    fn hit_info(&self, ray: &Ray) -> Option<HitInfo>;
}

pub trait Shape: Hittable {
    fn hit_by(self: Rc<Self>, ray: &Ray) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct Triangle {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
}

impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3) -> Triangle {
        Triangle { p0, p1, p2 }
    }

    pub fn normal(&self) -> Vec3 {
        let Self { p0, p1, p2 } = self;
        (*p1 - *p0).cross(*p2 - *p0).normalize()
    }

    pub fn is_in_plane(&self, point: Vec3) -> bool {
        let v = self.p0 - point;
        v.dot(self.normal()).abs() < EPS
    }

    pub fn contain(&self, point: Vec3) -> bool {
        if !self.is_in_plane(point) {
            return false;
        }
        let pp0 = self.p0 - point;
        let pp1 = self.p1 - point;
        let pp2 = self.p2 - point;
        let t0 = pp0.cross(pp1);
        let t1 = pp1.cross(pp2);
        let t2 = pp2.cross(pp0);
        t0.dot(t1) > -EPS && t1.dot(t2) > -EPS
    }
}

impl Hittable for Triangle {
    fn hit_info(&self, ray: &Ray) -> Option<HitInfo> {
        let e1 = self.p1 - self.p0;
        let e2 = self.p2 - self.p0;
        let h = ray.dir.cross(e2);
        let a = e1.dot(h);
        if -EPS < a && a < EPS {
            return None;
        }
        let f = 1. / a;
        let s = ray.pos - self.p0;
        let u = f * s.dot(h);
        if u < 0. || u > 1. {
            return None;
        }
        let q = s.cross(e1);
        let v = f * ray.dir.dot(q);
        if v < 0. || u + v > 1. {
            return None;
        }
        let t = f * e2.dot(q);
        if t > EPS {
            Some(HitInfo::new(
                t,
                e1.cross(e2).normalize(),
                t * ray.dir + ray.pos,
                ray.dir,
            ))
        } else {
            None
        }
    }
}

impl Shape for Triangle {
    fn hit_by(self: Rc<Self>, ray: &Ray) -> Option<HitRecord> {
        self.hit_info(ray).map(|info| HitRecord { obj: self, info })
    }
}

#[derive(Debug)]
pub struct Square {
    tri0: Triangle,
    tri1: Triangle,
}

impl Square {
    pub fn new(center: Vec3, x: Vec3, y: Vec3, len: f32) -> Square {
        let p0 = center - x * len / 2. + y * len / 2.;
        let p1 = center - x * len / 2. - y * len / 2.;
        let p2 = center + x * len / 2. - y * len / 2.;
        let p3 = center + x * len / 2. + y * len / 2.;
        Square {
            tri0: Triangle::new(p0, p1, p2),
            tri1: Triangle::new(p1, p2, p3),
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.tri0.normal()
    }

    // anti-clockwise
    pub fn from_points(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Square {
            tri0: Triangle::new(p0, p1, p2),
            tri1: Triangle::new(p1, p2, p3),
        }
    }

    pub fn contain(&self, p: Vec3) -> bool {
        self.tri0.contain(p) || self.tri1.contain(p)
    }

    pub fn is_in_plane(&self, p: Vec3) -> bool {
        self.tri0.is_in_plane(p)
    }

    pub fn get_corners(&self) -> Vec<Vec3> {
        let Triangle { p0, p1, p2 } = self.tri0;
        vec![p0, p1, p2, self.tri1.p2]
    }
}

impl Hittable for Square {
    fn hit_info(&self, ray: &Ray) -> Option<HitInfo> {
        self.tri0.hit_info(ray).or(self.tri1.hit_info(ray))
    }
}

impl Shape for Square {
    fn hit_by(self: Rc<Self>, ray: &Ray) -> Option<HitRecord> {
        self.hit_info(ray).map(|info| HitRecord { obj: self, info })
    }
}

#[derive(Debug)]
pub struct Cube {
    pub center: Vec3,
    pub x: Vec3,
    pub y: Vec3,
    pub len: f32,
}

impl Cube {
    pub fn new(center: Vec3, x: Vec3, y: Vec3, len: f32) -> Self {
        Cube { center, x, y, len }
    }

    fn squares(&self) -> Vec<Square> {
        let x = self.x.normalize();
        let y = self.y.normalize();
        let z = x.cross(y).normalize();
        let c = self.center;
        let len = self.len;
        let mut result = Vec::<Square>::new();
        result.push(Square::new(c + x * (len / 2.), y, z, len));
        result.push(Square::new(c + y * (len / 2.), -x, z, len));
        result.push(Square::new(c - x * (len / 2.), -y, z, len));
        result.push(Square::new(c - y * (len / 2.), x, z, len));
        result.push(Square::new(c + z * (len / 2.), x, y, len));
        result.push(Square::new(c - z * (len / 2.), x, -y, len));

        result
    }
}

impl Hittable for Cube {
    fn hit_info(&self, ray: &Ray) -> Option<HitInfo> {
        self.squares()
            .iter()
            .filter_map(|square| square.hit_info(ray))
            .min_by(|r1, r2| {
                let d1 = r1.distance;
                let d2 = r2.distance;
                d1.partial_cmp(&d2).unwrap_or(cmp::Ordering::Equal)
            })
    }
}

impl Shape for Cube {
    fn hit_by(self: Rc<Self>, ray: &Ray) -> Option<HitRecord> {
        self.hit_info(ray).map(|info| HitRecord { obj: self, info })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(o: Vec3, r: f32) -> Self {
        Sphere {
            center: o,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit_info(&self, ray: &Ray) -> Option<HitInfo> {
        let a = ray.dir.len2();
        let b = 2. * (ray.pos - self.center).dot(ray.dir);
        let c = (ray.pos - self.center).len2() - self.radius.powi(2);
        let delta = b.powi(2) - 4. * a * c;
        if delta < 0. {
            return None;
        }
        let t1 = (-b - delta.sqrt()) / (2. * a);
        let t2 = (-b + delta.sqrt()) / (2. * a);
        if t2 < 0. {
            return None;
        }
        let t = if t1 < 0. { t2 } else { t1 };
        let point = ray.pos + ray.dir * t;
        let norm = (point - self.center).normalize();
        let norm_proj = ray.dir.proj_to(norm);
        let dir = ray.dir - 2. * norm_proj;
        Some(HitInfo::new(t, norm, point, ray.dir))
    }
}

impl Shape for Sphere {
    fn hit_by(self: Rc<Self>, ray: &Ray) -> Option<HitRecord> {
        self.hit_info(ray).map(|info| HitRecord { obj: self, info })
    }
}

pub struct World {
    pub objects: Vec<Rc<dyn Shape>>,
    pub lights: Vec<Rc<dyn LightSource>>,
}

impl World {
    pub fn empty() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_obj<T: Shape + 'static>(&mut self, obj: T) {
        self.objects.push(Rc::new(obj));
    }

    pub fn add_light<T: LightSource + 'static>(&mut self, light: T) {
        self.lights.push(Rc::new(light));
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use utils::*;

    use super::{Ray, Shape, Triangle};

    #[test]
    fn test_triangle() {
        let tri = Rc::new(Triangle::new(
            vec3!(0, -1, 0),
            vec3!(1, 1, 0),
            vec3!(-1, 1, 0),
        ));
        let ray0 = Ray::new(vec3!(0, 0, 1), vec3!(0, 0, -1));
        let record = tri.clone().hit_by(&ray0).unwrap();
        assert_relative_eq!(record.position(), EPS * record.out_dir() + vec3!(0, 0, 0));
        assert_relative_eq!(record.out_dir(), vec3!(0, 0, 1));
        assert_relative_eq!(record.normal(), vec3!(0, 0, 1));

        let ray1 = Ray::new(vec3!(3, 0, 1), vec3!(0, 0, -1));
        assert!(tri.clone().hit_by(&ray1).is_none());

        let ray2 = Ray::new(vec3!(3, 0, -1), vec3!(0, 0, 1));
        assert!(tri.clone().hit_by(&ray2).is_none());

        let ray3 = Ray::new(vec3!(0, 0, 1), vec3!(1, 0, 0));
        assert!(tri.clone().hit_by(&ray3).is_none());

        let ray4 = Ray::new(vec3!(0, 0, -1), vec3!(1, 0, 0));
        assert!(tri.clone().hit_by(&ray4).is_none());

        let ray5 = Ray::new(vec3!(0, 0, -1), vec3!(0, 0, -1));
        assert!(tri.clone().hit_by(&ray5).is_none());

        assert!(tri.clone().is_in_plane(vec3!(1, 4, 0)));
        assert!(!tri.clone().is_in_plane(vec3!(1, 4, 0.1)));

        assert!(tri.clone().contain(vec3!(0, 0, 0)));
        assert!(!tri.clone().contain(vec3!(3, 0, 0)));
        assert!(!tri.clone().contain(vec3!(0, 0, 0.1)));
        assert!(tri.clone().contain(vec3!(1, 1, 0)));
    }
}
