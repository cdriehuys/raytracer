use raytracer::canvas::{renderers::render_as_ppm, Canvas};
use raytracer::lights::PointLight;
use raytracer::linear::Tuple;
use raytracer::objects::{Sphere, WorldObject};
use raytracer::{Color, Material, Ray};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    let eye_position = Tuple::new_point(0, 0, -5);
    let canvas_z = 10.0;

    let pixel_size = 7.0 / canvas.height() as f64;

    let sphere =
        Sphere::default().with_material(Material::default().with_color(Color::new(1, 0.2, 1)));

    let light = PointLight::new(Tuple::new_point(-10, 10, -10), Color::new(1, 1, 1));

    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let world_x = (x as f64 - (canvas.width() as f64 / 2.0)) * pixel_size;
            let world_y = (y as f64 - (canvas.height() as f64 / 2.0)) * pixel_size;

            let pixel_location = Tuple::new_point(world_x, world_y, canvas_z);
            let ray = Ray::new(eye_position, (pixel_location - eye_position).normalized());

            let color = match sphere.intersect(&ray).hit() {
                Some(hit) => {
                    let position = ray.position_at(hit.t());
                    let normal = sphere.normal_at(&position);
                    let eye = -ray.direction();

                    sphere.material().light(&light, &position, &eye, &normal)
                }
                None => Color::new(0, 0, 0),
            };

            set_world_pixel(&mut canvas, x, y, &color);
        }

        println!("Finished row: {:3}", y);
    }

    let outfile = File::create("./sphere.ppm").expect("Failed to open output file.");
    let writer = BufWriter::new(outfile);

    println!("Writing plot image...");
    render_as_ppm(&canvas, writer).expect("Failed to render as PPM.");
    println!("Finished writing plot image.");
}

fn set_world_pixel(canvas: &mut Canvas, x: usize, y: usize, color: &Color) {
    let y = canvas.height() - y - 1;

    canvas.write_pixel(x, y, *color);
}
