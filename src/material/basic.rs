use crate::{
    object::World,
    ray::{HitInfo, Ray},
    util::{Color, Vec3},
};
use crate::light::LightInfo;

use super::Material;

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
                let info = LightInfo::new(light.as_ref(), hit, world);
                let ratio1 = 1.;
                let ratio2 = hit.out_dir().dot(-info.dir());
                let mut rate = ratio1 * ratio2.powf(self.shininess);
                rate = min!(rate, 1.);
                rate = max!(rate, 0.);

                // specular illumination
                let si = rate;

                // diffuse illumination
                let di = max!(hit.normal().dot(-info.dir()), 0.);

                // ambient illumination
                let ai = 0.1;

                // light intensity
                let li = info.intensity() * info.color();

                // total intensity = specular + diffuse + ambient
                let ti = if info.is_in_shadow() {
                    ai * li
                } else {
                    (si * 0.5 + di * 0.5 + ai) * li
                };

                ti
            })
            .sum::<Vec3>();
        let kd = self.diffuse();
        kd * c * self.color
    }
    fn rays_to_trace(&self, _hit: &HitInfo) -> Vec<Ray> {
        Vec::new()
    }
}

#[derive(Clone, Copy)]
pub struct Specular {
    albedo: f32,
}

impl Specular {
    pub fn new(albedo: f32) -> Specular {
        Specular { albedo }
    }

    pub fn with_albedo(mut self, albedo: f32) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn albedo(&self) -> f32 {
        self.albedo
    }
}

impl Material for Specular {
    fn render(&self, _hit: &HitInfo, _world: &World, traced: &[Color]) -> Color {
        self.albedo * traced[0]
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
