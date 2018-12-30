use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::Color,
};
use crate::util::gen_point_in_sphere;

use super::{Diffuse, Material, Specular};

pub struct Metal {
    s: Specular,
    d: Diffuse,
}

impl Metal {
    pub fn new() -> Self {
        Metal {
            s: Specular::new(0.8).with_albedo(0.7),
            d: Diffuse::new().with_shininess(3.).with_diffuse(0.3),
        }
    }
}

impl Material for Metal {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color {
        let si = self.s.render(hit, world, traced);
        let di = self.d.render(hit, world, traced);
        si + di
    }
    fn rays_to_trace(&self, hit: &HitInfo) -> Vec<Ray> {
        let mut r = hit.reflect();
        r.dir = (r.dir() + gen_point_in_sphere(0.1)).unit();
        vec![r]
    }
}
