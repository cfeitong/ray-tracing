use image::{Pixel, Rgb};
use objects::World;
use ray::{Ray, Reflectable};
use std::cmp::Ordering;
use utils::Vec3;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Rgb<u8> {
    if depth == 0 {
        return *Rgb::from_slice(&[0, 0, 0]);
    }
    let result = ray.hit(world).map(|point| {
        let reflected_ray = point.reflect_ray();
        let r = trace(&reflected_ray, world, depth - 1);
        let mut c = point.obj.color(&reflected_ray);
        c.apply2(&r, |c1, c2| {
            c1.saturating_add((point.obj.decay() * (c2 as f32)) as u8)
        });
        c
    });

    result.unwrap_or(*Rgb::from_slice(&[0, 0, 0]))
}
