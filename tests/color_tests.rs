use raytracer::Color;

#[test]
fn create_color() {
    let color = Color::new(-0.5, 0.4, 1.7);

    assert_eq!(color.red(), -0.5);
    assert_eq!(color.green(), 0.4);
    assert_eq!(color.blue(), 1.7);
}

#[test]
fn add() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let want = Color::new(1.6, 0.7, 1);

    assert_eq!(c1 + c2, want);
}

#[test]
fn equals_mismatched_red() {
    let c1 = Color::new(0, 0, 0);
    let c2 = Color::new(1, 0, 0);

    assert_ne!(c1, c2);
}

#[test]
fn equals_mismatched_green() {
    let c1 = Color::new(0, 0, 0);
    let c2 = Color::new(0, 1, 0);

    assert_ne!(c1, c2);
}

#[test]
fn equals_mismatched_blue() {
    let c1 = Color::new(0, 0, 0);
    let c2 = Color::new(0, 0, 1);

    assert_ne!(c1, c2);
}

#[test]
fn equals_floating_point_comparison() {
    let c1 = Color::new(0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15);
    let c2 = Color::new(0.1 + 0.1 + 0.25, 0.1 + 0.1 + 0.25, 0.1 + 0.1 + 0.25);

    assert_eq!(c1, c2);
}

#[test]
fn multiply_color() {
    let c1 = Color::new(1, 0.2, 0.4);
    let c2 = Color::new(0.9, 1, 0.1);

    let want = Color::new(0.9, 0.2, 0.04);

    assert_eq!(c1 * c2, want);
}

#[test]
fn multiply_scalar() {
    let c1 = Color::new(0.2, 0.3, 0.4);
    let scalar = 2.0;

    let want = Color::new(0.4, 0.6, 0.8);

    assert_eq!(c1 * scalar, want);
}

#[test]
fn subtract() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    let want = Color::new(0.2, 0.5, 0.5);

    assert_eq!(c1 - c2, want);
}
