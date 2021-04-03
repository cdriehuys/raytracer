use crate::linear::{Matrix, Tuple};

/// A ray represents a ray of light travelling through a scene. It has a
/// starting location as well as a direction. Casting a ray through a scene and
/// determining what it hits allows the ray tracer to determine what a specific
/// loccation in the scene looks like.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    /// Create a new ray.
    ///
    /// # Arguments
    ///
    /// `origin` - The location that the ray originates from.
    /// `direction` - The direction the ray travels in.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Tuple;
    /// # use raytracer::Ray;
    /// let origin = Tuple::new_point(1, 2, 3);
    /// let direction = Tuple::new_vector(4, 5, 6);
    ///
    /// let ray = Ray::new(origin, direction);
    ///
    /// assert_eq!(ray.origin(), origin);
    /// assert_eq!(ray.direction(), direction);
    /// ```
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    /// Get the ray's origin.
    pub fn origin(&self) -> Tuple {
        self.origin
    }

    /// Get the ray's direction.
    pub fn direction(&self) -> Tuple {
        self.direction
    }

    /// Get a ray's position at a specific moment in time.
    ///
    /// # Arguments
    ///
    /// * `t` - The moment in time to retrieve the ray's position at.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Tuple;
    /// # use raytracer::Ray;
    /// let ray = Ray::new(Tuple::new_point(2, 3, 4), Tuple::new_vector(1, 0, 0));
    ///
    /// assert_eq!(ray.position_at(0.0), Tuple::new_point(2, 3, 4));
    /// assert_eq!(ray.position_at(1.0), Tuple::new_point(3, 3, 4));
    /// assert_eq!(ray.position_at(-1.0), Tuple::new_point(1, 3, 4));
    /// assert_eq!(ray.position_at(2.5), Tuple::new_point(4.5, 3.0, 4.0));
    /// ```
    pub fn position_at(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    /// Apply a transformation to a ray.
    ///
    /// # Arguments
    ///
    /// * `transform` - The transformation to apply.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::{Matrix, Tuple};
    /// # use raytracer::Ray;
    /// let origin = Tuple::new_point(1, 2, 3);
    /// let direction = Tuple::new_vector(0, 1, 0);
    /// let ray = Ray::new(origin, direction);
    ///
    /// // Translation only affects the origin.
    /// let transform = Matrix::translation(3, 4, 5);
    /// let r2 = ray.transformed(&transform);
    ///
    /// assert_eq!(r2.origin(), Tuple::new_point(4, 6, 8));
    /// assert_eq!(r2.direction(), direction);
    ///
    /// // Scaling affects the origin and direction.
    /// let transform = Matrix::scaling(2, 3, 4);
    /// let r3 = ray.transformed(&transform);
    ///
    /// assert_eq!(r3.origin(), Tuple::new_point(2, 6, 12));
    /// assert_eq!(r3.direction(), Tuple::new_vector(0, 3, 0));
    /// ```
    pub fn transformed(&self, transform: &Matrix) -> Self {
        Self {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}
