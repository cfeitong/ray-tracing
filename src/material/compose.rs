use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::*,
};

use super::*;

#[derive(Clone, Copy)]
pub struct Metal {
    s: Specular,
    fuzz: f32,
    color: Color,
}

impl Metal {
    pub fn new(fuzz: f32, albedo: f32) -> Self {
        let fuzz = if fuzz > 1. { 1. } else { fuzz };
        Metal {
            s: Specular::new(albedo),
            fuzz,
            color: (1., 1., 1.).into(),
        }
    }

    pub fn with_fuzz(mut self, fuzz: f32) -> Self {
        self.fuzz = fuzz;
        self
    }

    pub fn with_albedo(mut self, albedo: f32) -> Self {
        self.s = self.s.with_albedo(albedo);
        self
    }

    pub fn with_color<T: Into<Color>>(mut self, color: T) -> Self {
        self.color = color.into();
        self
    }
}

impl Material for Metal {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color {
        let si = self.s.render(hit, world, traced);
        si * self.color
    }
    fn scatter(&self, hit: &HitInfo) -> Vec<Ray> {
        let mut r = hit.reflect();
        r.dir = (r.dir() + gen_point_in_sphere(self.fuzz)).unit();
        vec![r]
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    s: Specular,
    r: Transparent,
}

impl Dielectric {
    pub fn new(ior: f32) -> Self {
        Dielectric {
            s: Specular::new(1.),
            r: Transparent::new(0., ior),
        }
    }

    pub fn with_ior(mut self, ior: f32) -> Self {
        self.r = self.r.with_ior(ior);
        self
    }
}

impl Material for Dielectric {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Vec3]) -> Vec3 {
        self.r.render(hit, world, traced)
    }

    fn scatter(&self, hit: &HitInfo) -> Vec<Ray> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0., 1.) < hit.reflect_prob(self.r.ior()) {
            self.s.scatter(hit)
        } else {
            self.r.scatter(hit)
        }
    }
}

#[derive(Clone, Copy)]
pub struct LambertianModel {
    s: Specular,
    c: Color,
}

impl LambertianModel {
    pub fn new(albedo: f32) -> Self {
        LambertianModel {
            s: Specular::new(albedo),
            c: (1., 1., 1.).into(),
        }
    }

    pub fn with_color<T: Into<Color>>(mut self, color: T) -> Self {
        self.c = color.into();
        self
    }
}

impl Material for LambertianModel {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color {
        self.c * self.s.render(hit, world, traced)
    }
    fn scatter(&self, hit: &HitInfo) -> Vec<Ray> {
        let mut r = hit.reflect();
        r.dir = (r.dir() + gen_point_in_sphere(1.)).unit();
        vec![r]
    }
}
