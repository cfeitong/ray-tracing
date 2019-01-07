use std::sync::Arc;

use crate::{
    object::{Shape, World},
    ray::{HitInfo, Ray},
    util::{Color, EPS, Vec3},
};

pub trait LightSource: Sync + Send {
    /// light intensity in [0, 1]
    fn intensity(&self, hit: &HitInfo) -> f32;
    fn dir_at(&self, hit: &HitInfo) -> Vec3;
    fn color(&self, dir: &HitInfo) -> Color;

    fn is_in_shadow(&self, hit: &HitInfo, world: &World) -> bool {
        hit.reflect().hit(world).is_some()
    }

    fn looked(&self, ray: &Ray, world: &World) -> Option<Color> {
        None
    }

    fn illuminate(&self, hit: &HitInfo, world: &World) -> Vec3 {
        if self.is_in_shadow(hit, world) {
            (0., 0., 0.).into()
        } else {
            self.intensity(hit) * self.color(hit)
        }
    }
}

pub struct LightInfo<'a> {
    light: &'a dyn LightSource,
    hit: &'a HitInfo,
    world: &'a World,
}

impl LightInfo<'_> {
    pub fn new<'a>(
        light: &'a dyn LightSource,
        hit: &'a HitInfo,
        world: &'a World,
    ) -> LightInfo<'a> {
        LightInfo { light, hit, world }
    }

    pub fn intensity(&self) -> f32 {
        self.light.intensity(self.hit)
    }

    pub fn dir(&self) -> Vec3 {
        self.light.dir_at(self.hit)
    }

    pub fn is_in_shadow(&self) -> bool {
        self.light.is_in_shadow(self.hit, self.world)
    }

    pub fn color(&self) -> Color {
        self.light.color(self.hit)
    }

    pub fn illuminate(&self) -> Vec3 {
        self.light.illuminate(self.hit, self.world)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParallelLight {
    dir: Vec3,
    light_color: Color,
}

impl ParallelLight {
    pub fn new<T: Into<Vec3>>(dir: T) -> ParallelLight {
        ParallelLight {
            dir: dir.into(),
            light_color: vec3!(1, 1, 1),
        }
    }

    pub fn with_color(mut self, color: Color) -> ParallelLight {
        self.light_color = color;
        self
    }
}

impl LightSource for ParallelLight {
    fn intensity(&self, _hit: &HitInfo) -> f32 {
        1.
    }
    fn dir_at(&self, _hit: &HitInfo) -> Vec3 {
        self.dir
    }
    fn is_in_shadow(&self, hit: &HitInfo, world: &World) -> bool {
        let point = hit.pos();
        let dir = -self.dir_at(hit);
        let ray = Ray::new(point, dir);
        ray.hit(world).is_some()
    }
    fn color(&self, _hit: &HitInfo) -> Color {
        self.light_color
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pos: Vec3,
    light_color: Color,
}

// TODO: add intensity decay with distance
impl LightSource for PointLight {
    fn intensity(&self, hit: &HitInfo) -> f32 {
        1. / (self.pos - hit.pos()).len2()
    }

    fn dir_at(&self, hit: &HitInfo) -> Vec3 {
        (hit.pos() - self.pos).unit()
    }
    fn is_in_shadow(&self, hit: &HitInfo, world: &World) -> bool {
        let point = hit.pos();
        let dir = -self.dir_at(hit);
        let ray = Ray::new(point, dir);
        ray.hit(world)
            .map(|hit| {
                let l1 = (point - hit.pos()).len2();
                let l2 = (point - self.pos).len2();
                l1 + EPS < l2
            })
            .unwrap_or(false)
    }

    fn color(&self, _hit: &HitInfo) -> Color {
        self.light_color
    }
}

impl PointLight {
    pub fn with_color(mut self, c: Color) -> Self {
        self.light_color = c;
        self
    }

    pub fn new<T: Into<Vec3>>(pos: T) -> Self {
        PointLight {
            pos: pos.into(),
            light_color: vec3!(1, 1, 1),
        }
    }
}

// sky light from `Ray Tracing in One Weekend`
#[derive(Clone, Copy, Debug)]
pub struct SkyLight;

impl SkyLight {
    fn color_from(&self, dir: Vec3) -> Color {
        let t = 0.5 * (dir.z + 1.0);
        let v = 1.0 - t;

        let a = v * vec3!(1.0, 1.0, 1.0);
        let b = t * vec3!(0.5, 0.7, 1.0);
        a + b
    }
}

impl LightSource for SkyLight {
    fn intensity(&self, _hit: &HitInfo) -> f32 {
        1.
    }

    fn dir_at(&self, hit: &HitInfo) -> Vec3 {
        -hit.dir_out()
    }

    fn color(&self, hit: &HitInfo) -> Vec3 {
        let dir = hit.dir_out();
        self.color_from(dir)
    }

    fn is_in_shadow(&self, hit: &HitInfo, world: &World) -> bool {
        hit.reflect().hit(world).is_some()
    }

    fn looked(&self, ray: &Ray, world: &World) -> Option<Color> {
        if ray.hit(world).is_none() {
            Some(self.color_from(ray.dir))
        } else {
            None
        }
    }
}

pub struct LightShape {
    shape: Box<Shape>,
    color: Color,
}

impl LightShape {
    pub fn new<T: Shape + 'static>(shape: T) -> Self {
        LightShape {
            shape: Box::new(shape),
            color: (1., 1., 1.).into(),
        }
    }
}

impl LightSource for LightShape {
    fn intensity(&self, hit: &HitInfo) -> f32 {
        if self.shape.hit_info(&hit.reflect()).is_some() {
            1.
        } else {
            0.
        }
    }

    fn dir_at(&self, hit: &HitInfo) -> Vec3 {
        -hit.reflect().dir
    }

    fn color(&self, _dir: &HitInfo) -> Vec3 {
        self.color
    }

    fn is_in_shadow(&self, hit: &HitInfo, world: &World) -> bool {
        let r = hit.reflect();
        let ha = r.hit(world);
        let hb = self.shape.hit_info(&r);
        ha.map_or(false, |rec| {
            hb.map_or(false, |info| rec.info.distance() < info.distance())
        })
    }

    fn looked(&self, ray: &Ray, world: &World) -> Option<Color> {
        let info = self.shape.hit_info(ray)?;
        ray.hit(world).map_or(Some(self.color), |rec| {
            if info.distance() < rec.info.distance() {
                Some(self.color)
            } else {
                None
            }
        })
    }
}
