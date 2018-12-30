#![feature(uniform_paths)]

#[macro_use]
extern crate approx;

pub use light::LightSource;
pub use material::Material;
pub use object::{RcObjectExt, Shape};
pub use ray::{Camera, Ray};
pub use util::{Color, Vec3};

#[macro_use]
pub mod util;
pub mod light;
pub mod material;
pub mod object;
pub mod ray;
