use float_cmp::approx_eq;

use crate::objects::WorldObject;
use std::{fmt::Debug, ops};

/// A representation of a ray's intersection with a world object.
#[derive(Debug)]
pub struct Intersection {
    t: f64,
    object: Box<dyn WorldObject>,
}

impl Intersection {
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
    /// # use raytracer::objects::Sphere;
    /// let sphere = Sphere::default();
    ///
    /// let intersection = Intersection::new(3.5, Box::new(sphere.clone()));
    ///
    /// assert_eq!(intersection.t(), 3.5);
    ///
    /// // It's possible to obtain the original object, but painful. The methods
    /// // from WorldObject should be comprehensive enough that downcasting is
    /// // only needed in tests.
    /// assert_eq!(
    ///     *(intersection.object().as_any().downcast_ref::<Sphere>().unwrap()),
    ///     sphere,
    /// );
    /// ```
    pub fn new(t: f64, object: Box<dyn WorldObject>) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &dyn WorldObject {
        &*self.object
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        approx_eq!(f64, self.t, other.t) && &*self.object == &*other.object
    }
}

/// A collection of intersections.
#[derive(Debug, Default)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
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
    /// let i1 = Intersection::new(1.0, Box::new(sphere.clone()));
    /// let i2 = Intersection::new(2.0, Box::new(sphere.clone()));
    ///
    /// let intersections = Intersections::new(vec![i1, i2]);
    ///
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), 1.0);
    /// assert_eq!(intersections[1].t(), 2.0);
    /// ```
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
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
    pub fn add_intersections(&mut self, intersections: Intersections) {
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

impl ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}
