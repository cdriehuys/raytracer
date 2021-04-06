use raytracer::{
    intersections::Intersection,
    lights::PointLight,
    linear::{Matrix, Tuple},
    objects::{Sphere, WorldObject},
    Color, Ray, World, DEFAULT_SPHERE_1, DEFAULT_SPHERE_2,
};

#[test]
fn intersect_default_world() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));

    let intersections = w.intersect(&r);

    // Should intersect with both spheres twice:
    // --> (Outer) --> (Inner) --> (Inner) --> (Outer) -->
    assert_eq!(intersections.len(), 4);

    assert_eq!(intersections[0].t(), 4.0);
    assert_eq!(intersections[1].t(), 4.5);
    assert_eq!(intersections[2].t(), 5.5);
    assert_eq!(intersections[3].t(), 6.0);
}

#[test]
fn is_shadowed_no_obstruction() {
    let w = World::default();
    let p = Tuple::new_point(0, 10, 0);

    assert!(!w.is_shadowed(&p));
}

#[test]
fn is_shadowed_obstruction() {
    let w = World::default();
    let p = Tuple::new_point(10, -10, 10);

    assert!(w.is_shadowed(&p))
}

#[test]
fn is_shadowed_object_behind_light() {
    let w = World::default();
    let p = Tuple::new_point(-20, 20, -20);

    assert!(!w.is_shadowed(&p));
}

#[test]
fn is_shadowed_object_behind_point() {
    let w = World::default();
    let p = Tuple::new_point(-2, 2, -2);

    assert!(!w.is_shadowed(&p));
}

#[test]
fn shade_hit() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));
    let shape = w.objects[0];
    let i = Intersection::new(4.0, shape);

    let info = i.prepare_info(&r);
    let c = w.shade_hit(&info);

    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_hit_inside() {
    let mut w = World::default();
    let light = PointLight::new(Tuple::new_point(0.0, 0.25, 0.0), Color::new(1, 1, 1));
    w.light = Some(&light);
    let r = Ray::new(Tuple::new_point(0, 0, 0), Tuple::new_vector(0, 0, 1));
    let shape = w.objects[1];
    let i = Intersection::new(0.5, shape);

    let info = i.prepare_info(&r);
    let c = w.shade_hit(&info);

    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}

#[test]
fn shade_hit_in_shadow() {
    let light = PointLight::new(Tuple::new_point(0, 0, -10), Color::new(1, 1, 1));
    let s1 = Sphere::default();
    let s2 = Sphere::default().with_transform(Matrix::translation(0, 0, 10));

    let w = {
        let mut world = World::new();
        world.light = Some(&light);
        world.objects = vec![&s1, &s2];

        world
    };

    let r = Ray::new(Tuple::new_point(0, 0, 5), Tuple::new_vector(0, 0, 1));
    let i = Intersection::new(4.0, &s2);

    let info = i.prepare_info(&r);
    let c = w.shade_hit(&info);

    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn color_at_miss() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 1, 0));

    let c = w.color_at(&r);

    assert_eq!(c, Color::new(0, 0, 0));
}

#[test]
fn color_at_hit() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0, 0, -5), Tuple::new_vector(0, 0, 1));

    let c = w.color_at(&r);

    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_with_intersection_behind_ray() {
    let mut w = World::default();
    let outer = DEFAULT_SPHERE_1.with_material(DEFAULT_SPHERE_1.material().with_ambient(1.0));
    let inner = DEFAULT_SPHERE_2.with_material(DEFAULT_SPHERE_2.material().with_ambient(1.0));
    w.objects = vec![&outer, &inner];

    let r = Ray::new(
        Tuple::new_point(0.0, 0.0, 0.75),
        Tuple::new_vector(0, 0, -1),
    );

    let c = w.color_at(&r);

    assert_eq!(c, inner.material().color());
}
