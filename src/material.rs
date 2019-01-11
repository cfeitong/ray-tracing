use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::Color,
};

pub use self::{basic::*, compose::*};

mod basic;
mod compose;

pub trait Material: Sync + Send {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color;
    fn scatter(&self, hit: &HitInfo) -> Vec<Ray> {
        vec![hit.reflect()]
    }
}
