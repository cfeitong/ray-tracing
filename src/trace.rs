use crate::object::World;
use crate::ray::Ray;
use crate::util::Color;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    ray.hit(world)
        .map(|hit| {
            let obj = &hit.obj;
            let traced = trace(&hit.out_ray(), world, depth - 1);
            obj.material.render(&hit.info, world, traced)
        })
        .unwrap_or(Color::new(0., 0., 0.))
}
