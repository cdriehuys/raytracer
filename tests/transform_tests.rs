use raytracer::linear::{Matrix, Tuple};
use std::f64::consts::{FRAC_PI_2, PI, SQRT_2};

const SQRT_2_OVER_2: f64 = SQRT_2 / 2.0;

#[test]
fn translate_point() {
    let transform = Matrix::translation(5, -3, 2);
    let point = Tuple::new_point(-3, 4, 5);

    let want = Tuple::new_point(2, 1, 7);

    assert_eq!(&transform * point, want);
}

#[test]
fn translate_point_inverse() {
    let transform = Matrix::translation(5, -3, 2);
    let inverse = transform.inverted();
    let point = Tuple::new_point(-3, 4, 5);

    let want = Tuple::new_point(-8, 7, 3);

    assert_eq!(&inverse * point, want);
}

#[test]
fn translate_does_not_affect_vectors() {
    let transform = Matrix::translation(5, -3, 2);
    let vector = Tuple::new_vector(-3, 4, 5);

    assert_eq!(&transform * vector, vector);
}

#[test]
fn scale_point() {
    let transform = Matrix::scaling(2, 3, 4);
    let point = Tuple::new_point(-4, 6, 8);

    let want = Tuple::new_point(-8, 18, 32);

    assert_eq!(&transform * point, want);
}

#[test]
fn scale_vector() {
    let transform = Matrix::scaling(2, 3, 4);
    let vector = Tuple::new_vector(-4, 6, 8);

    let want = Tuple::new_vector(-8, 18, 32);

    assert_eq!(&transform * vector, want);
}

#[test]
fn scale_vector_inverse() {
    let transform = Matrix::scaling(2, 3, 4).inverted();
    let vector = Tuple::new_vector(-4, 6, 8);

    let want = Tuple::new_vector(-2, 2, 2);

    assert_eq!(&transform * vector, want);
}

#[test]
fn scale_reflection() {
    let transform = Matrix::scaling(-1, 1, 1);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(-2, 3, 4);

    assert_eq!(&transform * point, want);
}

#[test]
fn rotate_x() {
    let point = Tuple::new_point(0, 1, 0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let full_quarter = Matrix::rotation_x(PI / 2.0);

    let want_half = Tuple::new_point(0.0, SQRT_2_OVER_2, SQRT_2_OVER_2);
    let want_full = Tuple::new_point(0, 0, 1);

    assert_eq!(&half_quarter * point, want_half);
    assert_eq!(&full_quarter * point, want_full);
}

#[test]
fn rotate_x_inverse() {
    let point = Tuple::new_point(0, 1, 0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let inverse = half_quarter.inverted();

    let want = Tuple::new_point(0.0, SQRT_2_OVER_2, -SQRT_2_OVER_2);

    assert_eq!(&inverse * point, want);
}

#[test]
fn rotate_y() {
    let point = Tuple::new_point(0, 0, 1);
    let half_quarter = Matrix::rotation_y(PI / 4.0);
    let full_quarter = Matrix::rotation_y(PI / 2.0);

    let want_half = Tuple::new_point(SQRT_2_OVER_2, 0.0, SQRT_2_OVER_2);
    let want_full = Tuple::new_point(1, 0, 0);

    assert_eq!(&half_quarter * point, want_half);
    assert_eq!(&full_quarter * point, want_full);
}

#[test]
fn rotate_z() {
    let point = Tuple::new_point(0, 1, 0);
    let half_quarter = Matrix::rotation_z(PI / 4.0);
    let full_quarter = Matrix::rotation_z(PI / 2.0);

    let want_half = Tuple::new_point(-SQRT_2_OVER_2, SQRT_2_OVER_2, 0.0);
    let want_full = Tuple::new_point(-1, 0, 0);

    assert_eq!(&half_quarter * point, want_half);
    assert_eq!(&full_quarter * point, want_full);
}

#[test]
fn shear_x_in_proportion_to_y() {
    let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(5, 3, 4);

    assert_eq!(&transform * point, want);
}

#[test]
fn shear_x_in_proportion_to_z() {
    let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(6, 3, 4);

    assert_eq!(&transform * point, want);
}

#[test]
fn shear_y_in_proportion_to_x() {
    let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(2, 5, 4);

    assert_eq!(&transform * point, want);
}

#[test]
fn shear_y_in_proportion_to_z() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(2, 7, 4);

    assert_eq!(&transform * point, want);
}

#[test]
fn shear_z_in_proportion_to_x() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(2, 3, 6);

    assert_eq!(&transform * point, want);
}

#[test]
fn shear_z_in_proportion_to_y() {
    let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let point = Tuple::new_point(2, 3, 4);

    let want = Tuple::new_point(2, 3, 7);

    assert_eq!(&transform * point, want);
}

#[test]
fn transform_order_sequence() {
    let p = Tuple::new_point(1, 0, 1);
    let a = Matrix::rotation_x(FRAC_PI_2);
    let b = Matrix::scaling(5, 5, 5);
    let c = Matrix::translation(10, 5, 7);

    let p2 = &a * p;
    assert_eq!(p2, Tuple::new_point(1, -1, 0));

    let p3 = &b * p2;
    assert_eq!(p3, Tuple::new_point(5, -5, 0));

    let p4 = &c * p3;
    assert_eq!(p4, Tuple::new_point(15, 0, 7));
}

#[test]
fn transform_order_chained() {
    let p = Tuple::new_point(1, 0, 1);
    let a = Matrix::rotation_x(FRAC_PI_2);
    let b = Matrix::scaling(5, 5, 5);
    let c = Matrix::translation(10, 5, 7);

    let transform = &(&c * &b) * &a;

    assert_eq!(&transform * p, Tuple::new_point(15, 0, 7));
}
