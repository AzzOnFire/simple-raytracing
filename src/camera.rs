use crate::Vec3;
use crate::Ray;

use num::Float;


#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Camera<T: Float> {
    origin: Vec3<T>,
    lower_left_corner: Vec3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>
}

impl<T: Float> Camera<T> {
    pub fn new(/*origin: Vec3<T>, direction: Vec3<T>, */fov: T, aspect_ratio: T) -> Self where f64: Into<T> {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0.into()).tan();
        let half_width = aspect_ratio * half_height;

        // TODO camera rotation
        /*let direction = *direction.normalize() * -1.0.into();
        let world_up_direction = Vec3::new(0.0.into(), 0.0.into(), 1.0.into());*/

        Self { 
            origin: Vec3::new(0.0.into(), 0.0.into(), 0.0.into()),
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0.into()),
            horizontal: Vec3::new(2.0.into() * half_width, 0.0.into(), 0.0.into()),
            vertical: Vec3::new(0.0.into(), 2.0.into() * half_height, 0.0.into()),
        }
    }

    pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v)
    }
}