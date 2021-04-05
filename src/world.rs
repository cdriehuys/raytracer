use crate::{
    intersections::{IntersectionInfo, Intersections},
    lights::PointLight,
    linear::{Matrix, Tuple},
    objects::{Sphere, WorldObject},
    Color, Material, Ray,
};

lazy_static! {
    // Note: The objects that make up the default world are relied on by the
    // tests. Changing their values will break test cases.

    pub static ref DEFAULT_LIGHT: PointLight = PointLight::new(Tuple::new_point(-10, 10, -10), Color::new(1, 1, 1));

    pub static ref DEFAULT_SPHERE_1: Sphere = Sphere::default().with_material(
        Material::default()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2),
    );
    pub static ref DEFAULT_SPHERE_2: Sphere = {
        let mut sphere = Sphere::default();
        sphere.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        sphere
    };
}

#[derive(Debug)]
pub struct World<'a> {
    pub objects: Vec<&'a dyn WorldObject>,
    pub light: Option<&'a PointLight>,
}

impl<'a> World<'a> {
    /// Create an empty world with no light source.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::World;
    /// let world = World::new();
    ///
    /// assert_eq!(world.objects.len(), 0);
    /// assert!(world.light.is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    /// Compute the color of the world by casting a ray into it.
    ///
    /// If the ray does not hit any of the objects in the world, the color
    /// defaults to black.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to cast into the world.
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = intersections.hit();

        match hit {
            Some(intersection) => {
                let info = intersection.prepare_info(ray);

                self.shade_hit(&info)
            }
            None => Color::new(0, 0, 0),
        }
    }

    /// Find the world objects hit by a given ray.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to cast into the world.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::default();

        for object in self.objects.iter() {
            intersections.add_intersections(object.intersect(ray));
        }

        intersections
    }

    /// Compute the lighting for a specific intersection.
    ///
    /// # Arguments
    ///
    /// * `hit_info` - Information about the intersection to shade.
    pub fn shade_hit(&self, hit_info: &IntersectionInfo) -> Color {
        let light = match self.light {
            Some(light) => *light,
            None => PointLight::new(Tuple::new_point(0, 0, 0), Color::new(0, 0, 0)),
        };

        hit_info.object().material().light(
            &light,
            &hit_info.point(),
            &hit_info.eye_vec(),
            &hit_info.normal_vec(),
        )
    }
}

impl<'a> Default for World<'a> {
    /// Create the default world with two concentric spheres.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::World;
    /// let w = World::default();
    ///
    /// assert!(w.light.is_some());
    /// assert_eq!(w.objects.len(), 2);
    /// ```
    fn default() -> Self {
        Self {
            light: Some(&DEFAULT_LIGHT),
            objects: vec![DEFAULT_SPHERE_1.as_trait(), DEFAULT_SPHERE_2.as_trait()],
        }
    }
}
