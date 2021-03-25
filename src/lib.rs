use float_cmp::approx_eq;
use std::ops;

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
    pub fn new_point<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.0,
        }
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
    pub fn new_vector<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.0,
        }
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

impl ops::Add for Tuple {
    type Output = Self;

    /// Add another tuple to the current one.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The tuple to add.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let point = Tuple::new_point(3, -2, 5);
    /// let vector = Tuple::new_vector(-2, 3, 1);
    ///
    /// let want = Tuple::new_point(1, 1, 6);
    ///
    /// assert_eq!(point + vector, want);
    /// ```
    fn add(self, rhs: Self) -> Self {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T: Into<f64>> ops::Div<T> for Tuple {
    type Output = Self;

    /// Shrink all of a tuple's components by the same scalar factor.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The factor to shrink each component by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let vector = Tuple::new_vector(1, -2, 3);
    /// let scale = 2;
    ///
    /// let want = Tuple::new_vector(0.5, -1.0, 1.5);
    ///
    /// assert_eq!(vector / scale, want);
    /// ```
    fn div(self, rhs: T) -> Self {
        let scalar = rhs.into();

        Tuple{x: self.x / scalar, y: self.y / scalar, z: self.z / scalar, w: self.w / scalar }
    }
}

impl<T: Into<f64>> ops::Mul<T> for Tuple {
    type Output = Self;

    /// Scale all of a tuple's components by the same scalar.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The amount to scale each component by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let vector = Tuple::new_vector(1, 2, 3);
    /// let scale = 2;
    ///
    /// let want = Tuple::new_vector(2, 4, 6);
    ///
    /// assert_eq!(vector * scale, want);
    /// ```
    fn mul(self, rhs: T) -> Self {
        let scalar = rhs.into();

        Tuple{x: self.x * scalar, y: self.y * scalar, z: self.z * scalar, w: self.w * scalar}
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    /// Negate a tuple. This only makes sense for vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let a = Tuple::new_vector(1, -2, 3);
    /// let b = Tuple::new_vector(-1, 2, -3);
    ///
    /// assert_eq!(-a, b);
    /// ```
    fn neg(self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    /// Subtract another tuple from the current tuple.
    ///
    /// # Arguments
    ///
    /// `rhs` - The tuple to subtract from the current one.
    ///
    /// # Examples
    ///
    /// Subtracting two points gives the vector between them:
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let p1 = Tuple::new_point(3, 2, 1);
    /// let p2 = Tuple::new_point(5, 6, 7);
    ///
    /// let want = Tuple::new_vector(-2, -4, -6);
    ///
    /// assert_eq!(p1 - p2, want);
    /// ```
    ///
    /// Subtracting a vector from a point gives the result of moving backwards
    /// by the given vector from the starting point:
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let point = Tuple::new_point(3, 2, 1);
    /// let vector = Tuple::new_vector(5, 6, 7);
    ///
    /// let want = Tuple::new_point(-2, -4, -6);
    ///
    /// assert_eq!(point - vector, want);
    /// ```
    ///
    /// Subtracting a vector from a vector gives the difference in direction
    /// between the two:
    ///
    /// ```
    /// # use raytracer::Tuple;
    /// let v1 = Tuple::new_vector(3, 2, 1);
    /// let v2 = Tuple::new_vector(5, 6, 7);
    ///
    /// let want = Tuple::new_vector(-2, -4, -6);
    ///
    /// assert_eq!(v1 - v2, want);
    /// ```
    fn sub(self, rhs: Self) -> Self {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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
mod tests {
    use super::*;

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
        let want = Tuple {
            x: 1.2,
            y: -3.4,
            z: 5.6,
            w: 1.0,
        };

        let got = Tuple::new_point(want.x, want.y, want.z);

        assert_eq!(got, want);
    }

    #[test]
    fn vector_creation() {
        let want = Tuple {
            x: 1.2,
            y: -3.4,
            z: 5.6,
            w: 0.0,
        };

        let got = Tuple::new_vector(want.x, want.y, want.z);

        assert_eq!(got, want);
    }

    #[test]
    fn tuple_negation() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let b = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };

        assert_eq!(-a, b);
    }

    #[test]
    fn tuple_multiply_by_scalar() {
        let tuple = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let scalar = 3.5;

        let want = Tuple {x: 3.5, y: -7.0, z: 10.5, w: -14.0};

        assert_eq!(tuple * scalar, want);
    }

    #[test]
    fn tuple_multiply_by_fractional_scalar() {
        let tuple = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let scalar = 0.5;

        let want = Tuple {x: 0.5, y: -1.0, z: 1.5, w: -2.0};

        assert_eq!(tuple * scalar, want);
    }

    #[test]
    fn tuple_divide_by_scalar() {
        let tuple = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let scalar = 2;

        let want = Tuple {x: 0.5, y: -1.0, z: 1.5, w: -2.0};

        assert_eq!(tuple / scalar, want);
    }
}
