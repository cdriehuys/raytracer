use std::{f64::consts::FRAC_PI_3, fs::File, io::BufWriter};

use raytracer::{
    camera::{view_transform, Camera},
    canvas::renderers::render_as_ppm,
    linear::{Matrix, Tuple},
    objects::{Plane, Shape, Sphere},
    Color, Material, World,
};

fn main() {
    let floor_material = Material::default()
        .with_color(Color::new(1, 0.9, 0.9))
        .with_specular(0.0);

    let floor = {
        let mut floor = Plane::default();
        floor.set_material(floor_material.clone());

        floor
    };

    let middle = {
        let mut middle = Sphere::default();
        middle.set_transform(Matrix::translation(-0.5, 1, 0.5));
        middle.set_material(
            Material::default()
                .with_color(Color::new(0.1, 1, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

        middle
    };

    let right = {
        let mut right = Sphere::default();
        right.set_transform(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5));
        right.set_material(
            Material::default()
                .with_color(Color::new(0.5, 1, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

        right
    };

    let left = {
        let mut left = Sphere::default();
        left.set_transform(
            &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
        );
        left.set_material(
            Material::default()
                .with_color(Color::new(1, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

        left
    };

    let mut world = World::default();
    world.objects = vec![&floor, &middle, &left, &right];

    let camera = Camera::new(1000, 500, FRAC_PI_3).with_transform(view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0, 1, 0),
        &Tuple::new_vector(0, 1, 0),
    ));

    let canvas = camera.render(&world);

    let outfile = File::create("./scene.ppm").expect("Failed to open output file.");
    let writer = BufWriter::new(outfile);

    println!("Writing plot image...");
    render_as_ppm(&canvas, writer).expect("Failed to render as PPM.");
    println!("Finished writing plot image.");
}
