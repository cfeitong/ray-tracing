use std::borrow::Borrow;

use crate::object::World;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::util::Color;
use crate::util::EPS;
use crate::util::Vec3;

pub trait LightSource {
    /// light intensity in [0, 1]
    fn intensity(&self, point: Vec3) -> f32;
    fn dir_at(&self, point: Vec3) -> Vec3;
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool;
    fn color(&self) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub struct ParallelLight {
    dir: Vec3,
    light_color: Color,
}

impl ParallelLight {
    pub fn new(dir: Vec3) -> ParallelLight {
        ParallelLight {
            dir,
            light_color: vec3!(1, 1, 1),
        }
    }

    pub fn with_color(mut self, color: Color) -> ParallelLight {
        self.light_color = color;
        self
    }
}

impl LightSource for ParallelLight {
    fn intensity(&self, _point: Vec3) -> f32 {
        1.
    }
    fn dir_at(&self, _point: Vec3) -> Vec3 {
        self.dir
    }
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool {
        let dir = -self.dir_at(point);
        let ray = Ray::new(point, dir);
        ray.hit(world).is_some()
    }
    fn color(&self) -> Color {
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
    fn intensity(&self, point: Vec3) -> f32 {
        1. / (self.pos - point).len2()
    }

    fn dir_at(&self, point: Vec3) -> Vec3 {
        (point - self.pos).normalize()
    }
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool {
        let dir = -self.dir_at(point);
        let ray = Ray::new(point, dir);
        ray.hit(world)
            .map(|hit| {
                let l1 = (point - hit.position()).len2();
                let l2 = (point - self.pos).len2();
                l1 + EPS < l2
            })
            .unwrap_or(false)
    }

    fn color(&self) -> Color {
        self.light_color
    }
}

impl PointLight {
    pub fn with_color(mut self, c: Color) -> Self {
        self.light_color = c;
        self
    }

    pub fn new(pos: Vec3) -> Self {
        PointLight {
            pos,
            light_color: vec3!(1, 1, 1),
        }
    }
}
