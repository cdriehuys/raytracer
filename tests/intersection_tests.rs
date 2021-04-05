use raytracer::{
    intersections::{Intersection, Intersections},
    linear::Tuple,
    objects::Sphere,
    Ray,
};

#[test]
fn hit_all_positive_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(1.0, &s));
}

#[test]
fn hit_some_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(1.0, &s));
}

#[test]
fn hit_all_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-2.0, &s);
    let i2 = Intersection::new(-1.0, &s);

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    if let Some(intersection) = hit {
        panic!("Unexpected hit: {:?}", intersection);
    }
}

#[test]
fn hit_is_lowest_non_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(7.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(2.0, &s);

    let intersections = Intersections::new(vec![i1, i2, i3, i4]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(2.0, &s));
}

#[test]
fn prepare_info_outside_hit() {
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let shape = Sphere::default();
    let i = Intersection::new(4.0, &shape);

    let info = i.prepare_info(&r);

    assert!(
        !info.inside(),
        "Expected 'inside' to be 'false', but it was 'true'."
    );
}

#[test]
fn prepare_info_inside_hit() {
    let r = Ray::new(Tuple::new_point(0, 0, 0), Tuple::new_vector(0, 0, 1));
    let shape = Sphere::default();
    let i = Intersection::new(1.0, &shape);

    let info = i.prepare_info(&r);

    assert_eq!(info.point(), Tuple::new_point(0, 0, 1));
    assert_eq!(info.eye_vec(), Tuple::new_vector(0, 0, -1));
    assert_eq!(info.inside(), true);
    assert_eq!(info.normal_vec(), Tuple::new_vector(0, 0, -1));
}
