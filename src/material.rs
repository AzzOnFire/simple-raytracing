use crate::Vec3;
use num::Float;

pub fn random_in_unit_sphere<T: Float>() -> Vec3<T> where f64: Into<T> {
    let one = Vec3::one();

    let mut result;

    loop {
        result = Vec3::rand() * 2.0.into() - one;
        
        if result.squared_length() <= 1.0.into() {
            break;
        }
    }

    
    result
}