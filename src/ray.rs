use std::sync::Arc;

use rand::prelude::*;

use crate::{
    object::{ArcObjectExt, Object, World},
    util::*,
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub(crate) pos: Vec3,
    pub(crate) dir: Vec3,
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
            dir: dir.unit(),
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    up: Vec3,
    sight: Vec3,
    sample_rate: f32,
    focus_dist: f32,
    aperture: f32,
    fov: f32,
    aspect: f32,
}

impl Camera {
    pub fn with_sample_rate(mut self, rate: f32) -> Self {
        self.sample_rate = rate;
        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f32) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn with_aperture(mut self, aperture: f32) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn with_fov(mut self, deg: f32) -> Self {
        self.fov = deg / 180. * PI;
        self
    }

    pub fn with_aspect(mut self, aspect: f32) -> Self {
        self.aspect = aspect;
        self
    }

    /// adjust this camera to look at `point`.
    pub fn look(&mut self, point: Vec3) {
        self.sight = (point - self.pos).unit();
        let right = self.right();
        self.up = right.cross(self.sight).unit();
    }

    /// return up direction of this camera.
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// return right direction of this camera.
    pub fn right(&self) -> Vec3 {
        self.sight.cross(self.up).unit()
    }

    /// return sight direction of this camera.
    pub fn sight(&self) -> Vec3 {
        self.sight
    }

    /// emit rays through a focus distance unit away square screen whose size is 2*focus_dist unit.
    pub fn emit_rays(&self, width: u32, height: u32) -> Vec<(u32, u32, Ray)> {
        let mut rng = rand::thread_rng();
        let vh = 2. * (self.fov / 2.).tan() * self.focus_dist;
        let vw = vh * self.aspect;
        let pw = vw / width as f32 * self.right();
        let ph = vh / height as f32 * self.up();
        let center = self.pos + self.focus_dist * self.sight;
        let bias = 0.5 * (pw - ph);
        let top_left = center - vw * self.right() / 2. + vh * self.up() / 2. + bias;
        (0..width)
            .flat_map(|w| (0..height).map(move |h| (w, h)))
            .flat_map(|(w, h)| {
                let mut sample = self.sample_rate;
                let mut rays = Vec::new();

                while rng.gen_range(0., 1.) <= sample {
                    let b = 0.5;
                    let (rw, rh) = (rng.gen_range(-b, b), rng.gen_range(-b, b));
                    let to = top_left + (w as f32 + rw) * pw - (h as f32 + rh) * ph;
                    //                    println!("{}, {}, {:?}", w,h,to);

                    let rd = gen_point_in_disk(self.aperture / 2.);
                    let offset = self.right() * rd.x + self.up() * rd.y;
                    let from = self.pos + offset;

                    let ray = Ray::new(from, to - from);
                    rays.push((w, h, ray));

                    sample -= 1.;
                }
                rays
            })
            .collect()
    }

    /// create a camera which is at `pos` and look at `point`.
    pub fn new<T: Into<Vec3>>(from: T, to: T) -> Camera {
        let mut camera = Camera {
            pos: from.into(),
            up: Vec3::new(0., 0., 1.),
            sight: Vec3::new(0., 0., 1.),
            sample_rate: 1.,
            focus_dist: 1.,
            aperture: 0.,
            fov: 45.,
            aspect: 1.,
        };
        camera.look(to.into());
        camera
    }
}

pub struct HitRecord {
    pub(crate) obj: Arc<Object>,
    pub(crate) info: HitInfo,
}

impl HitRecord {
    pub fn new(
        obj: Arc<Object>,
        distance: f32,
        norm: Vec3,
        hit_point: Vec3,
        dir_in: Vec3,
    ) -> HitRecord {
        let norm = norm.unit();
        HitRecord {
            obj,
            info: HitInfo::new(distance, norm, hit_point, dir_in),
        }
    }

    pub fn angle(&self) -> f32 {
        self.dir_out().dot(self.info.norm).acos()
    }

    pub fn dir_out(&self) -> Vec3 {
        self.info.dir_out
    }

    pub fn dir_in(&self) -> Vec3 {
        self.info.dir_in
    }

    pub fn object(&self) -> Arc<Object> {
        self.obj.clone()
    }

    pub fn normal(&self) -> Vec3 {
        self.info.norm
    }

    pub fn specular_ray(&self) -> Ray {
        let pos = self.pos();
        Ray::new(pos, self.info.dir_out)
    }

    pub fn diffuse_ray(&self) -> Ray {
        let pos = self.pos();
        let o = pos + self.info.norm;
        let p = gen_point_in_sphere(1.);
        let t = o + p;
        let dir = (t - pos).unit();
        Ray::new(pos, dir)
    }

    pub fn pos(&self) -> Vec3 {
        self.info.hit_point
    }

    pub fn distance(&self) -> f32 {
        self.info.distance
    }

    pub fn out_ray(&self) -> Ray {
        self.info.reflect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitInfo {
    distance: f32,
    norm: Vec3,
    hit_point: Vec3,
    dir_in: Vec3,
    dir_out: Vec3,
    outward: bool,
}

impl HitInfo {
    pub fn new(distance: f32, norm: Vec3, hit_point: Vec3, dir_in: Vec3) -> HitInfo {
        let mut norm = norm.unit();
        let dir_in = dir_in.unit();
        let mut outward = false;
        if norm.dot(dir_in) > -EPS {
            norm = -norm;
            outward = true;
        }
        let dir_out = (dir_in - 2. * dir_in.proj_to(norm)).unit();
        HitInfo {
            distance,
            norm,
            hit_point,
            dir_in,
            dir_out,
            outward,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn normal(&self) -> Vec3 {
        self.norm
    }

    pub fn dir_in(&self) -> Vec3 {
        self.dir_in
    }

    pub fn dir_out(&self) -> Vec3 {
        self.dir_out
    }

    pub fn pos(&self) -> Vec3 {
        self.hit_point + EPS * self.dir_out
    }

    pub fn is_to_outward(&self) -> bool {
        self.outward
    }

    pub fn ray_in(&self) -> Ray {
        Ray {
            pos: self.hit_point,
            dir: self.dir_in,
        }
    }

    pub fn reflect(&self) -> Ray {
        Ray {
            pos: self.pos(),
            dir: self.dir_out,
        }
    }

    // see https://blog.csdn.net/yinhun2012/article/details/79472364 for details
    // ratio = inward material ior / outward material ior
    pub fn refract(&self, ratio: f32) -> Option<Ray> {
        let uv = self.dir_in;
        let n = self.norm;
        let cos = uv.dot(n);

        let discriminant = 1.0 - ratio.powi(2) * (1.0 - cos.powi(2));
        if discriminant > 0.0 {
            let dir = ratio * (uv - n * cos) - n * discriminant.sqrt();
            Some(Ray {
                pos: self.hit_point + EPS * dir,
                dir,
            })
        } else {
            None
        }
    }

    // Schlick's approximation
    // see https://www.wikiwand.com/en/Schlick%27s_approximation for details
    pub fn reflect_prob(&self, ior: f32) -> f32 {
        let r0 = (1. - ior) / (1. + ior).powi(2);
        let cos = self.dir_in.dot(self.norm).abs();
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}
