use raytracer::canvas::{renderers::render_as_ppm, Canvas};
use raytracer::colors::Color;
use raytracer::linear::Tuple;
use raytracer::objects::{Sphere, WorldObject};
use raytracer::rays::Ray;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let mut canvas = Canvas::new(500, 500);

    let light_position = Tuple::new_point(0, 0, -5);
    let canvas_z = 10.0;

    let pixel_size = 15.0 / canvas.height() as f64;

    let sphere = Sphere::default();

    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let world_x = (x as f64 - (canvas.width() as f64 / 2.0)) * pixel_size;
            let world_y = (y as f64 - (canvas.height() as f64 / 2.0)) * pixel_size;

            let pixel_location = Tuple::new_point(world_x, world_y, canvas_z);
            let ray = Ray::new(
                light_position,
                (pixel_location - light_position).normalized(),
            );

            let color = match sphere.intersect(&ray).hit() {
                Some(_) => Color::new(1, 0, 0),
                None => Color::new(0, 0, 0),
            };

            canvas.write_pixel(x, y, color);
        }

        println!("Finished row: {:3}", y);
    }

    let outfile = File::create("./sphere.ppm").expect("Failed to open output file.");
    let writer = BufWriter::new(outfile);

    println!("Writing plot image...");
    render_as_ppm(&canvas, writer).expect("Failed to render as PPM.");
    println!("Finished writing plot image.");
}
