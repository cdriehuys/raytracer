mod sphere;

use std::{any::Any, fmt::Debug};

pub use sphere::Sphere;

use crate::{intersections::Intersections, rays::Ray};

pub trait WorldObject: Debug {
    /// Allow for downcasting to obtain the original concrete struct.
    fn as_any(&self) -> &dyn Any;

    /// Determine if and where a ray intersects the object.
    fn intersect(&self, ray: &Ray) -> Intersections;
}

impl PartialEq for &dyn WorldObject {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
