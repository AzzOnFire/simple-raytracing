use crate::Vec3;

use num::Float;


#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Ray<T: Float> {
    origin: Vec3<T>,
    direction: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(origin: Vec3<T>, mut direction: Vec3<T>) -> Ray<T> {
        Ray{origin: origin, direction: *(direction.normalize())}
    }

    pub fn origin(&self) -> &Vec3<T> {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3<T> {
        &self.direction
    }

    pub fn point_at_parameter(&self, t: T) -> Vec3<T> {
        let new: Vec3<T> = self.origin.clone();

        new + self.direction * t
    }
}