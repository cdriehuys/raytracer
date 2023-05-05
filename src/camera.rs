use rayon::prelude::*;

use crate::{
    canvas::Canvas,
    linear::{Matrix, Tuple},
    Ray, World,
};

/// Create a transformation matrix describing how the camera should be
/// positioned in the world.
///
/// The transform is really a description of how the world moves around the
/// camera, but it is more intuitive to think of moving the camera around the
/// world, so the resulting transform will be the inverse of what is expected.
///
/// # Arguments
///
/// * `from` - Where the camera is located in the world.
/// * `to` - The location that the camera is pointing at.
/// * `up` - A vector identifying which direction is up.
///
/// # Examples
///
/// The default transformation is located at the origin, pointing along the
/// negative z-axis with `up` being the positive y-axis.
///
/// ```
/// # use raytracer::{camera::view_transform, linear::{Matrix, Tuple}};
/// let from = Tuple::new_point(0, 0, 0);
/// let to = Tuple::new_point(0, 0, -1);
/// let up = Tuple::new_vector(0, 1, 0);
///
/// let transform = view_transform(&from, &to, &up);
///
/// assert_eq!(transform, Matrix::identity_4());
/// ```
pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (*to - *from).normalized();
    let left = forward.cross(up.normalized());
    let true_up = left.cross(forward);

    #[rustfmt::skip]
    let orientation = Matrix::square_4(
        left.x(), left.y(), left.z(), 0.0,
        true_up.x(), true_up.y(), true_up.z(), 0.0,
        -forward.x(), -forward.y(), -forward.z(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    &orientation * &Matrix::translation(-from.x(), -from.y(), -from.z())
}

/// A camera describes an observer of a 3D scene.
#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f64,
    transform: Matrix,

    // Computed properties used when casting rays.
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    /// Construct a new camera.
    ///
    /// # Arguments
    ///
    /// * `hsize` - The horizontal size (in pixels) of the camera's output.
    /// * `vsize` - The vertical size (in pixels) of the camera's output.
    /// * `fov` - The camera's field of view expressed in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::{camera::Camera, linear::Matrix};
    /// let hsize = 160;
    /// let vsize = 120;
    /// let fov = std::f64::consts::FRAC_PI_2;
    ///
    /// let c = Camera::new(hsize, vsize, fov);
    ///
    /// assert_eq!(c.hsize(), hsize);
    /// assert_eq!(c.vsize(), vsize);
    /// assert_eq!(c.fov(), fov);
    /// assert_eq!(*c.transform(), Matrix::identity_4());
    /// ```
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        // Compute the pixel size in advance.
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity_4(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn with_transform(&self, transform: Matrix) -> Self {
        Self { transform, ..*self }
    }

    /// Retrieve the size of a pixel in world space units based on the camera's
    /// attributes.
    ///
    /// # Examples
    ///
    /// For a horizontal canvas:
    ///
    /// ```
    /// # use raytracer::camera::Camera;
    /// let c = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);
    ///
    /// assert!( float_cmp::approx_eq!(f64, c.pixel_size(), 0.01) );
    /// ```
    ///
    /// For a vertical canvas:
    ///
    /// ```
    /// # use raytracer::camera::Camera;
    /// let c = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);
    ///
    /// assert!( float_cmp::approx_eq!(f64, c.pixel_size(), 0.01) );
    /// ```
    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    /// Get a ray pointing from the camera's location towards the world space
    /// location of the specified pixel.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel to get a ray for.
    /// * `y` - The y-coordinate of the pixel to get a ray for.
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform = self.transform.inverted();
        let pixel = &transform * Tuple::new_point(world_x, world_y, -1.0);
        let origin = &transform * Tuple::new_point(0, 0, 0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    /// Render a world using the camera.
    ///
    /// # Arguments
    ///
    /// * `world` - The world to render.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raytracer::{camera::{Camera, view_transform}, linear::Tuple, World};
    /// // Build up a world with objects (or use the default).
    /// let world = World::default();
    ///
    /// // Position the camera
    /// let from = Tuple::new_point(0, 0, -5);
    /// let to = Tuple::new_point(0, 0, 0);
    /// let up = Tuple::new_vector(0, 1, 0);
    ///
    /// let camera = Camera::new(11, 11, std::f64::consts::FRAC_PI_2)
    ///     .with_transform(view_transform(&from, &to, &up));
    ///
    /// // Render the world to a canvas.
    /// let _image = camera.render(&world);
    /// ```
    pub fn render(&self, world: &World) -> Canvas {
        let pixels = (0..self.hsize)
            .into_par_iter()
            .map(|x| {
                (0..self.vsize)
                    .into_par_iter()
                    .map(move |y| {
                        let ray = self.ray_for_pixel(x, y);
                        world.color_at(&ray)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Canvas::from_pixels(pixels)
    }
}
