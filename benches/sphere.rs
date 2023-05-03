use std::f64::consts::FRAC_PI_3;

use criterion::{criterion_group, criterion_main, Criterion};
use raytracer::{
    camera::{view_transform, Camera},
    linear::Tuple,
    objects::{Shape, Sphere},
    Color, Material, World,
};

fn default_camera(size: usize) -> Camera {
    Camera::new(size, size, FRAC_PI_3).with_transform(view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0, 1, 0),
        &Tuple::new_vector(0, 1, 0),
    ))
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let sphere = {
        let mut sphere = Sphere::default();
        sphere.set_material(
            Material::default()
                .with_color(Color::new(0.1, 1, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

        sphere
    };
    let mut world = World::default();
    world.objects = vec![&sphere];

    let camera = default_camera(50);

    c.bench_function("sphere 50", |b| b.iter(|| camera.render(&world)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
