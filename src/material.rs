use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::{Color, Vec3},
};

pub trait Material {
    fn render(&self, hit: &HitInfo, world: &World, traced: &[Color]) -> Color;

    fn rays_to_trace(&self, hit: &HitInfo) -> Vec<Ray> {
        vec![hit.reflect()]
    }
}

#[derive(Clone, Copy)]
pub struct Diffuse {
    shininess: f32,
    diffuse: f32,
    color: Color,
}

impl Diffuse {
    pub fn new() -> Self {
        Diffuse {
            shininess: 1.,
            diffuse: 0.5,
            color: (1., 1., 1.).into(),
        }
    }

    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_diffuse(mut self, kd: f32) -> Self {
        self.diffuse = kd;
        self
    }

    pub fn with_color<T: Into<Vec3>>(mut self, color: T) -> Self {
        self.color = color.into();
        self
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn diffuse(&self) -> f32 {
        self.diffuse
    }
}

impl Material for Diffuse {
    fn render(&self, hit: &HitInfo, world: &World, _traced: &[Color]) -> Color {
        let c = world
            .lights
            .iter()
            .map(|light| {
                let ratio1 = 1.;
                let ratio2 = hit.out_dir.dot(-light.dir_at(hit.position()));
                let mut rate = ratio1 * ratio2.powf(self.shininess);
                rate = min!(rate, 1.);
                rate = max!(rate, 0.);

                // specular illumination
                let si = rate;

                // diffuse illumination
                let di = max!(hit.norm.dot(-light.dir_at(hit.position())), 0.);

                // ambient illumination
                let ai = 0.1;

                let kd = self.diffuse();

                // light intensity
                let li = light.intensity(hit.position()) * light.color();

                // total intensity = specular + diffuse + ambient
                let ti = if light.is_in_shadow(hit.position(), world) {
                    ai * li
                } else {
                    (si * 0.5 + di * 0.5 + ai) * li
                };

                kd * ti
            })
            .sum::<Vec3>();
        c * self.color
    }
    fn rays_to_trace(&self, _hit: &HitInfo) -> Vec<Ray> {
        Vec::new()
    }
}

#[derive(Clone, Copy)]
pub struct Specular {
    reflection: f32,
}

impl Specular {
    pub fn new(reflection: f32) -> Specular {
        Specular { reflection }
    }

    pub fn reflection(&self) -> f32 {
        self.reflection
    }
}

impl Material for Specular {
    fn render(&self, _hit: &HitInfo, _world: &World, traced: &[Color]) -> Color {
        self.reflection * traced[0]
    }
}

pub struct Transparent {
    opacity: f32,
    ior: f32,
}

impl Transparent {
    pub fn ior(&self) -> f32 {
        self.ior
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }
}

impl Transparent {
    pub fn new(opacity: f32, ior: f32) -> Self {
        Transparent { opacity, ior }
    }

    pub fn with_ior(mut self, ior: f32) -> Self {
        self.ior = ior;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

impl Material for Transparent {
    fn render(&self, _hit: &HitInfo, _world: &World, traced: &[Color]) -> Color {
        (1. - self.opacity) * traced[0]
    }
    fn rays_to_trace(&self, hit: &HitInfo) -> Vec<Ray> {
        vec![hit.refract(1. / self.ior)]
    }
}
