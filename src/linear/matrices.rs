use std::{fmt::Debug, ops};

use float_cmp::approx_eq;

/// An algebraic matrix that can store floating point numbers.
#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    columns: usize,

    data: Vec<Vec<f64>>,
}

impl Matrix {
    /// Construct a new 2x2 matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Matrix;
    /// let m = Matrix::square_2(
    ///     1.0, 2.0,
    ///     3.0, 4.0,
    /// );
    /// ```
    pub fn square_2(v1: f64, v2: f64, v3: f64, v4: f64) -> Self {
        Self {
            rows: 2,
            columns: 2,
            data: vec![vec![v1, v2], vec![v3, v4]],
        }
    }

    /// Construct a new 3x3 matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Matrix;
    /// let m = Matrix::square_3(
    ///     1.0, 2.0, 3.0,
    ///     4.0, 5.0, 6.0,
    ///     7.0, 8.0, 9.0,
    /// );
    /// ```
    pub fn square_3(
        v1: f64,
        v2: f64,
        v3: f64,
        v4: f64,
        v5: f64,
        v6: f64,
        v7: f64,
        v8: f64,
        v9: f64,
    ) -> Self {
        Self {
            rows: 3,
            columns: 3,
            data: vec![vec![v1, v2, v3], vec![v4, v5, v6], vec![v7, v8, v9]],
        }
    }

    /// Construct a new 4x4 matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::linear::Matrix;
    /// let m = Matrix::square_4(
    ///     1.0, 2.0, 3.0, 4.0,
    ///     5.0, 6.0, 7.0, 8.0,
    ///     9.0, 8.0, 7.0, 6.0,
    ///     5.0, 4.0, 3.0, 2.0,
    /// );
    /// ```
    pub fn square_4(
        v1: f64,
        v2: f64,
        v3: f64,
        v4: f64,
        v5: f64,
        v6: f64,
        v7: f64,
        v8: f64,
        v9: f64,
        v10: f64,
        v11: f64,
        v12: f64,
        v13: f64,
        v14: f64,
        v15: f64,
        v16: f64,
    ) -> Self {
        Self {
            rows: 3,
            columns: 3,
            data: vec![
                vec![v1, v2, v3, v4],
                vec![v5, v6, v7, v8],
                vec![v9, v10, v11, v12],
                vec![v13, v14, v15, v16],
            ],
        }
    }
}

impl PartialEq for Matrix {
    /// Determine matrix equality by ensuring that the matrices have the same
    /// dimensions and that the values in each cell are approximately equal.
    fn eq(&self, rhs: &Self) -> bool {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return false;
        }

        for row in 0..self.rows {
            for col in 0..self.columns {
                if !approx_eq!(f64, self[row][col], rhs[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}
