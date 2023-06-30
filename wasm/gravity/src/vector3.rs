use wasm_bindgen::prelude::*;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign};

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x:f64, y:f64, z:f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Vector3 {
    pub fn magnitude_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_sqr().sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;     
        self.y /= mag;
        self.z /= mag;
    }

    pub fn normalized(&self) -> Vector3 {
        Vector3 {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        } 
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        } 
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        } 
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        } 
    }
}
