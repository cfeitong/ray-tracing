use std::borrow::Borrow;
use std::rc::Rc;

use light::LightSource;
use object::World;
use ray::HitInfo;
use ray::HitRecord;
use trace::trace;
use util::Color;
use util::Vec3;

pub trait Material {
    fn render(&self, hit: &HitInfo, world: &World, traced: Color) -> Color;
}

#[derive(Clone, Copy)]
pub struct Diffuse {
    shininess: f32,
    reflection_param: f32,
    diffuse_param: f32,
    color: Color,
}

impl Diffuse {
    pub fn new() -> Self {
        Diffuse {
            shininess: 1.,
            reflection_param: 0.5,
            diffuse_param: 0.5,
            color: (1., 1., 1.).into(),
        }
    }

    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_reflection_param(mut self, kr: f32) -> Self {
        self.reflection_param = kr;
        self
    }

    pub fn with_diffuse_param(mut self, kd: f32) -> Self {
        self.diffuse_param = kd;
        self
    }

    pub fn with_color<T: Into<Vec3>>(mut self, color: T) -> Self {
        self.color = color.into();
        self
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn reflection_param(&self) -> f32 {
        self.reflection_param
    }

    pub fn diffuse_param(&self) -> f32 {
        self.diffuse_param
    }
}

impl Material for Diffuse {
    fn render(&self, hit: &HitInfo, world: &World, traced: Color) -> Color {
        let c = world
            .lights
            .iter()
            .map(|light| {
                let rate1 = 1.;
                let rate2 = hit.out_dir.dot(-light.dir_at(hit.hit_point));
                let mut rate = rate1 * rate2.powf(self.shininess);
                rate = min!(rate, 1.);
                rate = max!(rate, 0.);

                let si = rate; // specular illumination

                // diffuse illumination
                let di = max!(hit.norm.dot(-light.dir_at(hit.hit_point)), 0.);

                // ambient illumination
                let ai = 0.1;

                let kd = self.diffuse_param();

                // light intensity
                let li = light.intensity(hit.hit_point) * light.color();

                // total intensity = specular + diffuse + ambient
                let ti = if light.is_in_shadow(hit.hit_point, world) {
                    ai * li
                } else {
                    (si * 0.5 + di * 0.5 + ai) * li
                };

                kd * ti
            })
            .sum::<Vec3>();
        let kr = self.reflection_param();
        (c + kr * traced) * self.color
    }
}

#[derive(Clone, Copy)]
pub struct Specular {
    reflection_param: f32,
}

impl Specular {
    pub fn new(reflection_param: f32) -> Specular {
        Specular { reflection_param }
    }

    pub fn reflection_param(&self) -> f32 {
        self.reflection_param
    }

    pub fn absorption_param(&self) -> f32 {
        1. - self.reflection_param
    }
}

impl Material for Specular {
    fn render(&self, hit: &HitInfo, world: &World, traced: Color) -> Color {
        self.reflection_param * traced
    }
}

pub struct Transparent {
    reflection_param: f32,
    refraction_param: f32,
}

impl Transparent {
    pub fn reflection_param(&self) -> f32 {
        self.reflection_param
    }

    pub fn refraction_param(&self) -> f32 {
        self.refraction_param
    }

    pub fn absorption_param(&self) -> f32 {
        1. - self.reflection_param - self.refraction_param
    }
}

//impl Material for Transparent {
//    fn render(&self, hit: &HitInfo, world: &World, traced: Color) -> Color {}
//}
