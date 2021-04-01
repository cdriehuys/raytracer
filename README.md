# raytracer-rs

[![Test](https://github.com/cdriehuys/raytracer/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/cdriehuys/raytracer/actions/workflows/ci.yml)

A ray tracer written in Rust, built by following
[*The Ray Tracer Challenge* by Jamis Buck][ray-tracer-challenge].

## Playground

At the end of each chapter, the new capabilities of the ray tracer are utilized
in the main program. It can be executed with:

```bash
cargo run
```

### Current Behavior (Ch. 4)

The raytracer currently creates the hour marks on a clock face to utilize the
new transformation matrices. The output looks like:

![raytracer clock](https://user-images.githubusercontent.com/4708504/113330377-80b48100-92d3-11eb-9b9e-b1d1498bd6c4.png)

### History

Expand the section below to see a progression of the project. These milestones
generally correspond to the exercises at the end of each chapter in the book
which utilize the new functionality added in the chapter.

<details>
  <summary>Expand to see progression</summary>

#### Canvas Usage (Ch. 2)

This was the first visual output from the project. It exercised basic tuple math
to compute the trajectory of a projectile and used the new canvas to plot the
projectile's position over time.

![projectile motion plot](https://user-images.githubusercontent.com/4708504/112875950-b6f6c400-9079-11eb-9832-253ac0f83d82.png)

</details>

## License

This project is licensed under the [MIT License](LICENSE).

[ray-tracer-challenge]: http://www.raytracerchallenge.com/
