use image::Rgb;
use objects::World;
use utils::Vec3;

use std::f32::INFINITY;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn hit(&self, world: &World) -> Option<HitPoint> {
        world
            .iter()
            .filter_map(|obj| obj.reflect(self).map(|ray| (obj.clone(), ray)))
            .min_by(|a, b| {
                let dist_a = self.pos.distance(a.1.pos);
                let dist_b = self.pos.distance(b.1.pos);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(obj, ray)| HitPoint { in_ray: ray, obj })
    }
}

pub trait Reflectable {
    fn reflect(&self, ray: &Ray) -> Option<Ray>;
    fn decay(&self) -> f32;
    fn color(&self, reflect_ray: &Ray) -> Rgb<u8>;
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    up: Vec3,
    sight: Vec3,
}

impl Camera {
    /// adjust this camera to look at `point`.
    pub fn look(&mut self, point: Vec3) {
        self.sight = (point - self.pos).normal();
    }

    /// return up direction of this camera.
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// return right direction of this camera.
    pub fn right(&self) -> Vec3 {
        self.sight.cross(self.up).normal()
    }

    /// emit needed ray through a 1 unit away square screen whose size is 2 unit.
    pub fn emit_ray(&self, width: u32, height: u32) -> Vec<(u32, u32, Ray)> {
        (0..width)
            .flat_map(|w| (0..height).map(move |h| (w, h)))
            .map(|(w, h)| {
                // f32 width and height
                let (fw, fh) = (w as f32, h as f32);
                // f32 window width and window height
                let (ww, wh) = (width as f32, height as f32);
                let (fw, fh) = (2. * fw / ww, 2. * fh / wh);
                let left_up_corner = self.pos + self.sight + self.up() - self.right();
                let point = left_up_corner - self.up() * fh + self.right() * fw;
                let ray = Ray {
                    pos: self.pos,
                    dir: (point - self.pos).normal(),
                };
                (w, h, ray)
            })
            .collect()
    }

    /// create a camera which is at `pos` and look at `point`.
    pub fn new(pos: Vec3, point: Vec3) -> Camera {
        Camera {
            pos,
            up: Vec3::new(0., 0., 1.),
            sight: (point - pos).normal(),
        }
    }
}

#[derive(Clone)]
pub struct HitPoint {
    pub in_ray: Ray,
    pub obj: Rc<dyn Reflectable>,
}

impl HitPoint {
    pub fn angle(&self, norm: Vec3) -> f32 {
        self.in_ray.dir.cross(norm).len().asin()
    }

    pub fn reflect_ray(&self) -> Ray {
        // there must be a reflected ray since its a valid hit point.
        self.obj.reflect(&self.in_ray).unwrap()
    }

    pub fn position(&self) -> Vec3 {
        self.in_ray.pos
    }
}
