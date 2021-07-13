use std::ops::{Add, Mul, Sub};
use std::clone::Clone;
use num::Float;

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

    pub fn length(vector: & Self) -> T {
        (vector.x.exp2() + vector.y.exp2() + vector.z.exp2()).sqrt()
    }

    pub fn normalize(&mut self) -> &Self {
        let length = Self::length(self);

        self.x = self.x / length;
        self.y = self.y / length; 
        self.z = self.z / length; 
        
        self
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z 
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
