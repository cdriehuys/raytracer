use std::f64::consts::PI;

use raytracer::linear::{Matrix, Tuple};
use raytracer::objects::{Sphere, WorldObject};
use raytracer::{Material, Ray};

#[test]
fn default_material() {
    let sphere = Sphere::default();

    assert_eq!(*sphere.material(), Material::default());
}

#[test]
fn default_transform() {
    let sphere = Sphere::default();

    assert_eq!(*sphere.transform(), Matrix::identity_4());
}

#[test]
fn intersect_two_points() {
    let ray = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), 4.0);
    assert_eq!(intersections[1].t(), 6.0);
}

#[test]
fn intersect_tangent() {
    let ray = Ray::new(Tuple::new_point(0, 1, -5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), 5.0);
    assert_eq!(intersections[1].t(), 5.0);
}

#[test]
fn intersect_no_hits() {
    let ray = Ray::new(Tuple::new_point(0, 2, -5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 0);
}

#[test]
fn intersect_origin_inside_sphere() {
    let ray = Ray::new(Tuple::new_point(0, 0, 0), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), -1.0);
    assert_eq!(intersections[1].t(), 1.0);
}

#[test]
fn intersect_ray_in_front_of_sphere() {
    let ray = Ray::new(Tuple::new_point(0, 0, 5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), -6.0);
    assert_eq!(intersections[1].t(), -4.0);
}

#[test]
fn intersect_scaled_sphere() {
    let ray = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let mut sphere = Sphere::default();
    sphere.set_transform(Matrix::scaling(2, 2, 2));

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t(), 3.0);
    assert_eq!(intersections[1].t(), 7.0);
}

#[test]
fn intersect_translated_sphere() {
    let ray = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let mut sphere = Sphere::default();
    sphere.set_transform(Matrix::translation(5, 0, 0));

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 0);
}

#[test]
fn normal_on_x_axis() {
    let s = Sphere::default();

    let n = s.normal_at(&Tuple::new_point(1, 0, 0));

    assert_eq!(n, Tuple::new_vector(1, 0, 0));
}

#[test]
fn normal_on_y_axis() {
    let s = Sphere::default();

    let n = s.normal_at(&Tuple::new_point(0, 1, 0));

    assert_eq!(n, Tuple::new_vector(0, 1, 0));
}

#[test]
fn normal_on_z_axis() {
    let s = Sphere::default();

    let n = s.normal_at(&Tuple::new_point(0, 0, 1));

    assert_eq!(n, Tuple::new_vector(0, 0, 1));
}

#[test]
fn normal_non_axial_point() {
    let s = Sphere::default();
    let sqrt_3_over_3 = (3.0 as f64).sqrt() / 3.0;

    let n = s.normal_at(&Tuple::new_point(
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
fn normal_is_normalized() {
    let s = Sphere::default();
    let sqrt_3_over_3 = (3.0 as f64).sqrt() / 3.0;

    let n = s.normal_at(&Tuple::new_point(
        sqrt_3_over_3,
        sqrt_3_over_3,
        sqrt_3_over_3,
    ));

    assert_eq!(n, n.normalized());
}

#[test]
fn normal_translated() {
    let mut s = Sphere::default();
    s.set_transform(Matrix::translation(0, 1, 0));

    let n = s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));

    assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
}

#[test]
fn normal_transformed() {
    let mut s = Sphere::default();
    let transform = &Matrix::scaling(1, 0.5, 1) * &Matrix::rotation_z(PI / 5.0);
    s.set_transform(transform);

    let sqrt_2_over_2 = (2.0 as f64).sqrt() / 2.0;

    let n = s.normal_at(&Tuple::new_point(0.0, sqrt_2_over_2, -sqrt_2_over_2));

    assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
}
