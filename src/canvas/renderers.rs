use crate::canvas::Canvas;

const PPM_VERSION: &str = "P3";
const PPM_COLOR_MAX: i16 = 255;
const PPM_MAX_LINE_LENGTH: i16 = 70;

/// Render a canvas as a PPM image.
///
/// A PPM image is very simple to write as it consists of numeric triples to
/// describe the red, green, and blue components of each pixel. The major
/// downside is that the file size of a PPM image will be much greater than a
/// standard JPG or PNG file.
///
/// # Arguments
///
/// * `canvas` - The canvas to render.
/// * `dest` - The ouput buffer to write the image data to.
pub fn render_as_ppm<T: std::io::Write>(
    canvas: &Canvas,
    mut dest: T,
) -> Result<(), std::io::Error> {
    // All PPM files start with a identifier which in our case.
    writeln!(dest, "{}", PPM_VERSION)?;

    // The next lines give the width and height of the image.
    writeln!(dest, "{} {}", canvas.width(), canvas.height())?;

    // The last line of the header indicates the maximum color value. In our
    // case, we scale colors between 0 and 255.
    writeln!(dest, "{}", PPM_COLOR_MAX)?;

    // Write each row of pixels as a triple of scaled color values. The spec for
    // PPM files indicates lines should not exceed 70 characters, so we insert
    // line breaks between numbers as necessary.
    for y in 0..canvas.height() {
        let mut column = 0;
        for x in 0..canvas.width() {
            let pixel = canvas.pixel_at(x, y);
            let pixel_values = vec![pixel.red(), pixel.green(), pixel.blue()];

            for color_value in pixel_values.iter() {
                let scaled_value = scale_color_value(*color_value).to_string();
                let value_length = scaled_value.chars().count();

                // If writing the value would cause us to exceed our line length
                // limit, start a new line.
                if column + value_length >= PPM_MAX_LINE_LENGTH as usize {
                    writeln!(dest)?;
                    column = 0;
                }

                if column == 0 {
                    write!(dest, "{}", scaled_value)?;
                    column += value_length;
                } else {
                    // Any values written after the first need a preceding
                    // space.
                    write!(dest, " {}", scaled_value)?;
                    column += value_length + 1;
                }
            }
        }

        writeln!(dest)?;
    }

    Ok(())
}

fn scale_color_value(value: f64) -> i64 {
    ((PPM_COLOR_MAX as f64 * value).round() as i64).clamp(0, PPM_COLOR_MAX as i64)
}
