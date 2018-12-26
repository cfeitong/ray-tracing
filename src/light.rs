use std::borrow::Borrow;

use objects::World;
use ray::HitRecord;
use ray::Ray;
use utils::Color;
use utils::EPS;
use utils::Vec3;

pub trait LightSource {
    /// light intensity in [0, 1]
    fn intensity(&self, point: Vec3) -> f32;
    fn dir_at(&self, point: Vec3) -> Vec3;
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool;
    fn color(&self) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub struct ParallelLight {
    dir: Vec3,
    light_color: Color,
}

impl ParallelLight {
    pub fn new(dir: Vec3) -> ParallelLight {
        ParallelLight {
            dir,
            light_color: vec3!(1, 1, 1),
        }
    }

    pub fn with_color(mut self, color: Color) -> ParallelLight {
        self.light_color = color;
        self
    }
}

impl LightSource for ParallelLight {
    fn intensity(&self, _point: Vec3) -> f32 {
        1.
    }
    fn dir_at(&self, _point: Vec3) -> Vec3 {
        self.dir
    }
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool {
        let dir = -self.dir_at(point);
        let ray = Ray::new(point, dir);
        ray.hit(world).is_some()
    }
    fn color(&self) -> Color {
        self.light_color
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pos: Vec3,
    light_color: Color,
}

// TODO: add intensity decay with distance
impl LightSource for PointLight {
    fn intensity(&self, point: Vec3) -> f32 {
        1. / (self.pos - point).len2()
    }

    fn dir_at(&self, point: Vec3) -> Vec3 {
        (point - self.pos).normalize()
    }
    fn is_in_shadow(&self, point: Vec3, world: &World) -> bool {
        let dir = -self.dir_at(point);
        let ray = Ray::new(point, dir);
        ray.hit(world)
            .map(|hit| {
                let l1 = (point - hit.position()).len2();
                let l2 = (point - self.pos).len2();
                l1 + EPS < l2
            })
            .unwrap_or(false)
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
    Hit: Borrow<HitRecord>,
    Light: Borrow<dyn LightSource>,
{
    const SHININESS: f32 = 2.;
    let point = point.borrow();
    let pos = point.position();
    let light = light.borrow();

    let rate1 = 1.;
    let rate2 = point.out_dir().dot(-light.dir_at(pos));
    let mut rate = rate1 * rate2.powf(SHININESS);
    rate = min!(rate, 1.);
    rate = max!(rate, 0.);

    let specular_illumination = rate;

    let diffuse_illumination = max!(point.normal().dot(-light.dir_at(pos)), 0.);

    let ambient_illumination = 0.1;

    (specular_illumination * 0.5 + diffuse_illumination * 0.5 + ambient_illumination)
        * light.intensity(pos)
        * light.color()
}

pub fn render_by_normal<Hit, Light>(point: &Hit, _light: &Light) -> Color
where
    Hit: Borrow<HitRecord>,
    Light: Borrow<dyn LightSource>,
{
    let n = point.borrow().normal();
    let n = vec3!(n.y, n.z, n.x);
    (n + 1.) / 2.
}
