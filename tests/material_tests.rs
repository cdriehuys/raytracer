use raytracer::{lights::PointLight, linear::Tuple, Color, Material};

#[test]
fn light_eye_between_light_and_surface() {
    let m = Material::default();
    let position = Tuple::new_point(0, 0, 0);

    let eye_v = Tuple::new_vector(0, 0, -1);
    let normal_v = Tuple::new_vector(0, 0, -1);
    let light = PointLight::new(Tuple::new_point(0, 0, -10), Color::new(1, 1, 1));

    let result = m.light(&light, &position, &eye_v, &normal_v);

    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn light_eye_between_light_and_surface_with_eye_45_offset() {
    let m = Material::default();
    let position = Tuple::new_point(0, 0, 0);

    let sqrt_2_over_2 = (2.0 as f64).sqrt() / 2.0;

    let eye_v = Tuple::new_vector(0.0, sqrt_2_over_2, -sqrt_2_over_2);
    let normal_v = Tuple::new_vector(0, 0, -1);
    let light = PointLight::new(Tuple::new_point(0, 0, -10), Color::new(1, 1, 1));

    let result = m.light(&light, &position, &eye_v, &normal_v);

    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn light_eye_opposite_surface_with_light_45_offset() {
    let m = Material::default();
    let position = Tuple::new_point(0, 0, 0);

    let eye_v = Tuple::new_vector(0, 0, -1);
    let normal_v = Tuple::new_vector(0, 0, -1);
    let light = PointLight::new(Tuple::new_point(0, 10, -10), Color::new(1, 1, 1));

    let result = m.light(&light, &position, &eye_v, &normal_v);

    assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
}

#[test]
fn light_eye_in_path_of_reflection_vector() {
    let m = Material::default();
    let position = Tuple::new_point(0, 0, 0);

    let sqrt_2_over_2 = (2.0 as f64).sqrt() / 2.0;

    let eye_v = Tuple::new_vector(0.0, -sqrt_2_over_2, -sqrt_2_over_2);
    let normal_v = Tuple::new_vector(0, 0, -1);
    let light = PointLight::new(Tuple::new_point(0, 10, -10), Color::new(1, 1, 1));

    let result = m.light(&light, &position, &eye_v, &normal_v);

    assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
}

#[test]
fn light_with_light_behind_surface() {
    let m = Material::default();
    let position = Tuple::new_point(0, 0, 0);

    let eye_v = Tuple::new_vector(0, 0, -1);
    let normal_v = Tuple::new_vector(0, 0, -1);
    let light = PointLight::new(Tuple::new_point(0, 0, 10), Color::new(1, 1, 1));

    let result = m.light(&light, &position, &eye_v, &normal_v);

    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
