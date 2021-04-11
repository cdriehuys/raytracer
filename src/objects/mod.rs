mod base_shape;
mod object_id;
mod plane;
mod shape;
mod sphere;

// Pull in the test shape only if running tests. It exercises the ability to
// delegate operations to the base shape.
#[cfg(test)]
mod test_shape;

// Allow other shapes to utilize the base shape, but don't expose it publicly.
use base_shape::BaseShape;
use object_id::ObjectID;

pub use plane::Plane;
pub use shape::Shape;
pub use sphere::Sphere;
