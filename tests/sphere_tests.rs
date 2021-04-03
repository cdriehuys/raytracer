use raytracer::linear::{Matrix, Tuple};
use raytracer::objects::{Sphere, WorldObject};
use raytracer::Ray;

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
