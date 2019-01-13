#[macro_use]
extern crate cft_ray_tracer as raytracer;

use image::{ImageBuffer, Pixel, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use raytracer::{
    light, material,
    object::{Sphere, World},
    Camera, Color,
};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const SAMPLE_RATE: f32 = 100.;

fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255.99 * max!(0., min!(1., c.x)).sqrt()) as u8;
    let g = (255.99 * max!(0., min!(1., c.y)).sqrt()) as u8;
    let b = (255.99 * max!(0., min!(1., c.z)).sqrt()) as u8;
    *Rgb::from_slice(&[r, g, b])
}

fn main() {
    let mut world = World::empty();
    let d = material::LambertianModel::new(1.0);
    let t = material::Dielectric::new(1.5);
    let m = material::Metal::new(0.3, 1.0);
    world.add_obj(
        Sphere::new((0., 0., -1000.), 1000.),
        d.with_color((0.5, 0.5, 0.5)),
    );
    let mut rng = rand::thread_rng();
    let mut rd = || rng.gen::<f32>();
    for a in -11..11 {
        for b in -11..11 {
            let center = vec3!(a as f32 + 0.9 * rd(), b as f32 + 0.9 * rd(), 0.2);
            let choose_material = rd();
            if choose_material < 0.8 {
                world.add_obj(
                    Sphere::new(center, 0.2),
                    d.with_color((rd().powi(2), rd().powi(2), rd().powi(2))),
                );
            } else if choose_material < 0.95 {
                world.add_obj(
                    Sphere::new(center, 0.2),
                    m.with_color(((1. + rd()) / 2., (1. + rd()) / 2., (1. + rd()) / 2.))
                        .with_fuzz(rd() / 2.),
                );
            } else {
                world.add_obj(Sphere::new(center, 0.2), t);
            }
        }
    }
    world.add_obj(Sphere::new((0., 0., 1.), 1.), t);
    world.add_obj(
        Sphere::new((-4., 0., 1.), 1.),
        d.with_color((0.4, 0.2, 0.1)),
    );
    world.add_obj(
        Sphere::new((4., 0., 1.), 1.),
        m.with_color((0.7, 0.6, 0.5)).with_fuzz(0.),
    );
    world.add_light(light::SkyLight);

    let camera = Camera::new((13., -3., 2.), (0., 0., 0.))
        .with_focus_dist(10.)
        .with_aperture(0.1)
        .with_fov(20.)
        .with_aspect(WIDTH as f32 / HEIGHT as f32)
        .with_sample_rate(SAMPLE_RATE);

    let mut raw = vec![(vec3!(0, 0, 0), 0); (WIDTH * HEIGHT) as usize];
    let rays = camera.emit_rays(WIDTH, HEIGHT);
    let bar = ProgressBar::new(rays.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    let pixels: Vec<_> = rays
        .into_par_iter()
        .map(|(w, h, ray)| {
            let result = (w, h, world.trace(&ray, 10));
            let bar = bar.clone();
            bar.tick();
            result
        })
        .collect();
    bar.finish_with_message("trace finished");
    for (w, h, p) in pixels {
        raw[(h * WIDTH + w) as usize].0 += p;
        raw[(h * WIDTH + w) as usize].1 += 1;
    }

    let raw: Vec<_> = raw
        .into_par_iter()
        .map(|(pixel, count)| vec3_to_rgb(pixel / (count as f32)))
        .collect();
    let img = ImageBuffer::from_fn(WIDTH, HEIGHT, |w, h| raw[(h * WIDTH + w) as usize]);
    img.save("examples/scene.jpg").unwrap();
}
