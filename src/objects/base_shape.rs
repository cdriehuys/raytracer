use crate::{
    intersections::Intersections,
    linear::{Matrix, Tuple},
    Material, Ray,
};

use super::Shape;

#[derive(Clone, Debug)]
pub struct BaseShape {
    material: Material,
    transform: Matrix,
}

impl Default for BaseShape {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transform: Matrix::identity_4(),
        }
    }
}

impl Shape for BaseShape {
    fn base_shape(&self) -> &BaseShape {
        unimplemented!("BaseShape should never be used directly. It should be delegated to by a concrete shape.")
    }

    fn base_shape_mut(&mut self) -> &mut BaseShape {
        unimplemented!("BaseShape should never be used directly. It should be delegated to by a concrete shape.")
    }

    fn intersect_local(&self, _ray: &Ray) -> Intersections {
        unimplemented!("BaseShape should never be used directly. It should be delegated to by a concrete shape.")
    }

    fn normal_at_local(&self, _point: &Tuple) -> Tuple {
        unimplemented!("BaseShape should never be used directly. It should be delegated to by a concrete shape.")
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}
