use raytracer::canvas::Canvas;
use raytracer::Color;

#[test]
fn create_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);

    let default_color = Color::new(0, 0, 0);

    for x in 0..10 {
        for y in 0..20 {
            assert_eq!(c.pixel_at(x, y), default_color);
        }
    }
}

#[test]
fn write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1, 0, 0);

    c.write_pixel(2, 3, red);

    assert_eq!(c.pixel_at(2, 3), red);
}
