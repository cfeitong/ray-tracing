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
}

impl Diffuse {
    pub fn new() -> Self {
        Diffuse {
            shininess: 1.,
            reflection_param: 0.5,
            diffuse_param: 0.5,
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
        world
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

                let kr = self.reflection_param();
                let kd = self.diffuse_param();

                // light intensity
                let li = light.intensity(hit.hit_point) * light.color();

                // total intensity = specular + diffuse + ambient
                let ti = if light.is_in_shadow(hit.hit_point, world) {
                    ai * li
                } else {
                    (si * 0.5 + di * 0.5 + ai) * li
                };

                kr * traced + kd * ti
            })
            .sum()
    }
}

#[derive(Clone, Copy)]
pub struct Specular {
    reflection_param: f32,
}

impl Specular {
    pub fn new(reflection_rate: f32) -> Specular {
        Specular {
            reflection_param: reflection_rate,
        }
    }

    pub fn reflection_param(&self) -> f32 {
        self.reflection_param
    }

    pub fn absorption_rate(&self) -> f32 {
        1. - self.reflection_param
    }
}

impl Material for Specular {
    fn render(&self, hit: &HitInfo, world: &World, traced: Color) -> Color {
        self.reflection_param * traced
    }
}

pub struct Transparent {}
//
//pub fn render<Record, Light>(point: &Record, light: &Light) -> Color
//{
//    const SHININESS: f32 = 2.;
//    let point = point.borrow();
//    let pos = point.position();
//    let light = light.borrow();
//
//    let rate1 = 1.;
//    let rate2 = point.out_dir().dot(-light.dir_at(pos));
//    let mut rate = rate1 * rate2.powf(SHININESS);
//    rate = min!(rate, 1.);
//    rate = max!(rate, 0.);
//
//    let specular_illumination = rate;
//
//    let diffuse_illumination = max!(point.normal().dot(-light.dir_at(pos)), 0.);
//
//    let ambient_illumination = 0.1;
//
//    (specular_illumination * 0.5 + diffuse_illumination * 0.5 + ambient_illumination)
//        * light.intensity(pos)
//        * light.color()
//}
//
//pub fn render_by_normal<Hit, Light>(point: &Hit, _light: &Light) -> Color
//    where
//        Hit: Borrow<HitRecord>,
//        Light: Borrow<dyn LightSource>,
//{
//    let n = point.borrow().normal();
//    let n = vec3!(n.y, n.z, n.x);
//    (n + 1.) / 2.
//}
