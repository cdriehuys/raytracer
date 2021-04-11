use std::cell::RefCell;

use crate::{intersections::Intersections, linear::Tuple, Ray};

use super::{BaseShape, Shape};

#[derive(Clone, Debug, Default)]
struct TestShape {
    base: BaseShape,
    last_intersected_ray: RefCell<Option<Ray>>,
}

// We expect shapes that want to use the default shape behavior to implement
// this trait.
impl Shape for TestShape {
    fn base_shape(&self) -> &BaseShape {
        &self.base
    }

    fn base_shape_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn intersect_local(&self, ray: &Ray) -> Intersections {
        self.last_intersected_ray.replace(Some(*ray));

        Intersections::default()
    }

    fn normal_at_local(&self, point: &Tuple) -> Tuple {
        // Just return the point's elements as a vector so we can pass a known
        // value to the default `normal_at` implementation in `shape.rs`.
        Tuple::new_vector(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{PI, SQRT_2};

    use super::*;

    use crate::{linear::Matrix, Material};

    #[test]
    fn test_material() {
        let mut s = TestShape::default();

        assert_eq!(s.material(), &Material::default());

        let override_material = Material::default().with_ambient(1.0);
        s.set_material(override_material);

        assert_eq!(s.material(), &override_material);
    }

    #[test]
    fn test_transform() {
        let mut s = TestShape::default();

        assert_eq!(s.transform(), &Matrix::identity_4());

        let new_transform = Matrix::translation(2, 3, 4);
        s.set_transform(new_transform.clone());

        assert_eq!(s.transform(), &new_transform);
    }

    #[test]
    fn intersect_scaled() {
        let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Matrix::scaling(2, 2, 2));

        s.intersect(&r);
        let intersected_ray = match s.last_intersected_ray.into_inner() {
            Some(ray) => ray,
            None => panic!("Expected an intersected ray, but found None"),
        };

        assert_eq!(intersected_ray.origin(), Tuple::new_point(0.0, 0.0, -2.5));
        assert_eq!(
            intersected_ray.direction(),
            Tuple::new_vector(0.0, 0.0, 0.5)
        );
    }

    #[test]
    fn intersect_translated() {
        let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Matrix::translation(5, 0, 0));

        s.intersect(&r);
        let intersected_ray = match s.last_intersected_ray.into_inner() {
            Some(ray) => ray,
            None => panic!("Expected an intersected ray, but found None"),
        };

        assert_eq!(intersected_ray.origin(), Tuple::new_point(-5, 0, -5));
        assert_eq!(intersected_ray.direction(), Tuple::new_vector(0, 0, 1));
    }

    #[test]
    fn normal_at_translated() {
        // Normal vector should not be impacted by translation.
        let mut s = TestShape::default();
        s.set_transform(Matrix::translation(0, 1, 0));

        let n = s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));

        // We know `normal_at_local` echos back the provided point's components
        // as a vector, so we're expecting the point to be translated to
        // (0, 0.70711, -0.70711) before being passed to `normal_at_local`.
        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_at_transformed_shape() {
        let frac_pi_5 = PI / 5.0;
        let frac_sqrt_2_2 = SQRT_2 / 2.0;

        let mut s = TestShape::default();
        s.set_transform(&Matrix::scaling(1, 0.5, 1) * &Matrix::rotation_z(frac_pi_5));

        let n = s.normal_at(&Tuple::new_point(0.0, frac_sqrt_2_2, -frac_sqrt_2_2));

        assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
    }
}
