use light::render;
use objects::World;
use ray::Ray;
use utils::Color;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    let result = ray.hit(world).map(|point| {
        let r = trace(&point.reflected_ray(), world, depth - 1);
        let c = world
            .lights
            .iter()
            .filter(|&light| {
                let mut ray = Ray::new(light.position(), point.position() - light.position());
                ray.hit(world)
                    .map(|hit| relative_eq!(hit.position(), point.position()))
                    .unwrap_or(true)
            })
            .map(|light| render(&point, light))
            .enumerate()
            .fold(Color::new(0., 0., 0.), |acc, (i, color)| {
                let i = i as f32;
                (acc * i + color) / (i + 1.)
            });
        let decay = point.object().decay();
        c * decay + r * (1. - decay)
    });
    result.unwrap_or(Color::new(0., 0., 0.))
}
