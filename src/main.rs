#![feature(uniform_paths)]

#[macro_use]
extern crate approx;

use image::{ImageBuffer, Rgb};

use light::ParallelLight;
use material::Diffuse;
use object::{Cube, Sphere, Square, World};
use ray::Camera;
use trace::trace;
use util::{Vec3, vec3_to_rgb};

#[macro_use]
mod util;
mod light;
mod material;
mod object;
mod ray;
mod trace;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SAMPLE_RATE: f32 = 20.;

fn main() {
    let mut world = World::empty();
    let m = Diffuse::new();
    let m2 = m.with_diffuse_param(0.3).with_reflection_param(0.75);
    world.add_obj(
        Sphere::new(vec3!(-0.55, 0., 0.5), 0.5),
        m2.with_color((0.3, 0.8, 0.2)),
    );
    world.add_obj(
        Sphere::new(vec3!(0.55, 0., 0.5), 0.5),
        m2.with_color((0.6, 0.2, 0.4)),
    );
    world.add_obj(
        Square::new(vec3!(0, 0, 0), vec3!(1, 0, 0), vec3!(0, 1, 0), 5.),
        m.with_diffuse_param(0.6).with_reflection_param(0.05),
    );
    world.add_light(ParallelLight::new(vec3!(0, 0, -1)));

    let camera =
        Camera::new(Vec3::new(-0.5, 2., 2.), Vec3::new(0., 0., 0.)).with_sample_rate(SAMPLE_RATE);
    let mut raw = vec![(vec3!(0, 0, 0), 0); (WIDTH * HEIGHT) as usize];
    for (w, h, ray) in camera.emit_rays(WIDTH, HEIGHT) {
        let pixel = trace(&ray, &world, 10);
        raw[(h * WIDTH + w) as usize].0 += pixel;
        raw[(h * WIDTH + w) as usize].1 += 1;
    }
    let raw: Vec<_> = raw
        .into_iter()
        .map(|(pixel, count)| vec3_to_rgb(pixel / (count as f32)))
        .collect();
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |w, h| raw[(h * WIDTH + w) as usize]);
    img.save("test.jpg").unwrap();
}
