use crate::{
    intersections::{Intersection, Intersections},
    linear::Tuple,
    Ray,
};

use super::{BaseShape, Shape};

/// A plane that extends infinitely along the x- and z-axis.
#[derive(Clone, Debug, Default)]
pub struct Plane {
    base: BaseShape,
}

impl Shape for Plane {
    fn base_shape(&self) -> &BaseShape {
        &self.base
    }

    fn base_shape_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn intersect_local(&self, ray: &Ray) -> Intersections {
        // If the ray has no y-component, we know it will never hit the plane.
        // It's either parallel to the plane, or it's coplanar in which case we
        // treat the infinite number of intersections as having no
        // intersections.
        if ray.direction().y().abs() < 1e-5 {
            return Intersections::default();
        }

        let t = -ray.origin().y() / ray.direction().y();
        let intersection = Intersection::new(t, self);

        Intersections::new(vec![intersection])
    }

    fn normal_at_local(&self, _point: &Tuple) -> Tuple {
        // Since the plane is an xz plane, the normal is constant and points
        // along the y-axis.
        Tuple::new_vector(0, 1, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersecct_local_ray_parallel_to_plane() {
        let p = Plane::default();
        let r = Ray::new(Tuple::new_point(0, 10, 0), Tuple::new_vector(0, 0, 1));

        let intersections = p.intersect_local(&r);

        assert!(
            intersections.is_empty(),
            "Found unexpected intersections: {:?}",
            intersections
        );
    }

    #[test]
    fn intersecct_local_ray_coplanar() {
        let p = Plane::default();
        let r = Ray::new(Tuple::new_point(0, 0, 0), Tuple::new_vector(0, 0, 1));

        let intersections = p.intersect_local(&r);

        assert!(
            intersections.is_empty(),
            "Found unexpected intersections: {:?}",
            intersections
        );
    }

    #[test]
    fn intersect_local_from_above() {
        let p = Plane::default();
        let r = Ray::new(Tuple::new_point(0, 1, 0), Tuple::new_vector(0, -1, 0));

        let intersections = p.intersect_local(&r);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t(), 1.0);
        assert_eq!(intersections[0].object(), &p);
    }

    #[test]
    fn intersect_local_from_below() {
        let p = Plane::default();
        let r = Ray::new(Tuple::new_point(0, -1, 0), Tuple::new_vector(0, 1, 0));

        let intersections = p.intersect_local(&r);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t(), 1.0);
        assert_eq!(intersections[0].object(), &p);
    }

    #[test]
    fn normal_at_local_is_consistent_everywhere() {
        let p = Plane::default();

        let n1 = p.normal_at_local(&Tuple::new_point(0, 0, 0));
        let n2 = p.normal_at_local(&Tuple::new_point(10, 0, -10));
        let n3 = p.normal_at_local(&Tuple::new_point(-5, 0, 150));

        let want = Tuple::new_vector(0, 1, 0);

        assert_eq!(n1, want);
        assert_eq!(n2, want);
        assert_eq!(n3, want);
    }
}
