use float_cmp::approx_eq;
use raytracer::Tuple;

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
