use std::any::Any;

use crate::intersections::{Intersection, Intersections};
use crate::linear::{Matrix, Tuple};
use crate::{Material, Ray};

use super::WorldObject;

#[derive(Clone, Debug)]
pub struct Sphere {
    material: Material,
    transform: Matrix,
}

impl Sphere {
    pub fn with_material(&self, material: Material) -> Self {
        Self {
            material,
            transform: self.transform.clone(),
        }
    }
}

impl WorldObject for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_trait(&self) -> &dyn WorldObject {
        self as &dyn WorldObject
    }

    /// Intersect a ray with the sphere and return a vector containing the
    /// values of *t* when the ray hits the sphere.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to determine intersections for.
    fn intersect(&self, ray: &Ray) -> Intersections {
        // Create an object-space (as opposed to world-space) copy of the ray so
        // that the sphere's transform impacts the operation.
        let object_ray = ray.transformed(&self.transform.inverted());

        let sphere_to_ray = object_ray.origin() - Tuple::new_point(0, 0, 0);

        let a = object_ray.direction().dot(object_ray.direction());
        let b = 2.0 * object_ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        // A discriminant less than zero indicates the ray misses the sphere
        // entirely.
        if discriminant < 0.0 {
            return Intersections::default();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);

        Intersections::new(vec![i1, i2])
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        let object_point = &self.transform.inverted() * *point;
        let object_normal = object_point - Tuple::new_point(0, 0, 0);

        let world_normal = &self.transform.inverted().transposed() * object_normal;
        let world_normal = Tuple::new(world_normal.x(), world_normal.y(), world_normal.z(), 0.0);

        world_normal.normalized()
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    /// Apply a new transformation matrix to the sphere.
    ///
    /// # Arguments
    ///
    /// * `transform` - The sphere's new transformation matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Matrix;
    /// # use raytracer::objects::{Sphere, WorldObject};
    /// let mut s = Sphere::default();
    /// let t = Matrix::translation(2, 3, 4);
    ///
    /// s.set_transform(t.clone());
    ///
    /// assert_eq!(*s.transform(), t);
    /// ```
    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transform: Matrix::identity_4(),
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, _: &Sphere) -> bool {
        true
    }
}
