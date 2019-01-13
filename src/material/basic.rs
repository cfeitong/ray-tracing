use crate::{
    light::LightInfo,
    object::World,
    ray::{HitInfo, Ray},
    util::{Color, Vec3},
};

use super::Material;

#[derive(Clone, Copy)]
pub struct PhongModel {
    shininess: f64,
    diffuse: f64,
    color: Color,
}

impl PhongModel {
    pub fn new() -> Self {
        PhongModel {
            shininess: 1.,
            diffuse: 0.5,
            color: (1., 1., 1.).into(),
        }
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_diffuse(mut self, kd: f64) -> Self {
        self.diffuse = kd;
        self
    }

    pub fn with_color<T: Into<Vec3>>(mut self, color: T) -> Self {
        self.color = color.into();
        self
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }
}

impl Default for PhongModel {
    fn default() -> Self {
        Self::new()
    }
}

impl Material for PhongModel {
    fn render(&self, hit: &HitInfo, world: &World, _traced: &[Color]) -> Color {
        let c = world
            .lights
            .iter()
            .map(|light| {
                if let Some(c) = light.looked(&hit.reflect(), world) {
                    return c;
                }
                let info = LightInfo::new(light.as_ref(), hit, world);
                let ratio1 = 1.;
                let ratio2 = hit.dir_out().dot(-info.dir());
                let mut ratio = ratio1 * ratio2.powf(self.shininess);
                ratio = min!(ratio, 1.);
                ratio = max!(ratio, 0.);

                // specular illumination
                let si = ratio;

                // diffuse illumination
                let di = max!(hit.normal().dot(-info.dir()), 0.);

                // ambient illumination
                let ai = 0.1;

                // light intensity
                let li = info.intensity() * info.color();

                // total intensity = specular + diffuse + ambient
                if info.is_in_shadow() {
                    ai * li
                } else {
                    (si * 0.5 + di * 0.5 + ai) * li
                }
            })
            .sum::<Vec3>();
        let kd = self.diffuse();
        kd * c * self.color
    }
    fn scatter(&self, _hit: &HitInfo) -> Vec<Ray> {
        Vec::new()
    }
}

#[derive(Clone, Copy)]
pub struct Specular {
    albedo: f64,
}

impl Specular {
    pub fn new(albedo: f64) -> Specular {
        Specular { albedo }
    }

    pub fn with_albedo(mut self, albedo: f64) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn albedo(self) -> f64 {
        self.albedo
    }
}

impl Material for Specular {
    fn render(&self, _hit: &HitInfo, _world: &World, traced: &[Color]) -> Color {
        self.albedo * traced[0]
    }
}

#[derive(Clone, Copy)]
pub struct Transparent {
    opacity: f64,
    ior: f64,
    color: Color,
}

impl Transparent {
    pub fn ior(&self) -> f64 {
        self.ior
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }
}

impl Transparent {
    pub fn new(opacity: f64, ior: f64) -> Self {
        Transparent {
            opacity,
            ior,
            color: (1., 1., 1.).into(),
        }
    }

    pub fn with_ior(mut self, ior: f64) -> Self {
        self.ior = ior;
        self
    }

    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_color<T: Into<Color>>(mut self, color: T) -> Self {
        self.color = color.into();
        self
    }
}

impl Material for Transparent {
    fn render(&self, _hit: &HitInfo, _world: &World, traced: &[Color]) -> Color {
        self.color * (1. - self.opacity) * traced[0]
    }

    fn scatter(&self, hit: &HitInfo) -> Vec<Ray> {
        let n = if hit.is_to_outward() {
            self.ior
        } else {
            1. / self.ior
        };
        // Total reflection
        hit.refract(n)
            .map(|ray| vec![ray])
            .unwrap_or(vec![hit.reflect()])
    }
}
