use raytracer::linear::Matrix;
use raytracer::Tuple;

#[test]
fn create_2x2() {
    #[rustfmt::skip]
    let m = Matrix::square_2(
        -3.0, 5.0,
        1.0, -2.0,
    );

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
}

#[test]
fn create_3x3() {
    #[rustfmt::skip]
    let m = Matrix::square_3(
        -3.0, 5.0, 0.0,
        1.0, -2.0, -7.0,
        0.0, 1.0, 1.0,
    );

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[1][1], -2.0);
    assert_eq!(m[2][2], 1.0);
}

#[test]
fn create_4x4() {
    #[rustfmt::skip]
    let m = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        5.5, 6.5, 7.5, 8.5,
        9.0, 10.0, 11.0, 12.0,
        13.5, 14.5, 15.5, 16.5,
    );

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn equal_same_4x4() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    );

    #[rustfmt::skip]
    let b = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    );

    assert_eq!(a, b);
}

#[test]
fn equal_different_4x4() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    );

    #[rustfmt::skip]
    let b = Matrix::square_4(
        2.0, 3.0, 4.0, 5.0,
        6.0, 7.0, 8.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    );

    assert_ne!(a, b);
}

#[test]
fn multiply_4x4() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 8.0, 7.0, 6.0,
        5.0, 4.0, 3.0, 2.0,
    );

    #[rustfmt::skip]
    let b = Matrix::square_4(
        -2.0, 1.0, 2.0, 3.0,
        3.0, 2.0, 1.0, -1.0,
        4.0, 3.0, 6.0, 5.0,
        1.0, 2.0, 7.0, 8.0,
    );

    #[rustfmt::skip]
    let want = Matrix::square_4(
        20.0, 22.0, 50.0, 48.0,
        44.0, 54.0, 114.0, 108.0,
        40.0, 58.0, 110.0, 102.0,
        16.0, 26.0, 46.0, 42.0,
    );

    assert_eq!(&a * &b, want);
}

#[test]
fn multiply_4x4_identity() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        0.0, 1.0, 2.0, 4.0,
        1.0, 2.0, 4.0, 8.0,
        2.0, 4.0, 8.0, 16.0,
        4.0, 8.0, 16.0, 32.0,
    );

    assert_eq!(&a * &Matrix::identity_4(), a);
}

#[test]
fn multiply_4x4_by_tuple() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        1.0, 2.0, 3.0, 4.0,
        2.0, 4.0, 4.0, 2.0,
        8.0, 6.0, 4.0, 1.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let tuple = Tuple::new(1, 2, 3, 1);

    let want = Tuple::new(18, 24, 33, 1);

    assert_eq!(&a * tuple, want);
}

#[test]
fn multiply_identity_4_by_tuple() {
    let identity = Matrix::identity_4();
    let tuple = Tuple::new(1, 2, 3, 4);

    assert_eq!(&identity * tuple, tuple);
}

#[test]
fn transpose_4x4() {
    #[rustfmt::skip]
    let a = Matrix::square_4(
        0.0, 9.0, 3.0, 0.0,
        9.0, 8.0, 0.0, 8.0,
        1.0, 8.0, 5.0, 3.0,
        0.0, 0.0, 5.0, 8.0,
    );

    #[rustfmt::skip]
    let want = Matrix::square_4(
        0.0, 9.0, 1.0, 0.0,
        9.0, 8.0, 8.0, 0.0,
        3.0, 0.0, 5.0, 5.0,
        0.0, 8.0, 3.0, 8.0,
    );

    assert_eq!(a.transposed(), want);
}

#[test]
fn transpose_identity_4() {
    let identity = Matrix::identity_4();

    assert_eq!(identity.transposed(), identity);
}
