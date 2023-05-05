use crate::intersections::{Intersection, Intersections};
use crate::linear::Tuple;
use crate::Ray;

use super::{BaseShape, Shape};

#[derive(Clone, Debug, Default)]
pub struct Sphere {
    base: BaseShape,
}

impl Shape for Sphere {
    fn base_shape(&self) -> &BaseShape {
        &self.base
    }

    fn base_shape_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn intersect_local(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin() - Tuple::new_point(0, 0, 0);

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        // A discriminant less than zero indicates the ray misses the sphere
        // entirely.
        if discriminant < 0.0 {
            return Intersections::default();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);

        Intersections::new(vec![i1, i2])
    }

    fn normal_at_local(&self, point: &Tuple) -> Tuple {
        *point - Tuple::new_point(0, 0, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect_local_two_points() {
        let ray = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = sphere.intersect_local(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t(), 4.0);
        assert_eq!(intersections[1].t(), 6.0);
    }

    #[test]
    fn intersect_local_tangent() {
        let ray = Ray::new(Tuple::new_point(0, 1, -5), Tuple::new_vector(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = sphere.intersect_local(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t(), 5.0);
        assert_eq!(intersections[1].t(), 5.0);
    }

    #[test]
    fn intersect_local_no_hits() {
        let ray = Ray::new(Tuple::new_point(0, 2, -5), Tuple::new_vector(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = sphere.intersect_local(&ray);

        assert!(
            intersections.is_empty(),
            "Found unexpected intersections: {:?}",
            intersections
        );
    }

    #[test]
    fn intersect_local_origin_inside_sphere() {
        let ray = Ray::new(Tuple::new_point(0, 0, 0), Tuple::new_vector(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = sphere.intersect_local(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t(), -1.0);
        assert_eq!(intersections[1].t(), 1.0);
    }

    #[test]
    fn intersect_local_ray_in_front_of_sphere() {
        let ray = Ray::new(Tuple::new_point(0, 0, 5), Tuple::new_vector(0, 0, 1));
        let sphere = Sphere::default();

        let intersections = sphere.intersect_local(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t(), -6.0);
        assert_eq!(intersections[1].t(), -4.0);
    }

    #[test]
    fn normal_at_local_on_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at_local(&Tuple::new_point(1, 0, 0));

        assert_eq!(n, Tuple::new_vector(1, 0, 0));
    }

    #[test]
    fn normal_at_local_on_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at_local(&Tuple::new_point(0, 1, 0));

        assert_eq!(n, Tuple::new_vector(0, 1, 0));
    }

    #[test]
    fn normal_at_local_on_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at_local(&Tuple::new_point(0, 0, 1));

        assert_eq!(n, Tuple::new_vector(0, 0, 1));
    }

    #[test]
    fn normal_at_local_non_axial_point() {
        let s = Sphere::default();
        let sqrt_3_over_3 = (3.0 as f64).sqrt() / 3.0;

        let n = s.normal_at_local(&Tuple::new_point(
            sqrt_3_over_3,
            sqrt_3_over_3,
            sqrt_3_over_3,
        ));

        assert_eq!(
            n,
            Tuple::new_vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3)
        );
    }

    #[test]
    fn normal_at_local_is_normalized() {
        let s = Sphere::default();
        let sqrt_3_over_3 = (3.0 as f64).sqrt() / 3.0;

        let n = s.normal_at_local(&Tuple::new_point(
            sqrt_3_over_3,
            sqrt_3_over_3,
            sqrt_3_over_3,
        ));

        assert_eq!(n, n.normalized());
    }
}
