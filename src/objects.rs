use image::{Pixel, Rgb};
use light::{LightSource, PointLight};
use ray::{Ray, Reflectable};
use std::rc::Rc;
use std::slice::{Iter, IterMut};
use utils::Vec3;

#[derive(Debug)]
pub struct Square {
    pub center: Vec3,
    pub x: Vec3,
    pub y: Vec3,
    pub len: f32,
}

impl Square {
    pub fn normal(&self) -> Vec3 {
        self.x.cross(self.y).normal()
    }

    // anti-clockwise
    pub fn from_points(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Square {
            center: (p0 + p1 + p2 + p3) / 4.,
            x: (p2 - p1).normal(),
            y: (p0 - p1).normal(),
            len: (p0 - p1).len(),
        }
    }

    pub fn contain(&self, p: Vec3) -> bool {
        let n = self.normal();
        let w = (p - self.center) / self.len;

        let a = w.dot(self.x.normal());
        let b = w.dot(self.y.normal());

        if relative_ne!(n.dot(w), 0.) {
            return false;
        }

        if (-0.5 < a || relative_eq!(-0.5, a))
            && (a < 0.5 || relative_eq!(a, 0.5))
            && (-0.5 < b || relative_eq!(-0.5, b))
            && (b < 0.5 || relative_eq!(b, 0.5))
        {
            true
        } else {
            false
        }
    }

    pub fn get_corners(&self) -> Vec<Vec3> {
        let mut result = Vec::new();
        let Self { center, x, y, len } = *self;
        result.push(center + (-x - y) * (len / 2.));
        result.push(center + (x - y) * (len / 2.));
        result.push(center + (x + y) * (len / 2.));
        result.push(center + (-x + y) * (len / 2.));
        result
    }
}

impl Reflectable for Square {
    fn reflect(&self, ray: &Ray) -> Option<Ray> {
        let mut n = self.normal();
        let a = self.center;

        if ray.dir.dot(n) > 0. {
            n *= -1.;
        }

        let t = (a - ray.pos).dot(n) / (ray.dir.dot(n));
        let p = ray.pos + ray.dir * t; // hit point

        //        debug!("p {:?}, {:?}", p, self);
        //        debug!("n {:?}, {:?}", n, ray);
        //        debug!("t {:?}", t);

        if t < 0. || !self.contain(p) || relative_eq!(n.dot(ray.dir), 0.) {
            return None;
        }

        let mut d = p - ray.pos;

        let h = n * d.dot(n).abs();
        d += h * 2.;

        Some(Ray { pos: p, dir: d })
    }

    fn decay(&self) -> f32 {
        1.
    }

    fn color(&self, reflect_ray: &Ray) -> Rgb<u8> {
        *Rgb::from_slice(&[255, 0, 0])
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
    fn planes(&self) -> Vec<Square> {
        let x = self.x.normal();
        let y = self.y.normal();
        let z = x.cross(y).normal();
        let c = self.center;
        let len = self.len;
        let mut result = Vec::<Square>::new();
        result.push(Square {
            center: c + x * (len / 2.),
            x: y,
            y: z,
            len,
        });
        result.push(Square {
            center: c + y * (len / 2.),
            x: -x,
            y: z,
            len,
        });
        result.push(Square {
            center: c - x * (len / 2.),
            x: -y,
            y: z,
            len,
        });
        result.push(Square {
            center: c - y * (len / 2.),
            x,
            y: z,
            len,
        });
        result.push(Square {
            center: c + z * (len / 2.),
            x,
            y,
            len,
        });
        result.push(Square {
            center: c - z * (len / 2.),
            x,
            y: -y,
            len,
        });

        result
    }
}

impl Reflectable for Cube {
    fn reflect(&self, ray: &Ray) -> Option<Ray> {
        use std::cmp::Ordering;
        self.planes()
            .iter()
            .filter_map(|plane| plane.reflect(ray))
            .min_by(|r1, r2| {
                let d1 = r1.pos.distance(ray.pos);
                let d2 = r2.pos.distance(ray.pos);
                d1.partial_cmp(&d2).unwrap_or(Ordering::Equal)
            })
    }

    fn decay(&self) -> f32 {
        1.
    }

    fn color(&self, reflect_ray: &Ray) -> Rgb<u8> {
        *Rgb::from_slice(&[255, 0, 0])
    }
}

pub struct World {
    pub objects: Vec<Rc<dyn Reflectable>>,
    pub lights: Vec<Rc<dyn LightSource>>,
}

impl World {
    pub fn new() -> World {
        let mut objects: Vec<Rc<dyn Reflectable>> = Vec::new();
        const EDG_LEN: f32 = 20.;
        objects.push(Rc::new(Square::from_points(
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
        )));
        objects.push(Rc::new(Square::from_points(
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
        )));
        objects.push(Rc::new(Square::from_points(
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
        )));
        objects.push(Rc::new(Square::from_points(
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
        )));
        objects.push(Rc::new(Square::from_points(
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., EDG_LEN / 2.),
        )));
        objects.push(Rc::new(Square::from_points(
            Vec3::new(-EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(-EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., -EDG_LEN / 2., -EDG_LEN / 2.),
            Vec3::new(EDG_LEN / 2., EDG_LEN / 2., -EDG_LEN / 2.),
        )));
        let mut lights = Vec::<Rc<dyn LightSource>>::new();
        lights.push(Rc::new(PointLight::new(Vec3::new(100., 0., 0.))));
        World { objects, lights }
    }

    pub fn push(&mut self, obj: Rc<dyn Reflectable>) {
        self.objects.push(obj);
    }

    pub fn iter(&self) -> Iter<Rc<dyn Reflectable>> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Rc<dyn Reflectable>> {
        self.objects.iter_mut()
    }
}

impl IntoIterator for World {
    type Item = Rc<dyn Reflectable>;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}

impl<'a> IntoIterator for &'a World {
    type Item = &'a Rc<dyn Reflectable>;
    type IntoIter = <&'a Vec<Rc<dyn Reflectable>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.objects).into_iter()
    }
}
