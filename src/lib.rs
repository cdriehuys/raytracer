use float_cmp::approx_eq;

/// A tuple represents a point or vector.
#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    // A value used to distinguish between points (1.0) and vectors (0.0).
    w: f64,
}

impl Tuple {
    /// Construct a new tuple that represents a point in space.
    ///
    /// # Arguments
    ///
    /// * `x` - The location of the point on the x-axis.
    /// * `y` - The location of the point on the y-axis.
    /// * `z` - The location of the point on the z-axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let point = Tuple::new_point(1.0, 2.0, 3.0);
    /// ```
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self{x: x, y: y, z: z, w: 1.0}
    }

    /// Construct a new tuple that represents a vector.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-component of the vector.
    /// * `y` - The y-component of the vector.
    /// * `z` - The z-component of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let vector = Tuple::new_vector(1.0, 2.0, 3.0);
    /// ```
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self{x: x, y: y, z: z, w: 0.0}
    }

    /// Determine if a tuple represents a point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let point = Tuple::new_point(1.0, 2.0, 3.0);
    /// let vector = Tuple::new_vector(1.0, 2.0, 3.0);
    ///
    /// assert!(point.is_point());
    /// assert!(!vector.is_point());
    /// ```
    pub fn is_point(&self) -> bool {
        // We use exact equality because a point is always constructed with the
        // literal 1.0 which can be exactly represented, so there is no chance
        // for floating point rounding to affect this operation.
        self.w == 1.0
    }

    /// Determine if a tuple represents a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let vector = Tuple::new_vector(1.0, 2.0, 3.0);
    /// let point = Tuple::new_point(1.0, 2.0, 3.0);
    ///
    /// assert!(vector.is_vector());
    /// assert!(!point.is_vector());
    /// ```
    pub fn is_vector(&self) -> bool {
        // We use exact equality because a vector is always constructed with the
        // literal 0.0 which can be exactly represented, so there is no chance
        // for floating point rounding to affect this operation.
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, ulps = 2)
            && approx_eq!(f64, self.y, other.y, ulps = 2)
            && approx_eq!(f64, self.z, other.z, ulps = 2)
            && approx_eq!(f64, self.w, other.w, ulps = 2)
    }
}

#[cfg(test)]
mod test_tuple {
    use super::*;

    #[test]
    fn is_point() {
        let point = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);

        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn is_vector() {
        let vector = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);

        assert!(vector.is_vector());
        assert!(!vector.is_point());
    }

    #[test]
    fn equal_mismatched_x() {
        let t1 = Tuple {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        let t2 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        assert_ne!(t1, t2);
    }

    #[test]
    fn equal_mismatched_y() {
        let t1 = Tuple {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        };
        let t2 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        assert_ne!(t1, t2);
    }

    #[test]
    fn equal_mismatched_z() {
        let t1 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        };
        let t2 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        assert_ne!(t1, t2);
    }

    #[test]
    fn equal_mismatched_w() {
        let t1 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        assert_ne!(t1, t2);
    }

    #[test]
    fn equal_same_values() {
        let t1 = Tuple {
            x: 1.2,
            y: -3.4,
            z: 5.6,
            w: 1.0,
        };
        let t2 = Tuple {
            x: 1.2,
            y: -3.4,
            z: 5.6,
            w: 1.0,
        };

        assert_eq!(t1, t2);
    }

    #[test]
    fn equal_same_values_floating_points() {
        let t1 = Tuple {
            x: 0.15 + 0.15 + 0.15,
            y: 0.15 + 0.15 + 0.15,
            z: 0.15 + 0.15 + 0.15,
            w: 0.15 + 0.15 + 0.15,
        };
        let t2 = Tuple {
            x: 0.1 + 0.1 + 0.25,
            y: 0.1 + 0.1 + 0.25,
            z: 0.1 + 0.1 + 0.25,
            w: 0.1 + 0.1 + 0.25,
        };

        assert_eq!(t1, t2);
    }

    #[test]
    fn point_creation() {
        let want = Tuple{x: 1.2, y: -3.4, z: 5.6, w: 1.0};

        let got = Tuple::new_point(want.x, want.y, want.z);

        assert_eq!(got, want);
    }

    #[test]
    fn vector_creation() {
        let want = Tuple{x: 1.2, y: -3.4, z: 5.6, w: 0.0};

        let got = Tuple::new_vector(want.x, want.y, want.z);

        assert_eq!(got, want);
    }
}
