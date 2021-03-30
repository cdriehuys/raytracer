use raytracer::linear::Matrix;

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
