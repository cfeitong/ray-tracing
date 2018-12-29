use std::rc::Rc;

use rand::prelude::*;

use crate::{
    object::{Object, RcObjectExt, World},
    util::{EPS, gen_point_in_sphere, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn hit(&self, world: &World) -> Option<HitRecord> {
        world
            .objects
            .iter()
            .filter_map(|obj| obj.hit_by(self))
            .min_by(|a, b| {
                let dist_a = a.distance();
                let dist_b = b.distance();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }

    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Ray {
            pos,
            dir: dir.normalize(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    up: Vec3,
    sight: Vec3,
    sample_rate: f32,
}

impl Camera {
    pub fn with_sample_rate(mut self, rate: f32) -> Self {
        self.sample_rate = rate;
        self
    }

    /// adjust this camera to look at `point`.
    pub fn look(&mut self, point: Vec3) {
        self.sight = (point - self.pos).normalize();
        let right = self.right();
        self.up = right.cross(self.sight).normalize();
    }

    /// return up direction of this camera.
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// return right direction of this camera.
    pub fn right(&self) -> Vec3 {
        self.sight.cross(self.up).normalize()
    }

    /// emit needed ray through a 1 unit away square screen whose size is 2 unit.
    pub fn emit_rays(&self, width: u32, height: u32) -> Vec<(u32, u32, Ray)> {
        (0..width)
            .flat_map(|w| (0..height).map(move |h| (w, h)))
            .flat_map(|(w, h)| {
                let mut rays = Vec::new();
                let mut sample = self.sample_rate;
                let mut rng = rand::thread_rng();

                while rng.gen_range(0., 1.) <= sample {
                    let b = 0.5;
                    let (rw, rh) = (rng.gen_range(-b, b), rng.gen_range(-b, b));

                    // f32 width and height
                    let (fw, fh) = (w as f32 + rw + 0.5, h as f32 + rh + 0.5);
                    // f32 window width and window height
                    let (ww, wh) = (width as f32, height as f32);
                    let (fw, fh) = (fw / ww, fh / wh);
                    let top_left = self.pos + self.sight + self.up() - self.right();
                    let point = top_left + 2. * self.right() * fw - 2. * self.up() * fh;
                    let ray = Ray::new(self.pos, point - self.pos);
                    rays.push((w, h, ray));

                    sample -= 1.;
                }
                rays
            })
            .collect()
    }

    /// create a camera which is at `pos` and look at `point`.
    pub fn new(from: Vec3, to: Vec3) -> Camera {
        let mut camera = Camera {
            pos: from,
            up: Vec3::new(0., 0., 1.),
            sight: Vec3::new(0., 0., 1.),
            sample_rate: 1.,
        };
        camera.look(to);
        camera
    }
}

pub struct HitRecord {
    pub(crate) obj: Rc<Object>,
    pub(crate) info: HitInfo,
}

impl HitRecord {
    pub fn new(
        obj: Rc<Object>,
        distance: f32,
        norm: Vec3,
        hit_point: Vec3,
        in_dir: Vec3,
    ) -> HitRecord {
        let norm = norm.normalize();
        HitRecord {
            obj,
            info: HitInfo::new(distance, norm, hit_point, in_dir),
        }
    }

    pub fn angle(&self) -> f32 {
        self.out_dir().dot(self.info.norm).acos()
    }

    pub fn out_dir(&self) -> Vec3 {
        self.info.out_dir
    }

    pub fn in_dir(&self) -> Vec3 {
        self.info.in_dir
    }

    pub fn object(&self) -> Rc<Object> {
        self.obj.clone()
    }

    pub fn normal(&self) -> Vec3 {
        self.info.norm
    }

    pub fn specular_ray(&self) -> Ray {
        let pos = self.position();
        Ray::new(pos, self.info.out_dir)
    }

    pub fn diffuse_ray(&self) -> Ray {
        let pos = self.position();
        let o = pos + self.info.norm;
        let p = gen_point_in_sphere(1.);
        let t = o + p;
        let dir = (t - pos).normalize();
        Ray::new(pos, dir)
    }

    pub fn position(&self) -> Vec3 {
        self.info.hit_point
    }

    pub fn distance(&self) -> f32 {
        self.info.distance
    }

    pub fn out_ray(&self) -> Ray {
        self.info.reflect()
    }
}

#[derive(Clone, Copy)]
pub struct HitInfo {
    pub distance: f32,
    pub norm: Vec3,
    hit_point: Vec3,
    pub in_dir: Vec3,
    pub out_dir: Vec3,
}

impl HitInfo {
    pub fn new(distance: f32, norm: Vec3, hit_point: Vec3, in_dir: Vec3) -> HitInfo {
        let mut norm = norm.normalize();
        if norm.dot(in_dir) > -EPS {
            norm = -norm;
        }
        let out_dir = in_dir - 2. * in_dir.proj_to(norm);
        HitInfo {
            distance,
            norm,
            hit_point,
            in_dir,
            out_dir,
        }
    }

    pub fn reflect(&self) -> Ray {
        Ray {
            pos: self.position(),
            dir: self.out_dir,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.hit_point + EPS * self.out_dir
    }

    pub fn in_ray(&self) -> Ray {
        Ray {
            pos: self.hit_point,
            dir: self.in_dir,
        }
    }

    // ratio = 1 / index of refraction
    pub fn refract(&self, ratio: f32) -> Ray {
        let cos = self.in_dir.dot(self.norm);
        let dir = -self.norm * (1. - ratio.powi(2) * (1. - cos.powi(2)))
            + ratio * (self.in_dir + cos * self.norm);
        let pos = self.hit_point + EPS * dir;
        Ray { pos, dir }
    }
}
