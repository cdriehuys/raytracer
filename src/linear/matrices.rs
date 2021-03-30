use std::{fmt::Debug, ops};

use float_cmp::approx_eq;

use crate::Tuple;

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
            rows: 4,
            columns: 4,
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

impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        assert_eq!(
            self.columns, rhs.rows,
            "Cannot multiply a matrix with {} column(s) by a matrix with {} row(s)",
            self.columns, self.rows
        );

        // The resulting matrix has the same number of rows as the LHS and the
        // same number of columns as the RHS, which gives us our bounds for
        // iteration.

        let mut new_rows = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            let mut new_cols = Vec::with_capacity(rhs.columns);

            for col in 0..rhs.columns {
                let mut new_value = 0.0;

                // We could iterate up to the LHS' rows or the RHS' columns
                // since they are the same. This step computes the dot product
                // of tuples given by the LHS' current row and the RHS' current
                // column.
                for index in 0..self.rows {
                    new_value += self[row][index] * rhs[index][col];
                }

                new_cols.push(new_value);
            }

            new_rows.push(new_cols);
        }

        Self {
            rows: self.rows,
            columns: rhs.columns,
            data: new_rows,
        }
    }
}

// Since our Tuple implementation is essentially a 4x1 matrix, we can multiply
// them as a matrix operation.
impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        assert_eq!(
            self.columns, 4,
            "Cannot multiply a {} column matrix with a 4 row tuple",
            self.columns,
        );

        Tuple::new(
            self[0][0] * rhs.x()
                + self[0][1] * rhs.y()
                + self[0][2] * rhs.z()
                + self[0][3] * rhs.w(),
            self[1][0] * rhs.x()
                + self[1][1] * rhs.y()
                + self[1][2] * rhs.z()
                + self[1][3] * rhs.w(),
            self[2][0] * rhs.x()
                + self[2][1] * rhs.y()
                + self[2][2] * rhs.z()
                + self[2][3] * rhs.w(),
            self[3][0] * rhs.x()
                + self[3][1] * rhs.y()
                + self[3][2] * rhs.z()
                + self[3][3] * rhs.w(),
        )
    }
}
