use raytracer::canvas::{renderers::render_as_ppm, Canvas};
use raytracer::colors::Color;
use raytracer::linear::Tuple;
use std::fs::File;
use std::io::BufWriter;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn tick(&mut self, environment: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + environment.gravity + environment.wind;
    }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let mut canvas = Canvas::new(900, 500);

    let start = Tuple::new_point(0, 1, 0);
    let velocity = Tuple::new_vector(1.0, 1.8, 0.0).normalized() * 11.25;

    let mut p = Projectile {
        position: start,
        velocity,
    };
    let e = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0),
    };

    let mut tick = 0;
    while p.position.y() > 0.0 {
        tick += 1;
        p.tick(&e);

        println!(
            "Tick {:3} - Position ({:.2}, {:.2})",
            tick,
            p.position.x(),
            p.position.y()
        );

        record_position(&mut canvas, &p);
    }

    let outfile = File::create("./plot.ppm").expect("Failed to open output file.");
    let writer = BufWriter::new(outfile);

    println!("Writing plot image...");
    render_as_ppm(&canvas, writer).expect("Failed to render as PPM.");
    println!("Finished writing plot image.");
}

fn record_position(canvas: &mut Canvas, projectile: &Projectile) {
    let x = projectile.position.x().round() as usize;
    let y = projectile.position.y().round() as usize;

    // Since the canvas' y-coordinates move top-down, we have to invert the
    // projectile's y-coordinate before plotting it.
    let y = canvas.height() - y;

    // Record a 3x3 area around the projectile's position to make it easier to
    // see.
    for x in (x - 1)..(x + 1) {
        for y in (y - 1)..(y + 1) {
            if x >= canvas.width() || y >= canvas.height() {
                continue;
            }

            canvas.write_pixel(x, y, Color::new(1, 0, 0));
        }
    }
}
