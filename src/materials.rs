use float_cmp::approx_eq;

use crate::{lights::PointLight, linear::Tuple, Color};

/// A material describes how a surface looks using ambient, diffuse, and
/// specular reflections.
#[derive(Clone, Copy, Debug)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn with_color(&self, color: Color) -> Self {
        Material { color, ..*self }
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn with_ambient(&self, ambient: f64) -> Self {
        Self { ambient, ..*self }
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn with_diffuse(&self, diffuse: f64) -> Self {
        Self { diffuse, ..*self }
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn with_specular(&self, specular: f64) -> Self {
        Self { specular, ..*self }
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    /// Calculate the color of the material based on where the material is
    /// observed from and what lighting sources are present.
    ///
    /// # Arguments
    ///
    /// * `light` - The light source.
    /// * `position` - The material's position.
    /// * `eye_v` - The vector pointing from the illuminated point towards the
    ///   "eye" or camera.
    /// * `normal_v` - The normal vector of the surface being lit.
    /// * `in_shadow` - A boolean indicating if the point does not have direct
    ///   line of sight to the light source.
    pub fn light(
        &self,
        light: &PointLight,
        position: &Tuple,
        eye_v: &Tuple,
        normal_v: &Tuple,
        in_shadow: bool,
    ) -> Color {
        // Combine surface color with light color.
        let effective_color = self.color * light.intensity();

        // The ambient contribution only depends on the effective color.
        let ambient = effective_color * self.ambient;

        // If the point is shadowed, the ambient reflection is the only part
        // that matters since the diffuse and specular reflections depend on a
        // direct light source.
        if in_shadow {
            return ambient;
        }

        // Get a vector from the point being lit towards the light source.
        let light_v = (light.position() - *position).normalized();

        let diffuse: Color;
        let specular: Color;

        // A negative value for the dot product indicates the light is on the
        // other side of the surface, so we would get no diffuse or specular
        // reflections.
        let light_dot_normal = light_v.dot(*normal_v);
        if light_dot_normal < 0.0 {
            diffuse = Color::new(0, 0, 0);
            specular = Color::new(0, 0, 0);
        } else {
            // If the light is on the same side of the surface as the observer,
            // we always get a diffuse reflection that depends only on the
            // light position and normal vector.
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // Compute the dot product of the vector that the reflected light
            // takes and the vector towards the observer. If the value is
            // positive, it indicates that the observer will see some specular
            // reflection.
            let reflect_v = (-light_v).reflected_over(normal_v);
            let reflect_dot_eye = reflect_v.dot(*eye_v);
            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0, 0, 0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    /// Construct the default material.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::{Color, Material};
    /// let m = Material::default();
    ///
    /// assert_eq!(m.color(), Color::new(1, 1, 1));
    /// assert_eq!(m.ambient(), 0.1);
    /// assert_eq!(m.diffuse(), 0.9);
    /// assert_eq!(m.specular(), 0.9);
    /// assert_eq!(m.shininess(), 200.0);
    /// ```
    fn default() -> Self {
        Self {
            color: Color::new(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && approx_eq!(f64, self.ambient, other.ambient)
            && approx_eq!(f64, self.diffuse, other.diffuse)
            && approx_eq!(f64, self.specular, other.specular)
            && approx_eq!(f64, self.shininess, other.shininess)
    }
}
