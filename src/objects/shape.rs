use crate::{
    intersections::Intersections,
    linear::{Matrix, Tuple},
    Material, Ray,
};

use super::BaseShape;

pub trait Shape: std::fmt::Debug {
    /// Retrieve the object's base shape.
    ///
    /// This is used to provide default implementations for most of the shape's
    /// behavior. It can be replaced with [`unimplemented!`] if a shape wishes
    /// to provide custom implementations for everything.
    fn base_shape(&self) -> &BaseShape;

    /// Get a mutable reference to the object's base shape.
    ///
    /// This is used to provide default implementations for most of the shape's
    /// behavior. It can be replaced with [`unimplemented!`] if a shape wishes
    /// to provide custom implementations for everything.
    fn base_shape_mut(&mut self) -> &mut BaseShape;

    /// Find the intersections between the object and a ray in object space.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to intersect with the object. It is treated as being
    ///   in object space (as opposed to world space).
    fn intersect_local(&self, ray: &Ray) -> Intersections;

    /// Find the normal vector at an object space location on the object's
    /// surface.
    ///
    /// # Arguments
    ///
    /// * `point` - The point on the object's surface (in object space) that the
    ///   normal vector should be found for. This point MUST be on the object's
    ///   surface to get a good result.
    fn normal_at_local(&self, point: &Tuple) -> Tuple;

    ////////////////////////////////////////////////////////////////////////////
    // The following methods have default implementations that rely on the    //
    // above methods.                                                         //
    //                                                                        //
    // See `test_shape.rs` for tests exercising these default                 //
    // implementations.                                                       //
    ////////////////////////////////////////////////////////////////////////////

    /// Retrieve the shape's material.
    fn material(&self) -> &Material {
        self.base_shape().material()
    }

    /// Set the shape's material.
    fn set_material(&mut self, material: Material) {
        self.base_shape_mut().set_material(material);
    }

    /// Retrieve the object's unique ID.
    fn object_id(&self) -> usize {
        self.base_shape().object_id()
    }

    /// Retrieve the shape's transform.
    fn transform(&self) -> &Matrix {
        self.base_shape().transform()
    }

    /// Set the shape's transform.
    fn set_transform(&mut self, transform: Matrix) {
        self.base_shape_mut().set_transform(transform);
    }

    /// Find the intersections between the object and a specific ray.
    ///
    /// The ray is assumed to be in world space. By default, this method
    /// converts the ray to object space and passes it to
    /// [`intersect_local`][Self::intersect_local].
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to intersect with the object.
    fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transformed(&self.transform().inverted());

        self.intersect_local(&local_ray)
    }

    /// Find the normal vector at a point on the object's surface.
    ///
    /// By default, the point is converted into object space, the normal is
    /// found using [`normal_at_local`][Self::normal_at_local], and then the
    /// normal is converted back into world space.
    ///
    /// # Arguments
    ///
    /// * `point` - The point on the object's surface to find the normal vector
    ///   at. This MUST be a point on the object's surface. The result will not
    ///   be correct, and is not well defined, for any other points.
    fn normal_at(&self, point: &Tuple) -> Tuple {
        let inverted_transform = self.transform().inverted();

        let local_point = &inverted_transform * *point;
        let local_normal = self.normal_at_local(&local_point);
        let world_normal = &inverted_transform.transposed() * local_normal;

        // Since the proper calculations involve taking the submatrix containing
        // only the x, y, and z components of the transform, we have to zero out
        // w here. The simplest way of doing that is by constructing a new
        // vector.
        Tuple::new_vector(world_normal.x(), world_normal.y(), world_normal.z()).normalized()
    }
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        // We only use shape equality in test cases where we already have a
        // reference to the object we expect to see, so we can do this ID
        // comparison. This check will break down if we need to be able to
        // construct expected objects from their material/transform.
        self.object_id() == other.object_id()
    }
}
