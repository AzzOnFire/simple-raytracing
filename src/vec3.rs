use std::ops::{Add, Mul, Div, Sub, Neg};
use std::fmt;
use std::clone::Clone;
use num::Float;
use rand::prelude::*;

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vec3<T: Float> {
   pub x: T,
   pub y: T,
   pub z: T,
}

impl<T: Float> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }

    pub fn zero() -> Self where f64: Into<T> {
        Self{ x: 0.0.into(), y: 0.0.into(), z: 0.0.into() }
    }

    pub fn one() -> Self where f64: Into<T> {
        Self{ x: 1.0.into(), y: 1.0.into(), z: 1.0.into() }
    }

    pub fn rand() -> Self where f64: Into<T> {
        let mut rng = rand::thread_rng();

        Self { 
            x: rng.gen::<f64>().into(),
            y: rng.gen::<f64>().into(),
            z: rng.gen::<f64>().into(),
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) -> &Self {
        let length = self.length();

        self.x = self.x / length;
        self.y = self.y / length; 
        self.z = self.z / length; 
        
        self
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z 
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,  
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        ) 

        /*Vec3::new(
            self.z * other.y - self.y * other.z,  
            self.x * other.z - self.z * other.x,
            self.y * other.x - self.x * other.y
        )*/
    }
}

impl<T: Float> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x, 
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.x.to_f64() {
            Some(x) => { 
                match self.y.to_f64() {
                    Some(y) => { 
                        match self.z.to_f64() {
                            Some(z) => { 
                                write!(f, "({}, {}, {})", x, y, z)
                            },
                            _ => write!(f, "bad"),
                        }
                    },
                    _ => write!(f, "bad")
                } 
            },
            _ => write!(f, "bad")
        }
    }
}

impl<T: Float> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

macro_rules! left_multiplication {
    ($($T:ty),*) => {
        $(
            impl Mul<Vec3<$T>> for $T {
                type Output = Vec3<$T>;
                fn mul(self, rhs: Self::Output) -> Self::Output {
                    Self::Output {x : rhs.x * self, y : rhs.y * self, z : rhs.z * self}
                }
            }
        )*
    }
}

left_multiplication!{
    f32, f64
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}
