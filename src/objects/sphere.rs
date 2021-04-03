use std::any::Any;

use crate::intersections::{Intersection, Intersections};
use crate::linear::Tuple;
use crate::rays::Ray;

use super::WorldObject;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere {}

impl WorldObject for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Intersect a ray with the sphere and return a vector containing the
    /// values of *t* when the ray hits the sphere.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to determine intersections for.
    fn intersect(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin() - Tuple::new_point(0, 0, 0);

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        // A discriminant less than zero indicates the ray misses the sphere
        // entirely.
        if discriminant < 0.0 {
            return Intersections::default();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, Box::new(*self));
        let i2 = Intersection::new(t2, Box::new(*self));

        Intersections::new(vec![i1, i2])
    }
}

impl PartialEq for Sphere {
    fn eq(&self, _: &Sphere) -> bool {
        true
    }
}
