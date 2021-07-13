use crate::Vec3;
use crate::Ray;
use crate::objects::{Objects, Sphere};

use num::Float;


#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Scene<T: Float> {
    objects: Vec<Objects<T>>,
}

impl<T: Float> Scene<T> {
    pub fn new() -> Self {
        Self { objects: vec!() }
    }

    pub fn add_demo_sphere(&mut self) where f64: Into<T> {
        let sphere = Sphere::new(Vec3::new(0.0.into(), 0.0.into(), -1.0.into()), 0.5.into());
        let sphere_object = Objects::Sphere(sphere);
        self.objects.push(sphere_object);
    }

    pub fn color(&self, ray: &Ray<T>) -> Vec3<T> where f64: Into<T> {
        for object in &self.objects {
            match object {
                Objects::Sphere(sphere) => { 
                    if let Some(t) = sphere.hit(ray) {
                        let point = ray.point_at_parameter(t) - sphere.center;

                        return Vec3::new(point.x + 1.0.into(), point.y + 1.0.into(), point.z + 1.0.into()) * 0.5.into()
                    }
                },
                // _ => false, 
            }; 
    
        }

        let t = 0.5.into() * (ray.direction().y + 1.0.into());

        Vec3::new(1.0.into(), 1.0.into(), 1.0.into()) * (1.0.into() - t) + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t 
    }
}