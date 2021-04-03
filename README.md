# raytracer-rs

[![Test](https://github.com/cdriehuys/raytracer/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/cdriehuys/raytracer/actions/workflows/ci.yml)

A ray tracer written in Rust, built by following
[*The Ray Tracer Challenge* by Jamis Buck][ray-tracer-challenge].

## Playground

At the end of each chapter, the new capabilities of the ray tracer are utilized
in the main program. It can be executed with:

```bash
# Using the release profile instead of the default debug profile has a massive
# effect on performance.
cargo run --release
```

### Current Behavior (Ch. 5)

Running the application produces the silhouette of a sphere. The sphere is a
unit-sphere located at the origin. Rays are cast from a point in space towards a
"wall" behind the sphere. Locations on the wall are colored red if the ray hits
the sphere. This produces an image like:

![Ray traced sphere](https://user-images.githubusercontent.com/4708504/113485288-99eb3800-9461-11eb-87ba-c232ccf0587b.png)

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

#### Matrix Transforms (Ch. 4)

The next major addition was matrix transformations which allows us to use a
consistent operation (matrix multiplication) to perform arbitrary transforms of
objects. For example, the hour markers on a clock can be drawn by applying
rotations to a single hour marker as shown in the image:

![raytracer clock](https://user-images.githubusercontent.com/4708504/113330377-80b48100-92d3-11eb-9b9e-b1d1498bd6c4.png)

</details>

## License

This project is licensed under the [MIT License](LICENSE).

[ray-tracer-challenge]: http://www.raytracerchallenge.com/
