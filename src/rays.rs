use crate::linear::Tuple;

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
    /// # use raytracer::rays::Ray;
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
    /// # use raytracer::rays::Ray;
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
}
