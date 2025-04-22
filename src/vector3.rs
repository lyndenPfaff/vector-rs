//! Simple 3 dimentional vector
//! 
//! provides a struct of 3 64-bit floating point values for the x, y, and z components
//! of a mathematical vector, along with the standard set of vector operations.
//! 
//! This submodule is re-exported as vector::Vector3 in the public API

/* --- IMPORTS --- */

use std::ops::{ Add, Mul, Sub };

/* --- ------- --- */



/* --- STRUCTS --- */

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* --- ------- --- */



/* --- IMPLEMENTATIONS --- */

impl Vector3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }


    pub fn dot(&self, b: Vector3) -> f64 {
        self.x*b.x + self.y*b.y + self.z*b.z
    }


    pub fn cross(&self, b: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }


    pub fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }


    pub fn normalize(&self) -> Vector3 {
        let len = self.length();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

}



impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as f64
    }
}

/* --- --------------- --- */
