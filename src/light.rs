use image::{Pixel, Rgb};

use objects::World;
use ray::HitPoint;
use utils::Vec3;

pub trait LightSource {
    fn render(&self, point: &HitPoint) -> Rgb<u8>;
}

pub struct PointLight {
    pos: Vec3,
    color: Rgb<u8>,
}

impl LightSource for PointLight {
    fn render(&self, hit: &HitPoint) -> Rgb<u8> {
        let theta = hit.angle(hit.position() - self.pos);
        let red = (self.color[0] as f32 * theta.cos()) as u8;
        let green = (self.color[1] as f32 * theta.cos()) as u8;
        let blue = (self.color[2] as f32 * theta.cos()) as u8;
        *Rgb::from_slice(&[red, green, blue])
    }
}

impl PointLight {
    pub fn new(pos: Vec3) -> Self {
        PointLight {
            pos,
            color: *Rgb::from_slice(&[128, 0, 0]),
        }
    }
}
