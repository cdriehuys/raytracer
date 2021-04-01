use raytracer::colors::Color;
use raytracer::linear::Tuple;
use raytracer::{
    canvas::{renderers::render_as_ppm, Canvas},
    linear::Matrix,
};
use std::io::BufWriter;
use std::{f64::consts::PI, fs::File};

fn main() {
    let mut canvas = Canvas::new(500, 500);

    // Transform to move from the origin (top left of canvas) to the center.
    let center_transform = Matrix::translation(250, 250, 0);

    // Transform to move from the origin to the tick mark on the clock.
    let hour_hand_transform = Matrix::translation(0, 200, 0);

    for hour in 0..12 {
        let rotation_radians = 2.0 * PI / 12.0 * hour as f64;
        let rotation = Matrix::rotation_z(rotation_radians);

        // Our transform steps are:
        // 1. Move out from the origin by the radius of the clock.
        // 2. Rotate around the origin based on the hour.
        // 3. Apply the offset to move from the origin to the center of the
        //    canvas.
        //
        // We chain these together by multiplying them in reverse.
        let transform = &(&center_transform * &rotation) * &hour_hand_transform;

        let point = &transform * Tuple::new_point(0, 0, 0);

        record_position(&mut canvas, &point);
    }

    let outfile = File::create("./clock.ppm").expect("Failed to open output file.");
    let writer = BufWriter::new(outfile);

    println!("Writing plot image...");
    render_as_ppm(&canvas, writer).expect("Failed to render as PPM.");
    println!("Finished writing plot image.");
}

fn record_position(canvas: &mut Canvas, position: &Tuple) {
    let x = position.x().round() as usize;
    let y = position.y().round() as usize;

    // Record a 3x3 area around the position to make it easier to see.
    for x in (x - 1)..(x + 1) {
        for y in (y - 1)..(y + 1) {
            if x >= canvas.width() || y >= canvas.height() {
                continue;
            }

            canvas.write_pixel(x, y, Color::new(1, 0, 0));
        }
    }
}
