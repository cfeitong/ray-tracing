use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::Color,
};

pub use self::{
    basic::*,
    compose::*
};

mod basic;
mod compose;

pub trait Material {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color;
    fn rays_to_trace(&self, hit: &HitInfo) -> Vec<Ray> {
        vec![hit.reflect()]
    }
}
