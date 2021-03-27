use std::ops;

/// Represent a color as an RGB tuple.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// Construct a new color.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    pub fn new<R: Into<f64>, G: Into<f64>, B: Into<f64>>(r: R, g: G, b: B) -> Self {
        Color {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }

    /// Retrieve the color's red component.
    pub fn red(&self) -> f64 {
        self.r
    }

    /// Retrieve the color's green component.
    pub fn green(&self) -> f64 {
        self.g
    }

    /// Retrieve the color's blue component.
    pub fn blue(&self) -> f64 {
        self.b
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        float_cmp::approx_eq!(f64, self.r, rhs.r)
            && float_cmp::approx_eq!(f64, self.g, rhs.g)
            && float_cmp::approx_eq!(f64, self.b, rhs.b)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}
