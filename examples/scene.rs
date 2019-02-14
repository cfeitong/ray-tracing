#[macro_use]
extern crate cft_ray_tracer as raytracer;

use std::sync::Arc;
use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb};
use parking_lot::Mutex;
use rand::Rng;
use threadpool::ThreadPool;

use raytracer::{
    light, material,
    object::{Sphere, World},
    Camera, Color,
};

const WIDTH: u64 = 400;
const HEIGHT: u64 = 300;
const SAMPLE_RATE: u64 = 5;
const TRACE_DEPTH: u64 = 10;

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
    let mut rd = || rng.gen::<f64>();
    for a in -11..11 {
        for b in -11..11 {
            let center = vec3!(a as f64 + 0.9 * rd(), b as f64 + 0.9 * rd(), 0.2);
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
        .with_aspect(WIDTH as f64 / HEIGHT as f64)
        .with_sample_rate(SAMPLE_RATE);

    let raw = vec![vec3!(0, 0, 0); (WIDTH * HEIGHT) as usize];

    let start = Instant::now();

    let pool = ThreadPool::new(num_cpus::get());

    let world = Arc::new(world);
    let raw = Arc::new(Mutex::new(raw));

    for vec in chunks(128, camera.emit_rays(WIDTH, HEIGHT)) {
        let world = world.clone();
        let raw = raw.clone();
        pool.execute(move || {
            for (w, h, ray) in vec {
                let p = world.trace(&ray, TRACE_DEPTH);
                raw.lock()[(h * WIDTH + w) as usize] += p;
            }
        });
    }

    pool.join();

    let duration = Instant::now().duration_since(start);
    println!(
        "total: {} seconds, {:} ns/pixel",
        duration.as_secs(),
        duration.as_nanos() / (WIDTH * HEIGHT * SAMPLE_RATE) as u128
    );

    let raw: Vec<_> = raw
        .lock()
        .iter()
        .map(|pixel| vec3_to_rgb(*pixel / SAMPLE_RATE as f64))
        .collect();
    let img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |w, h| {
        raw[(h * WIDTH as u32 + w) as usize]
    });
    img.save("test.jpg").unwrap();
}

struct Chunks<T, I: Iterator<Item=T>> {
    size: usize,
    iter: I,
}

impl<T, I: Iterator<Item=T>> Iterator for Chunks<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let storage: Vec<_> = (0..self.size).into_iter().filter_map(|_| self.iter.next()).collect();
        if storage.is_empty() {
            None
        } else {
            Some(storage)
        }
    }
}

fn chunks<T, I: Iterator<Item=T>>(size: usize, iter: I) -> Chunks<T, I> {
    Chunks {
        size,
        iter,
    }
}

fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255.99 * max!(0., min!(1., c.x)).sqrt()) as u8;
    let g = (255.99 * max!(0., min!(1., c.y)).sqrt()) as u8;
    let b = (255.99 * max!(0., min!(1., c.z)).sqrt()) as u8;
    *Rgb::from_slice(&[r, g, b])
}

