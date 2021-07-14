use crate::Vec3;
use crate::{Ray, Intersection};

use num::Float;


#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Sphere<T: Float> {
    pub center: Vec3<T>,
    pub radius: T,
}

impl<T: Float> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self { center: center, radius: radius }
    }
    
    pub fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Intersection<T>> where f64: Into<T> {
        let difference = ray.origin().clone() - self.center;
    
        let a = ray.direction().dot(ray.direction()); 
        let b = ray.direction().dot(&difference);
        let c = difference.dot(&difference) - self.radius * self.radius;

        let discriminant = b * b - a * c;        
        
        if discriminant.is_sign_positive() {
            let t = (-b - discriminant.sqrt()) / a;
            
            if t_min < t && t < t_max {
                let point = ray.point_at_parameter(t); 

                return Some(Intersection {
                    t: t,
                    point: point,
                    normal: (point - self.center) * (1.0.into() / self.radius)
                })
            } 

            let t = (-b + discriminant.sqrt()) / a;
            
            if t_min < t && t < t_max {
                let point = ray.point_at_parameter(t); 

                return Some(Intersection {
                    t: t,
                    point: point,
                    normal: (point - self.center) * (1.0.into() / self.radius)
                })
            } 
        } 

        None
    }
}


#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub enum Objects<T: Float> {
    Sphere(Sphere<T>),
}