use crate::{linear::Tuple, Color};

/// A point light emits a specific intensity from a single point in space.
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    /// Create a new point light.
    ///
    /// # Arguments
    ///
    /// * `position` - The location of the light source.
    /// * `color` - The color emitted by the light.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::lights::PointLight;
    /// # use raytracer::linear::Tuple;
    /// # use raytracer::Color;
    /// let position = Tuple::new_point(0, 0, 0);
    /// let intensity = Color::new(1, 1, 1);
    ///
    /// let light = PointLight::new(position, intensity);
    ///
    /// assert_eq!(light.position(), position);
    /// assert_eq!(light.intensity(), intensity);
    /// ```
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn position(&self) -> Tuple {
        self.position
    }
}

impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}
