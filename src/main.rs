use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs::File,
    io::BufWriter,
};

use raytracer::{
    camera::{view_transform, Camera},
    canvas::renderers::render_as_ppm,
    linear::{Matrix, Tuple},
    objects::{Shape, Sphere},
    Color, Material, World,
};

fn main() {
    let floor = {
        let mut floor = Sphere::default();
        floor.set_transform(Matrix::scaling(10, 0.01, 10));
        floor.set_material(
            Material::default()
                .with_color(Color::new(1, 0.9, 0.9))
                .with_specular(0.0),
        );

        floor
    };

    let left_wall = {
        let mut left_wall = Sphere::default();
        left_wall.set_transform(
            &(&(&Matrix::translation(0, 0, 5) * &Matrix::rotation_y(-FRAC_PI_4))
                * &Matrix::rotation_x(FRAC_PI_2))
                * &Matrix::scaling(10, 0.01, 10),
        );
        left_wall.set_material(floor.material().clone());

        left_wall
    };

    let right_wall = {
        let mut right_wall = Sphere::default();
        right_wall.set_transform(
            &(&(&Matrix::translation(0, 0, 5) * &Matrix::rotation_y(FRAC_PI_4))
                * &Matrix::rotation_x(FRAC_PI_2))
                * &Matrix::scaling(10, 0.01, 10),
        );
        right_wall.set_material(floor.material().clone());

        right_wall
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
