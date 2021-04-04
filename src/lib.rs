pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod linear;
pub mod objects;

mod colors;
mod materials;
mod rays;

pub use colors::Color;
pub use materials::Material;
pub use rays::Ray;
