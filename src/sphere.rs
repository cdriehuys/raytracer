use crate::linear::Tuple;
use crate::rays::Ray;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere {}

impl Sphere {
    /// Intersect a ray with the sphere and return a vector containing the
    /// values of *t* when the ray hits the sphere.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to determine intersections for.
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin() - Tuple::new_point(0, 0, 0);

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        // A discriminant less than zero indicates the ray misses the sphere
        // entirely.
        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![t1, t2]
    }
}
