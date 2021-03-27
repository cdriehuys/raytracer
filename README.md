# raytracer-rs

A ray tracer written in Rust, built by following
[*The Ray Tracer Challenge* by Jamis Buck][ray-tracer-challenge].

## Playground

At the end of each chapter, the new capabilities of the ray tracer are utilized
in the main program.

The current behavior is to track projectile motion to utilize tuple addition.

```
$ cargo run
Tick 1 - Position Tuple { x: 0.7071067811865475, y: 1.7071067811865475, z: 0.0, w: 1.0 }
Tick 2 - Position Tuple { x: 1.404213562373095, y: 2.314213562373095, z: 0.0, w: 1.0 }
Tick 3 - Position Tuple { x: 2.0913203435596426, y: 2.821320343559642, z: 0.0, w: 1.0 }
Tick 4 - Position Tuple { x: 2.7684271247461902, y: 3.2284271247461898, z: 0.0, w: 1.0 }
Tick 5 - Position Tuple { x: 3.4355339059327377, y: 3.5355339059327373, z: 0.0, w: 1.0 }
Tick 6 - Position Tuple { x: 4.092640687119285, y: 3.7426406871192848, z: 0.0, w: 1.0 }
Tick 7 - Position Tuple { x: 4.739747468305833, y: 3.849747468305832, z: 0.0, w: 1.0 }
Tick 8 - Position Tuple { x: 5.37685424949238, y: 3.85685424949238, z: 0.0, w: 1.0 }
Tick 9 - Position Tuple { x: 6.003961030678928, y: 3.7639610306789275, z: 0.0, w: 1.0 }
Tick 10 - Position Tuple { x: 6.621067811865475, y: 3.571067811865475, z: 0.0, w: 1.0 }
Tick 11 - Position Tuple { x: 7.228174593052023, y: 3.2781745930520225, z: 0.0, w: 1.0 }
Tick 12 - Position Tuple { x: 7.82528137423857, y: 2.88528137423857, z: 0.0, w: 1.0 }
Tick 13 - Position Tuple { x: 8.412388155425118, y: 2.3923881554251176, z: 0.0, w: 1.0 }
Tick 14 - Position Tuple { x: 8.989494936611665, y: 1.7994949366116653, z: 0.0, w: 1.0 }
Tick 15 - Position Tuple { x: 9.556601717798213, y: 1.1066017177982128, z: 0.0, w: 1.0 }
Tick 16 - Position Tuple { x: 10.11370849898476, y: 0.3137084989847604, z: 0.0, w: 1.0 }
Tick 17 - Position Tuple { x: 10.660815280171308, y: -0.579184719828692, z: 0.0, w: 1.0 }
```

[ray-tracer-challenge]: http://www.raytracerchallenge.com/
