use raytracer::{linear::Tuple, Ray, World};

#[test]
fn intersect_default_world() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));

    let intersections = w.intersect(&r);

    // Should intersect with both spheres twice:
    // --> (Outer) --> (Inner) --> (Inner) --> (Outer) -->
    assert_eq!(intersections.len(), 4);

    assert_eq!(intersections[0].t(), 4.0);
    assert_eq!(intersections[1].t(), 4.5);
    assert_eq!(intersections[2].t(), 5.5);
    assert_eq!(intersections[3].t(), 6.0);
}
