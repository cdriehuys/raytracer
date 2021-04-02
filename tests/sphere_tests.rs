use raytracer::linear::Tuple;
use raytracer::rays::Ray;
use raytracer::sphere::Sphere;

#[test]
fn intersect_two_points() {
    let ray = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0], 4.0);
    assert_eq!(intersections[1], 6.0);
}

#[test]
fn intersect_tangent() {
    let ray = Ray::new(Tuple::new_point(0, 1, -5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0], 5.0);
    assert_eq!(intersections[1], 5.0);
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
    assert_eq!(intersections[0], -1.0);
    assert_eq!(intersections[1], 1.0);
}

#[test]
fn intersect_ray_in_front_of_sphere() {
    let ray = Ray::new(Tuple::new_point(0, 0, 5), Tuple::new_vector(0, 0, 1));
    let sphere = Sphere::default();

    let intersections = sphere.intersect(&ray);

    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0], -6.0);
    assert_eq!(intersections[1], -4.0);
}
