use super::colors::Color;

/// A canvas contains a 2D array of pixels.
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,

    pixels: Vec<Vec<Color>>
}

impl Canvas {
    /// Construct a new canvas of the given size. The canvas will be filled with
    /// black pixels by default.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the canvas.
    /// * `height` - The height of the canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::canvas::Canvas;
    /// let c = Canvas::new(10, 20);
    ///
    /// assert_eq!(c.width(), 10);
    /// assert_eq!(c.height(), 20);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        let mut columns = Vec::with_capacity(width);
        for _ in 0..width {
            let row = vec![Color::new(0, 0, 0); height];
            columns.push(row);
        }

        Self{
            width,
            height,
            pixels: columns
        }
    }

    /// Get the canvas' width in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the canvas' height in pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the pixel at a specific location on the canvas.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel to retrieve.
    /// * `y` - The y-coordinate of the pixel to retrieve.
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    /// Set a specific pixel to the given color.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel to write.
    /// * `y` - The y-coordinate of the pixel to write.
    /// * `color` - The color to write to the given location.
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color;
    }
}
