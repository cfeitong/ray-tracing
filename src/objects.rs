use light::LightSource;
use ray::{Ray, Reflectable};
use std::rc::Rc;
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
        self.x.cross(self.y).normalize()
    }

    // anti-clockwise
    pub fn from_points(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Square {
            center: (p0 + p1 + p2 + p3) / 4.,
            x: (p2 - p1).normalize(),
            y: (p0 - p1).normalize(),
            len: (p0 - p1).len(),
        }
    }

    pub fn contain(&self, p: Vec3) -> bool {
        let n = self.normal();
        let w = (p - self.center) / self.len;

        let a = w.dot(self.x.normalize());
        let b = w.dot(self.y.normalize());

        if relative_ne!(n.dot(w), 0.) {
            return false;
        }

        (-0.5 < a || relative_eq!(-0.5, a))
            && (a < 0.5 || relative_eq!(a, 0.5))
            && (-0.5 < b || relative_eq!(-0.5, b))
            && (b < 0.5 || relative_eq!(b, 0.5))
    }

    pub fn is_in_plane(&self, p: Vec3) -> bool {
        let t = self.center - p;
        relative_eq!(t.dot(self.normal()), 0.)
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

        if t < 0. || !self.contain(p) || relative_eq!(n.dot(ray.dir), 0.) {
            return None;
        }

        let mut d = p - ray.pos;

        let h = n * d.dot(n).abs();
        d += h * 2.;

        Some(Ray::new(p, d))
    }

    fn decay(&self) -> f32 {
        1.
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
        self.squares()
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

impl Reflectable for Sphere {
    fn reflect(&self, ray: &Ray) -> Option<Ray> {
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
        let reflected_ray = Ray::new(point, -norm_proj + (ray.dir - norm_proj));
        Some(reflected_ray)
    }

    fn decay(&self) -> f32 {
        1.
    }
}

pub struct World {
    pub objects: Vec<Rc<dyn Reflectable>>,
    pub lights: Vec<Rc<dyn LightSource>>,
}

impl World {
    pub fn empty() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: Rc<dyn Reflectable>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Rc<dyn LightSource>) {
        self.lights.push(light);
    }
}
