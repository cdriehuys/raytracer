mod sphere;

use std::{any::Any, fmt::Debug};

pub use sphere::Sphere;

use crate::intersections::Intersections;
use crate::linear::{Matrix, Tuple};
use crate::{Material, Ray};

pub trait WorldObject: Debug {
    /// Allow for downcasting to obtain the original concrete struct.
    fn as_any(&self) -> &dyn Any;

    /// Determine if and where a ray intersects the object.
    fn intersect(&self, ray: &Ray) -> Intersections;

    /// Get the object's material.
    fn material(&self) -> &Material;

    /// Compute the normal vector at a specific point on the object's surface.
    fn normal_at(&self, point: &Tuple) -> Tuple;

    /// Retrieve the object's transformation matrix.
    fn transform(&self) -> &Matrix;

    /// Set the object's transformation matrix.
    fn set_transform(&mut self, transform: Matrix);
}

impl PartialEq for dyn WorldObject {
    fn eq(&self, other: &Self) -> bool {
        self.transform() == other.transform()
    }
}
