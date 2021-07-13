use crate::Vec3;
use crate::Ray;

use num::Float;


#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Sphere<T: Float> {
    center: Vec3<T>,
    radius: T,
}

impl<T: Float> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self { center: center, radius: radius }
    }
    
    pub fn hit(&self, ray: &Ray<T>) -> bool where f64: Into<T> {
        let difference = ray.origin().clone() - self.center;

        let a = ray.direction().dot(ray.direction()); 
        let b = 2.0.into() * ray.direction().dot(&difference);
        let c = difference.dot(&difference) - self.radius * self.radius;

        let discriminant = b * b - 4.0.into() * a * c;
        
        discriminant.is_sign_positive()
    }
}


#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub enum Objects<T: Float> {
    Sphere(Sphere<T>),
}