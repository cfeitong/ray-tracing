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
        let mut c = world.lights.iter().map(|light| render(&point, light)).fold(
            Color::new(0., 0., 0.),
            |acc, color| {
                (acc + color) / 2.
            },
        );
        c * point.object().decay()
    });

    result.unwrap_or(vec3!(0, 0, 0))
}
