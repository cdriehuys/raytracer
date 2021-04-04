use crate::{
    intersections::Intersections,
    lights::PointLight,
    linear::{Matrix, Tuple},
    objects::{Sphere, WorldObject},
    Color, Material, Ray,
};

#[derive(Debug)]
pub struct World {
    objects: Vec<Box<dyn WorldObject>>,
    light: Option<PointLight>,
}

impl World {
    /// Create an empty world with no light source.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::World;
    /// let world = World::new();
    ///
    /// assert_eq!(world.objects().len(), 0);
    ///
    /// if let Some(light) = world.light() {
    ///     panic!("Unexpected light source: {:?}", light);
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn objects(&self) -> &Vec<Box<dyn WorldObject>> {
        &self.objects
    }

    pub fn light(&self) -> Option<PointLight> {
        self.light
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::default();

        for object in self.objects.iter() {
            intersections.add_intersections(object.intersect(ray));
        }

        intersections
    }
}

impl Default for World {
    /// Create the default world with two concentric spheres.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::lights::PointLight;
    /// # use raytracer::linear::{Matrix, Tuple};
    /// # use raytracer::objects::{Sphere, WorldObject};
    /// # use raytracer::{Color, Material, World};
    /// let light = PointLight::new(Tuple::new_point(-10, 10, -10), Color::new(1, 1, 1));
    ///
    /// let w = World::default();
    ///
    /// assert_eq!(w.light().unwrap(), light);
    /// assert_eq!(w.objects().len(), 2);
    /// ```
    fn default() -> Self {
        // Note: The objects in the default world are important, and there are
        // tests that rely on their exact values.

        let light = PointLight::new(Tuple::new_point(-10, 10, -10), Color::new(1, 1, 1));

        let s1 = Sphere::default().with_material(
            Material::default()
                .with_color(Color::new(0.8, 1.0, 0.6))
                .with_diffuse(0.7)
                .with_specular(0.2),
        );

        let s2 = {
            let mut sphere = Sphere::default();
            sphere.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

            sphere
        };

        Self {
            light: Some(light),
            objects: vec![Box::new(s1), Box::new(s2)],
        }
    }
}
