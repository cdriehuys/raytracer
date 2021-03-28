use raytracer::canvas::{renderers::render_as_ppm, Canvas};
use raytracer::colors::Color;

#[test]
fn ppm_header() {
    let c = Canvas::new(5, 3);
    let mut output = Vec::new();

    match render_as_ppm(&c, &mut output) {
        Err(err) => panic!(err),
        _ => (),
    };

    let string_output = String::from_utf8(output).unwrap();

    let want_header = "P3
5 3
255
";

    assert!(
        string_output.starts_with(want_header),
        "Should start with:\n{}\n\nGot:\n{}",
        want_header,
        string_output
    );
}

#[test]
fn ppm_body() {
    let mut c = Canvas::new(5, 3);

    let c1 = Color::new(1.5, 0, 0);
    let c2 = Color::new(0, 0.5, 0);
    let c3 = Color::new(-0.5, 0, 1);

    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);

    let want_pixels = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";

    let mut output = Vec::new();
    match render_as_ppm(&c, &mut output) {
        Err(err) => panic!(err),
        _ => (),
    }

    let string_output = String::from_utf8(output).unwrap();

    assert!(
        string_output.contains(want_pixels),
        "Should contain:\n{}\n\nGot:\n{}",
        want_pixels,
        string_output,
    );
}

#[test]
fn ppm_wrap_lines() {
    let mut canvas = Canvas::new(10, 2);
    let color = Color::new(1, 0.8, 0.6);

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            canvas.write_pixel(x, y, color);
        }
    }

    let want_pixels = "
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"
        .trim();

    let mut output = Vec::new();
    match render_as_ppm(&canvas, &mut output) {
        Err(err) => panic!(err),
        _ => (),
    }

    let string_output = String::from_utf8(output).unwrap();

    assert!(
        string_output.contains(want_pixels),
        "Should contain:\n{}\n\nGot:\n{}",
        want_pixels,
        string_output
    );
}

#[test]
fn ppm_ends_with_newline() {
    let canvas = Canvas::new(5, 3);
    let mut output = Vec::new();

    match render_as_ppm(&canvas, &mut output) {
        Err(err) => panic!(err),
        _ => (),
    };

    let string_output = String::from_utf8(output).unwrap();

    assert!(
        string_output.ends_with("\n"),
        "Output does not end with a newline."
    );
}
