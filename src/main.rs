#![feature(box_syntax)]

#[macro_use]
extern crate approx;
extern crate image;

use std::rc::Rc;

use image::ImageBuffer;

use light::PointLight;
use objects::Cube;
use objects::Sphere;
use objects::World;
use ray::Camera;
use trace::trace;
use utils::vec3_to_rgb;
use utils::Vec3;

#[macro_use]
mod utils;
mod light;
mod objects;
mod ray;
mod trace;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

fn main() {
    let mut world = World::empty();
    world.add_obj(Rc::new(Sphere::new(vec3!(0, 0, 0), 1.)));
    world.add_obj(Rc::new(Sphere::new(vec3!(1, 0, 0), 0.3)));
    world.add_obj(Rc::new(Cube::new(
        vec3!(0, 0, 0),
        vec3!(1, 0, 0),
        vec3!(0, 1, 0),
        10.,
    )));
    world.add_light(Rc::new(PointLight::new(Vec3::new(3., 0., 0.))));

    let camera = Camera::new(Vec3::new(3.0, 0.0, 0.), Vec3::new(0., 0., 0.));
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    for (w, h, ray) in camera.emit_ray(WIDTH, HEIGHT) {
        let pixel = vec3_to_rgb(trace(&ray, &world, 1));
        img.put_pixel(w, h, pixel);
    }
    img.save("test.jpg").unwrap();
}
