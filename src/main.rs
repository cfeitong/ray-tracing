#![feature(box_syntax)]

#[macro_use]
extern crate approx;
extern crate image;
extern crate rand;

use std::rc::Rc;

use image::{ImageBuffer, Rgb};

use light::PointLight;
use objects::{Cube, Sphere, World};
use ray::Camera;
use trace::trace;
use utils::{vec3_to_rgb, Vec3};

#[macro_use]
mod utils;
mod light;
mod objects;
mod ray;
mod trace;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const SAMPLE_RATE: f32 = 5.;

fn main() {
    let mut world = World::empty();
    world.add_obj(Rc::new(Sphere::new(vec3!(0, 0, 0), 1.)));
    //    world.add_obj(Rc::new(Sphere::new(vec3!(1, 0, 0), 0.3)));
    world.add_obj(Rc::new(Cube::new(
        vec3!(0, 0, 0),
        vec3!(1, 0, 0),
        vec3!(0, 1, 0),
        10.,
    )));
    world.add_light(Rc::new(PointLight::new(Vec3::new(3., 0., 0.))));

    let camera =
        Camera::new(Vec3::new(3.0, 0.0, 0.), Vec3::new(0., 0., 0.)).with_sample_rate(SAMPLE_RATE);
    let mut raw = vec![(vec3!(0, 0, 0), 0); (WIDTH * HEIGHT) as usize];
    for (w, h, ray) in camera.emit_rays(WIDTH, HEIGHT) {
        let pixel = trace(&ray, &world, 1);
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
