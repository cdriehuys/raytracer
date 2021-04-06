use float_cmp::approx_eq;
use std::{fmt::Debug, ops};

use crate::{linear::Tuple, objects::WorldObject, Ray};

/// A representation of a ray's intersection with a world object.
#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a dyn WorldObject,
}

impl<'a> Intersection<'a> {
    /// Construct a new intersection.
    ///
    /// # Arguments
    ///
    /// * `t` - The time when the intersection occurred.
    /// * `object` - The object that was intersected.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::intersections::Intersection;
    /// # use raytracer::objects::{Sphere, WorldObject};
    /// let sphere = Sphere::default();
    ///
    /// let intersection = Intersection::new(3.5, &sphere);
    ///
    /// assert_eq!(intersection.t(), 3.5);
    /// assert_eq!(intersection.object(), sphere.as_trait());
    /// ```
    pub fn new(t: f64, object: &'a dyn WorldObject) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a dyn WorldObject {
        self.object
    }

    /// Precompute information about an intersection.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray used in the intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::{
    ///     intersections::Intersection,
    ///     linear::Tuple,
    ///     objects::{Sphere, WorldObject},
    ///     Ray,
    /// };
    /// let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    /// let shape = Sphere::default();
    /// let i = Intersection::new(4.0, &shape);
    ///
    /// let info = i.prepare_info(&r);
    ///
    /// assert_eq!(info.t(), 4.0);
    /// assert_eq!(info.object(), shape.as_trait());
    /// assert_eq!(info.point(), Tuple::new_point(0, 0, -1));
    /// assert_eq!(info.eye_vec(), Tuple::new_vector(0, 0, -1));
    /// assert_eq!(info.normal_vec(), Tuple::new_vector(0, 0, -1));
    /// ```
    pub fn prepare_info(&self, ray: &Ray) -> IntersectionInfo {
        let point = ray.position_at(self.t);
        let eye_vec = -ray.direction();
        let mut normal_vec = self.object.normal_at(&point);

        // The normal vector always points to the outside of the shape. If the
        // hit comes from inside the shape, the eye vector and normal vector
        // will point in opposite directions, causing the surface to be far
        // darker than it should be. If the hit comes from the inside, we have
        // to invert the normal vector to get the correct shading.
        let inside = normal_vec.dot(eye_vec) < 0.0;
        if inside {
            normal_vec = -normal_vec;
        }

        // Compute the offset point used for shadow calculations.
        let over_point = point + normal_vec * 1e-5;

        IntersectionInfo {
            t: self.t,
            object: self.object,
            point,
            over_point,
            eye_vec,
            normal_vec,
            inside,
        }
    }
}

impl<'a, 'b> PartialEq<Intersection<'b>> for Intersection<'a> {
    fn eq(&self, other: &Intersection<'b>) -> bool {
        approx_eq!(f64, self.t, other.t) && self.object == other.object
    }
}

/// Additional information about an intersection that is useful when performing
/// lighting calculations.
#[derive(Debug)]
pub struct IntersectionInfo<'a> {
    t: f64,
    object: &'a dyn WorldObject,
    point: Tuple,
    over_point: Tuple,
    eye_vec: Tuple,
    inside: bool,
    normal_vec: Tuple,
}

impl<'a> IntersectionInfo<'a> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a dyn WorldObject {
        &*self.object
    }

    pub fn point(&self) -> Tuple {
        self.point
    }

    /// Get a point slightly offset from the computed intersection point in the
    /// direction of the normal vector.
    ///
    /// This is necessary because when we try to determine if the intersection
    /// is shadowed, floating point inaccuracies can cause the surface of the
    /// shape to shadow itself. If we use this slightly offset point instead,
    /// the problem is eliminated.
    pub fn over_point(&self) -> Tuple {
        self.over_point
    }

    pub fn eye_vec(&self) -> Tuple {
        self.eye_vec
    }

    pub fn normal_vec(&self) -> Tuple {
        self.normal_vec
    }

    pub fn inside(&self) -> bool {
        self.inside
    }
}

/// A collection of intersections.
#[derive(Debug, Default)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    /// Create a new collection of intersections.
    ///
    /// The provided intersections will be sorted by their `t` value.
    ///
    /// # Arguments
    ///
    /// * `intersections` - A vector of the intersections to store.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::intersections::{Intersection, Intersections};
    /// # use raytracer::objects::Sphere;
    /// let sphere = Sphere::default();
    /// let i1 = Intersection::new(1.0, &sphere);
    /// let i2 = Intersection::new(2.0, &sphere);
    ///
    /// let intersections = Intersections::new(vec![i1, i2]);
    ///
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), 1.0);
    /// assert_eq!(intersections[1].t(), 2.0);
    /// ```
    pub fn new(mut intersections: Vec<Intersection<'a>>) -> Self {
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Self { intersections }
    }

    /// Add a group of intersections to the collection.
    ///
    /// The collection's order is maintained.
    ///
    /// # Arguments
    ///
    /// * `intersections` - The intersections to add to the collection.
    pub fn add_intersections(&mut self, intersections: Intersections<'a>) {
        for intersection in intersections.intersections.into_iter() {
            match self
                .intersections
                .binary_search_by(|a| a.t.partial_cmp(&intersection.t).unwrap())
            {
                Ok(_) => (),
                Err(pos) => self.intersections.insert(pos, intersection),
            }
        }
    }

    /// Determine which intersection from the collection is the first to be hit.
    ///
    /// This is always the intersection with the lowest non-negative `t` value.
    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }

    /// Find the number of elements in the intersection collection.
    pub fn len(&self) -> usize {
        self.intersections.len()
    }
}

impl<'a> ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}
