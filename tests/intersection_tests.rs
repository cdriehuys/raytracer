use raytracer::intersections::{Intersection, Intersections};
use raytracer::objects::Sphere;

#[test]
fn hit_all_positive_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, Box::new(s.clone()));
    let i2 = Intersection::new(2.0, Box::new(s.clone()));

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(1.0, Box::new(s.clone())));
}

#[test]
fn hit_some_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, Box::new(s.clone()));
    let i2 = Intersection::new(1.0, Box::new(s.clone()));

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(1.0, Box::new(s.clone())));
}

#[test]
fn hit_all_negative_t() {
    let s = Sphere::default();
    let i1 = Intersection::new(-2.0, Box::new(s.clone()));
    let i2 = Intersection::new(-1.0, Box::new(s.clone()));

    let intersections = Intersections::new(vec![i1, i2]);
    let hit = intersections.hit();

    if let Some(intersection) = hit {
        panic!("Unexpected hit: {:?}", intersection);
    }
}

#[test]
fn hit_is_lowest_non_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(5.0, Box::new(s.clone()));
    let i2 = Intersection::new(7.0, Box::new(s.clone()));
    let i3 = Intersection::new(-3.0, Box::new(s.clone()));
    let i4 = Intersection::new(2.0, Box::new(s.clone()));

    let intersections = Intersections::new(vec![i1, i2, i3, i4]);
    let hit = intersections.hit();

    assert_eq!(*hit.unwrap(), Intersection::new(2.0, Box::new(s.clone())));
}
