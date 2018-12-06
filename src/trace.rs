use image::{Pixel, Rgb};

use objects::World;
use ray::Ray;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Rgb<u8> {
    if depth == 0 {
        return *Rgb::from_slice(&[0, 0, 0]);
    }
    let result = ray.hit(world).map(|point| {
        let r = trace(&point.reflect_ray(), world, depth - 1);
        let mut c: Rgb<u8> = world.lights
            .iter()
            .map(|light| {
                point.obj.color(&point, light.as_ref())
            })
            .fold(*Rgb::from_slice(&[0, 0, 0]), |mut acc, color| {
                acc.blend(&color);
                acc
            });
        c.apply2(&r, |c1, c2| {
            c1.saturating_add((point.obj.decay() * (c2 as f32)) as u8)
        });
        c
    });

    result.unwrap_or(*Rgb::from_slice(&[0, 0, 0]))
}
