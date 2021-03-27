use raytracer;

struct Projectile {
    position: raytracer::Tuple,
    velocity: raytracer::Tuple,
}

impl Projectile {
    fn tick(&mut self, environment: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + environment.gravity + environment.wind;
    }
}

struct Environment {
    gravity: raytracer::Tuple,
    wind: raytracer::Tuple,
}

fn main() {
    let mut p = Projectile {
        position: raytracer::Tuple::new_point(0, 1, 0),
        velocity: raytracer::Tuple::new_vector(1, 1, 0).normalized(),
    };
    let e = Environment {
        gravity: raytracer::Tuple::new_vector(0.0, -0.1, 0.0),
        wind: raytracer::Tuple::new_vector(-0.01, 0.0, 0.0),
    };

    let mut tick = 0;
    while p.position.get_y() > 0.0 {
        tick += 1;
        p.tick(&e);

        println!("Tick {} - Position {:?}", tick, p.position);
    }
}
