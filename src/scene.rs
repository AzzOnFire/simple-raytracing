use crate::Vec3;
use crate::{Ray, Intersection};
use crate::material::*;
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

        let sphere = Sphere::new(Vec3::new(0.0.into(), -10.5.into(), -1.0.into()), 10.0.into());
        let sphere_object = Objects::Sphere(sphere);
        self.objects.push(sphere_object);
    }

    pub fn color(&self, ray: &Ray<T>, depth: u8) -> Vec3<T> where f64: Into<T> {
        if depth < 128 {
            let mut closest = Float::max_value();
            let mut intersection: Option<Intersection<T>> = None;        
    
            for object in &self.objects {
                match object {
                    Objects::Sphere(sphere) => { 
                        if let Some(result) = sphere.hit(ray, 0.001.into(), closest) {
                            closest = result.t;
                            intersection = Some(result);
                        }
                    },
                    // _ => false, 
                }; 
            }
    
            match intersection {
                Some(result) => {
                    let target: Vec3<T> = result.point + result.normal + random_in_unit_sphere();
                    //println!("hit");
                    return self.color(&Ray::new(result.point, target - result.point), depth + 1) * 0.5.into();
                    //return Vec3::new(result.normal.x + 1.0.into(), result.normal.y + 1.0.into(), result.normal.z + 1.0.into()) * 0.5.into();
                },
                None => {}
            }
        }
       
        let t = 0.5.into() * (ray.direction().y + 1.0.into());

        Vec3::one() * (1.0.into() - t) + Vec3::new(0.5.into(), 0.7.into(), 1.0.into()) * t 
    }
}