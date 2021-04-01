use float_cmp::approx_eq;
use raytracer::linear::Tuple;

#[test]
fn cross_product_vectors() {
    let a = Tuple::new_vector(1, 2, 3);
    let b = Tuple::new_vector(2, 3, 4);

    assert_eq!(a.cross(b), Tuple::new_vector(-1, 2, -1));
    assert_eq!(b.cross(a), Tuple::new_vector(1, -2, 1));
}

#[test]
fn dot_product_vectors() {
    let a = Tuple::new_vector(1, 2, 3);
    let b = Tuple::new_vector(2, 3, 4);

    assert_eq!(a.dot(b), 20.0);
    assert_eq!(b.dot(a), 20.0);
}

#[test]
fn normalize_x_vector() {
    let vector = Tuple::new_vector(4, 0, 0);
    let want = Tuple::new_vector(1, 0, 0);

    assert_eq!(vector.normalized(), want);
}

#[test]
fn normalize_arbitrary_vector() {
    let vector = Tuple::new_vector(1, 2, 3);

    // The vector has a magnitude of sqrt(14), so the normalized vector's
    // components should be scaled by that factor.
    let magnitude = (14 as f64).sqrt();
    let want = Tuple::new_vector(1.0 / magnitude, 2.0 / magnitude, 3.0 / magnitude);

    assert_eq!(vector.normalized(), want);
}

#[test]
fn magnitude() {
    let vector = Tuple::new_vector(1, 2, 3);
    let want = (14 as f64).sqrt();

    assert!(approx_eq!(f64, vector.magnitude(), want));
}

#[test]
fn magnitude_negative_components() {
    let vector = Tuple::new_vector(-1, -2, -3);
    let want = (14 as f64).sqrt();

    assert!(approx_eq!(f64, vector.magnitude(), want));
}
