#[macro_use]
extern crate cft_ray_tracer as raytracer;

use image::{ImageBuffer, Pixel, Rgb};
use rayon::prelude::*;

use raytracer::{
    light, material,
    object::{Cube, Sphere, Square, World},
    Camera, Color, Vec3,
};
use std::sync::Mutex;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const SAMPLE_RATE: f32 = 5.;

fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255.99 * max!(0., min!(1., c.x))) as u8;
    let g = (255.99 * max!(0., min!(1., c.y))) as u8;
    let b = (255.99 * max!(0., min!(1., c.z))) as u8;
    *Rgb::from_slice(&[r, g, b])
}

fn main() {
    let mut world = World::empty();
    let d = material::LambertianModel::new(0.8);

    world.add_obj(Cube::new((0., 0., 0.), (1., 0., 0.), (0., 1., 0.), 2.), d);
    world.add_light(light::LightShape::new(Square::new(
        (0., 0., 0.99),
        (1., 0., 0.),
        (0., -1., 0.),
        0.9,
    )));

    let camera =
        Camera::new(Vec3::new(0.8, 0.0, 0.0), Vec3::new(0., 0., 0.0)).with_sample_rate(SAMPLE_RATE);
    let raw = Mutex::new(vec![(vec3!(0, 0, 0), 0); (WIDTH * HEIGHT) as usize]);
    camera
        .emit_rays(WIDTH, HEIGHT)
        .into_par_iter()
        .map(|(w, h, ray)| (w, h, world.trace(&ray, 10)))
        .for_each(|(w, h, p)| {
            let mut raw = raw.lock().expect("fail to lock raw array");
            raw[(h * WIDTH + w) as usize].0 += p;
            raw[(h * WIDTH + w) as usize].1 += 1;
        });
    let raw: Vec<_> = raw
        .into_inner()
        .unwrap()
        .into_iter()
        .map(|(pixel, count)| vec3_to_rgb(pixel / (count as f32)))
        .collect();
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |w, h| raw[(h * WIDTH + w) as usize]);
    img.save("test.jpg").unwrap();
}
