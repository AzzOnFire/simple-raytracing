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
    pub fn new(lookfrom: Vec3<T>, lookat: Vec3<T>, fov: T, aspect_ratio: T) -> Self where f64: Into<T> {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0.into()).tan();
        let half_width = aspect_ratio * half_height;

        let w = *(lookfrom + lookat).normalize();
        let vup = Vec3::new(0.0.into(), -1.0.into(), 0.0.into());
        let u = *vup.cross(&w).normalize();
        let v = w.cross(&u);

        Self { 
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * 2.0.into() * half_width,
            vertical: v * 2.0.into() * half_height,
        }
    }

    pub fn get_ray(&self, u: T, v: T) -> Ray<T> {
        Ray::new(self.origin, -(self.lower_left_corner + self.horizontal * u + self.vertical * v))
    }
}