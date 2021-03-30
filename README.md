# raytracer-rs

A ray tracer written in Rust, built by following
[*The Ray Tracer Challenge* by Jamis Buck][ray-tracer-challenge].

## Playground

At the end of each chapter, the new capabilities of the ray tracer are utilized
in the main program. It can be executed with:

```bash
cargo run
```

The current behavior is to track projectile motion to utilize vector math. The
projectile's motion is plotted to demonstrate the use of a canvas, which is then
rendered to a PPM image file. The output looks like:

![projectile motion plot](https://user-images.githubusercontent.com/4708504/112875950-b6f6c400-9079-11eb-9832-253ac0f83d82.png)

## License

This project is licensed under the [MIT License](LICENSE).

[ray-tracer-challenge]: http://www.raytracerchallenge.com/
