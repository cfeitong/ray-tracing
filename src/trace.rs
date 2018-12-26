use light::render;
use objects::World;
use ray::Ray;
use utils::Color;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    ray.hit(world)
        .map(|point| {
            let c = world
                .lights
                .iter()
                .map(|light| {
                    let mut illumination = 0.3 * light.color() * light.intensity(point.position());
                    if !light.is_in_shadow(point.position(), world) {
                        illumination += render(&point, light);
                    }
                    illumination
                })
                .enumerate()
                .fold(Color::new(0., 0., 0.), |acc, (i, color)| {
                    let i = i as f32;
                    (acc * i + color) / (i + 1.)
                });
            if depth != 0 {
//                let r = trace(&point.specular_ray(), world, depth - 1);
//                let decay = point.object().decay();
//                c * decay + r * (1. - decay)
                c
            } else {
                c
            }
        })
        .unwrap_or(Color::new(0., 0., 0.))
}
