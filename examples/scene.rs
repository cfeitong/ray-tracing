#[macro_use]
extern crate cft_ray_tracer as raytracer;

use image::{ImageBuffer, Pixel, Rgb};
use rayon::prelude::*;

use raytracer::{
    Camera, Color,
    light,
    material, object::{Cube, Sphere, Square, World}, Vec3,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SAMPLE_RATE: f32 = 50.;

fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255.99 * max!(0., min!(1., c.x))) as u8;
    let g = (255.99 * max!(0., min!(1., c.y))) as u8;
    let b = (255.99 * max!(0., min!(1., c.z))) as u8;
    *Rgb::from_slice(&[r, g, b])
}

fn main() {
    let mut world = World::empty();
    let d = material::Diffuse::new(0.8);
    let t = material::Transparent::new(0.0, 1.01);
    let m = material::Specular::new(1.);
    world.add_obj(Sphere::new((0., 0., -20.), 20.), d);
    world.add_obj(Sphere::new((0., 0., 0.5), 0.5), d);
    world.add_obj(Sphere::new((1., 0., 0.5), 0.5), t);
    world.add_obj(Sphere::new((-1., 0., 0.5), 0.5), m);
    world.add_light(light::SkyLight);

    let camera = Camera::new(Vec3::new(-0.0, 3.0, 1.0), Vec3::new(0., 0., 1.0))
        .with_sample_rate(SAMPLE_RATE);
    let mut raw = vec![(vec3!(0, 0, 0), 0); (WIDTH * HEIGHT) as usize];
    let pixels: Vec<_> = camera
        .emit_rays(WIDTH, HEIGHT)
        .into_par_iter()
        .map(|(w, h, ray)| {
            (w, h, world.trace(&ray, 10))
        })
        .collect();
    for (w, h, p) in pixels {
        raw[(h * WIDTH + w) as usize].0 += p;
        raw[(h * WIDTH + w) as usize].1 += 1;
    }
    let raw: Vec<_> = raw
        .into_iter()
        .map(|(pixel, count)| vec3_to_rgb(pixel / (count as f32)))
        .collect();
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |w, h| raw[(h * WIDTH + w) as usize]);
    img.save("test.jpg").unwrap();
}
