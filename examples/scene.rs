#[macro_use]
extern crate cft_ray_tracer as raytracer;
extern crate image;

use image::{ImageBuffer, Pixel, Rgb};

use raytracer::{
    Camera,
    Color,
    light::SkyLight,
    material::{Diffuse, Metal, Transparent}, object::{Sphere, Square, World}, Vec3,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SAMPLE_RATE: f32 = 20.;

fn vec3_to_rgb(c: Color) -> Rgb<u8> {
    let r = (255.99 * max!(0., min!(1., c.x))) as u8;
    let g = (255.99 * max!(0., min!(1., c.y))) as u8;
    let b = (255.99 * max!(0., min!(1., c.z))) as u8;
    *Rgb::from_slice(&[r, g, b])
}

fn main() {
    let mut world = World::empty();
    let m1 = Diffuse::new();
    let m2 = m1.with_diffuse(0.3);
    let m3 = Transparent::new(0.0, 0.8);
    world.add_obj(Sphere::new((-0.55, 0., 0.5), 0.5), Metal::new());
    world.add_obj(
        Sphere::new((0.55, 0., 0.5), 0.5),
        m2.with_color((0.6, 0.2, 0.4)),
    );
    world.add_obj(Sphere::new((0., 1., 0.3), 0.2), m3);
    world.add_obj(
        Square::new(vec3!(0, 0, 0), vec3!(1, 0, 0), vec3!(0, 1, 0), 5.),
        m1.with_diffuse(0.6),
    );
    //    world.add_light(ParallelLight::new(vec3!(0, 0, -1)));
    world.add_light(SkyLight);

    let camera =
        Camera::new(Vec3::new(-0.0, 2., 1.5), Vec3::new(0., 0., 0.)).with_sample_rate(SAMPLE_RATE);
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
