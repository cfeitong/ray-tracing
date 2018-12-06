use image::{Pixel, Rgb};

use ray::HitPoint;
use std::borrow::Borrow;
use utils::Vec3;

pub trait LightSource {
    /// light intensity in [0, 1]
    fn intensity(&self, point: Vec3) -> f32;
    fn position(&self) -> Vec3;
    fn color(&self) -> Rgb<u8>;
}

pub struct PointLight {
    pos: Vec3,
    light_color: Rgb<u8>,
}

impl LightSource for PointLight {
    fn intensity(&self, point: Vec3) -> f32 {
        1. / (self.pos - point).len2()
    }

    fn position(&self) -> Vec3 {
        self.pos
    }

    fn color(&self) -> Rgb<u8> {
        self.light_color
    }
}

impl PointLight {
    pub fn new(pos: Vec3) -> Self {
        PointLight {
            pos,
            light_color: *Rgb::from_slice(&[255, 0, 0]),
        }
    }
}

pub fn render<Hit, Light>(point: &Hit, light: &Light) -> Rgb<u8>
where
    Hit: Borrow<HitPoint>,
    Light: Borrow<dyn LightSource>,
{
    const SHININESS: f32 = 1.;
    let point = point.borrow();
    let light = light.borrow();
    let rate1 = 1.;
    let rate2 = point
        .out_dir()
        .dot((light.position() - point.position()).normalize());
    let mut rate = rate1 * rate2.powf(SHININESS);
    rate = min!(rate, 1.);
    rate = max!(rate, 0.);
    let c = light.color();
    *Rgb::from_slice(&[
        (c[0] as f32 * rate) as u8,
        (c[1] as f32 * rate) as u8,
        (c[2] as f32 * rate) as u8,
    ])
}
