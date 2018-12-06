use std::rc::Rc;

use light::LightSource;
use objects::World;
use std::fmt::Debug;
use utils::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn hit(&self, world: &World) -> Option<HitPoint> {
        world
            .objects
            .iter()
            .filter_map(|obj| {
                obj.reflect(self).map(|out_ray| HitPoint {
                    obj: obj.clone(),
                    out_ray,
                    norm: self.dir.mid_vec(out_ray.dir),
                })
            })
            .min_by(|a, b| {
                let dist_a = self.pos.distance(a.position());
                let dist_b = self.pos.distance(b.position());
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }
}

pub trait Reflectable: Debug {
    fn reflect(&self, ray: &Ray) -> Option<Ray>;
    fn decay(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    up: Vec3,
    sight: Vec3,
}

impl Camera {
    /// adjust this camera to look at `point`.
    pub fn look(&mut self, point: Vec3) {
        self.sight = (point - self.pos).normalize();
        let right = self.right();
        self.up = right.cross(self.sight).normalize();
    }

    /// return up direction of this camera.
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// return right direction of this camera.
    pub fn right(&self) -> Vec3 {
        self.sight.cross(self.up).normalize()
    }

    /// emit needed ray through a 1 unit away square screen whose size is 2 unit.
    pub fn emit_ray(&self, width: u32, height: u32) -> Vec<(u32, u32, Ray)> {
        (0..width)
            .flat_map(|w| (0..height).map(move |h| (w, h)))
            .map(|(w, h)| {
                // f32 width and height
                let (fw, fh) = (w as f32, h as f32);
                // f32 window width and window height
                let (ww, wh) = (width as f32, height as f32);
                let (fw, fh) = (2. * fw / ww, 2. * fh / wh);
                let left_up_corner = self.pos + self.sight + self.up() - self.right();
                let point = left_up_corner - self.up() * fh + self.right() * fw;
                let ray = Ray {
                    pos: self.pos,
                    dir: (point - self.pos).normalize(),
                };
                (w, h, ray)
            })
            .collect()
    }

    /// create a camera which is at `pos` and look at `point`.
    pub fn new(pos: Vec3, point: Vec3) -> Camera {
        let mut camera = Camera {
            pos,
            up: Vec3::new(0., 0., 1.),
            sight: Vec3::new(0., 0., 1.),
        };
        camera.look(point);
        camera
    }
}

#[derive(Clone)]
pub struct HitPoint {
    out_ray: Ray,
    obj: Rc<dyn Reflectable>,
    norm: Vec3,
}

impl HitPoint {
    pub fn angle(&self) -> f32 {
        self.out_ray.dir.dot(self.norm).acos()
    }

    pub fn out_dir(&self) -> Vec3 {
        self.out_ray.dir.normalize()
    }

    pub fn in_dir(&self) -> Vec3 {
        let out_dir = self.out_ray.dir.normalize();
        let h = out_dir - out_dir.proj_to(self.norm);
        out_dir.proj_to(self.norm) - h
    }

    pub fn object(&self) -> Rc<dyn Reflectable> {
        self.obj.clone()
    }

    pub fn normal(&self) -> Vec3 {
        self.norm.normalize()
    }

    pub fn reflected_ray(&self) -> Ray {
        self.out_ray
    }

    pub fn position(&self) -> Vec3 {
        self.out_ray.pos
    }
}
