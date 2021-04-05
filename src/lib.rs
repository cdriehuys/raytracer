#[macro_use]
extern crate lazy_static;

pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod linear;
pub mod objects;

mod colors;
mod materials;
mod rays;
mod world;

pub use colors::Color;
pub use materials::Material;
pub use rays::Ray;
pub use world::{World, DEFAULT_LIGHT, DEFAULT_SPHERE_1, DEFAULT_SPHERE_2};
