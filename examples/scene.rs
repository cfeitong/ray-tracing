#[macro_use]
extern crate cft_ray_tracer as raytracer;
extern crate image;

use image::{ImageBuffer, Pixel, Rgb};

use raytracer::{
    light,
    material,
    object::{
        Cube,
        Sphere,
        Square,
        World
    },
    Camera, Color, Vec3,
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
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
    for (w, h, ray) in camera.emit_rays(WIDTH, HEIGHT) {
        let pixel = world.trace(&ray, 10);
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
