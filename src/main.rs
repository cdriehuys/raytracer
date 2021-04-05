use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs::File,
    io::BufWriter,
};

use raytracer::{
    camera::{view_transform, Camera},
    canvas::renderers::render_as_ppm,
    linear::{Matrix, Tuple},
    objects::{Sphere, WorldObject},
    Color, Material, World,
};

fn main() {
    let floor = Sphere::default()
        .with_transform(Matrix::scaling(10, 0.01, 10))
        .with_material(
            Material::default()
                .with_color(Color::new(1, 0.9, 0.9))
                .with_specular(0.0),
        );

    let left_wall = Sphere::default()
        .with_transform(
            &(&(&Matrix::translation(0, 0, 5) * &Matrix::rotation_y(-FRAC_PI_4))
                * &Matrix::rotation_x(FRAC_PI_2))
                * &Matrix::scaling(10, 0.01, 10),
        )
        .with_material(floor.material().clone());

    let right_wall = Sphere::default()
        .with_transform(
            &(&(&Matrix::translation(0, 0, 5) * &Matrix::rotation_y(FRAC_PI_4))
                * &Matrix::rotation_x(FRAC_PI_2))
                * &Matrix::scaling(10, 0.01, 10),
        )
        .with_material(floor.material().clone());

    let middle = Sphere::default()
        .with_transform(Matrix::translation(-0.5, 1, 0.5))
        .with_material(
            Material::default()
                .with_color(Color::new(0.1, 1, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let right = Sphere::default()
        .with_transform(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5))
        .with_material(
            Material::default()
                .with_color(Color::new(0.5, 1, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let left = Sphere::default()
        .with_transform(
            &Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
        )
        .with_material(
            Material::default()
                .with_color(Color::new(1, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let mut world = World::default();
    world.objects = vec![&floor, &left_wall, &right_wall, &middle, &left, &right];

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
