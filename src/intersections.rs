use crate::objects::WorldObject;
use std::{fmt::Debug, ops};

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
    /// let intersection = Intersection::new(3.5, Box::new(sphere));
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

#[derive(Debug, Default)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    /// Create a new collection of intersections.
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
    /// let i1 = Intersection::new(1.0, Box::new(sphere));
    /// let i2 = Intersection::new(2.0, Box::new(sphere));
    ///
    /// let intersections = Intersections::new(vec![i1, i2]);
    ///
    /// assert_eq!(intersections.len(), 2);
    /// assert_eq!(intersections[0].t(), 1.0);
    /// assert_eq!(intersections[1].t(), 2.0);
    /// ```
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self { intersections }
    }

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
