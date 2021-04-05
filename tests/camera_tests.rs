use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use raytracer::{
    camera::{view_transform, Camera},
    linear::{Matrix, Tuple},
    Color, World,
};

#[test]
fn view_transform_looking_positive_z() {
    let from = Tuple::new_point(0, 0, 0);
    let to = Tuple::new_point(0, 0, 1);
    let up = Tuple::new_vector(0, 1, 0);

    let transform = view_transform(&from, &to, &up);

    assert_eq!(transform, Matrix::scaling(-1, 1, -1));
}

#[test]
fn view_transform_moves_the_world() {
    let from = Tuple::new_point(0, 0, 8);
    let to = Tuple::new_point(0, 0, 0);
    let up = Tuple::new_vector(0, 1, 0);

    let transform = view_transform(&from, &to, &up);

    assert_eq!(transform, Matrix::translation(0, 0, -8));
}

#[test]
fn view_transform_arbitrary() {
    let from = Tuple::new_point(1, 3, 2);
    let to = Tuple::new_point(4, -2, 8);
    let up = Tuple::new_vector(1, 1, 0);

    #[rustfmt::skip]
    let want = Matrix::square_4(
        -0.50709, 0.50709, 0.67612, -2.36643,
        0.76772, 0.60609, 0.12122, -2.82843,
        -0.35857, 0.59761, -0.71714, 0.00000,
        0.00000, 0.00000, 0.00000, 1.00000,
    );

    let transform = view_transform(&from, &to, &up);

    assert_eq!(transform, want);
}

#[test]
fn camera_ray_center_of_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2);

    let r = c.ray_for_pixel(100, 50);

    assert_eq!(r.origin(), Tuple::new_point(0, 0, 0));
    assert_eq!(r.direction(), Tuple::new_vector(0, 0, -1));
}

#[test]
fn camera_ray_corner_of_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2);

    let r = c.ray_for_pixel(0, 0);

    assert_eq!(r.origin(), Tuple::new_point(0, 0, 0));
    assert_eq!(r.direction(), Tuple::new_vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn camera_ray_camera_has_transform() {
    let c = Camera::new(201, 101, FRAC_PI_2)
        .with_transform(&Matrix::rotation_y(FRAC_PI_4) * &Matrix::translation(0, -2, 5));

    let r = c.ray_for_pixel(100, 50);

    let sqrt_2_over_2 = (2.0 as f64).sqrt() / 2.0;

    assert_eq!(r.origin(), Tuple::new_point(0, 2, -5));
    assert_eq!(
        r.direction(),
        Tuple::new_vector(sqrt_2_over_2, 0.0, -sqrt_2_over_2)
    );
}

#[test]
pub fn camera_render_sanity_check() {
    let w = World::default();
    let from = Tuple::new_point(0, 0, -5);
    let to = Tuple::new_point(0, 0, 0);
    let up = Tuple::new_vector(0, 1, 0);

    let c = Camera::new(11, 11, FRAC_PI_2).with_transform(view_transform(&from, &to, &up));

    let image = c.render(&w);

    assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
}
