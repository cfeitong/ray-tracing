use std::borrow::Borrow;

use ray::HitPoint;
use utils::Color;
use utils::Vec3;

pub trait LightSource {
    /// light intensity in [0, 1]
    fn intensity(&self, point: Vec3) -> f32;
    fn position(&self) -> Vec3;
    fn color(&self) -> Color;
}

pub struct PointLight {
    pos: Vec3,
    light_color: Color
}

impl LightSource for PointLight {
    fn intensity(&self, point: Vec3) -> f32 {
        1. / (self.pos - point).len2()
    }

    fn position(&self) -> Vec3 {
        self.pos
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

pub fn render<Hit, Light>(point: &Hit, light: &Light) -> Color
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
    light.color() * rate
}
