use crate::vectors::VectorProperties::*;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn cross(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
        let x: f64 = (vec1.y * vec2.z) - (vec1.z * vec2.y);
        let y: f64 = (vec1.z * vec2.x) - (vec1.x * vec2.z);
        let z: f64 = (vec1.x * vec2.y) - (vec1.y * vec2.x);

        Vector3 {
            x,
            y,
            z
        }
    }
}

impl VectorProperties for Vector3 {
    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn add(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
        Vector3 {
            x: vec1.x + vec2.x,
            y: vec1.y + vec2.y,
            z: vec1.z + vec2.z
        }
    }
    
    fn sub(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
        Vector3 {
            x: vec1.x - vec2.x,
            y: vec1.y - vec2.y,
            z: vec1.z - vec2.z
        }
    }

    fn mul(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
        Vector3 {
            x: vec1.x * vec2.x,
            y: vec1.y * vec2.y,
            z: vec1.z * vec2.z
        }
    }

    fn lerp(vec1: &Vector3, vec2: &Vector3, t: f64) -> Vector3 {
        let x: f64 = vec1.x + ((vec2.x - vec1.x) * t);
        let y: f64 = vec1.y + ((vec2.y - vec1.y) * t);
        let z: f64 = vec1.z + ((vec2.z - vec1.z) * t);

        Vector3 {
            x,
            y,
            z
        }
    }

    fn dot(vec1: &Vector3, vec2: &Vector3) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    fn angle(vec1: &Vector3, vec2: &Vector3) -> f64 {
        let dot = Vector3::dot(&vec1, &vec2);
        let f = dot/(vec1.length() * vec2.length());

        return f.acos()
    }

    fn dist(vec1: &Vector3, vec2: &Vector3) -> f64 {
        let x: f64 = vec1.x - vec2.x;
        let y: f64 = vec1.y - vec2.y;
        let z: f64 = vec1.z - vec2.z;

        return (x*x + y*y + z*z).sqrt()
    }

    fn getNormalized(v: &Vector3) -> Vector3 {
        let mut f: Vector3 = *v;
        f.normalize();
        return f;
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    fn normalize(&mut self) {
        let mag = self.length();

        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z)
    }
}

// Operator overloading
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        <Vector3 as VectorProperties>::add(&self, &rhs)
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = Vector3::add(&self, &rhs);
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        <Vector3 as VectorProperties>::sub(&self, &rhs)
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = Vector3::sub(&self, &rhs)
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Vector3) -> f64 {
        Vector3::dot(&self, &rhs)
    }
}

impl ops::Rem<Vector3> for Vector3 {
    type Output = Vector3;

    fn rem(self, rhs: Vector3) -> Vector3 {
        Vector3::cross(&self, &rhs)
    }
}

impl ops::RemAssign<Vector3> for Vector3 {
    fn rem_assign(&mut self, rhs: Vector3) {
        *self = Vector3::cross(&self, &rhs)
    }
}