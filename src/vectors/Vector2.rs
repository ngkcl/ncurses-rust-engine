use crate::vectors::VectorProperties::*;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {
            x,
            y
        }
    }
}

impl VectorProperties for Vector2 {
    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    fn add(vec1: &Vector2, vec2: &Vector2) -> Vector2 {
        Vector2 {
            x: vec1.x + vec2.x,
            y: vec1.y + vec2.y
        }
    }
    
    fn sub(vec1: &Vector2, vec2: &Vector2) -> Vector2 {
        Vector2 {
            x: vec1.x - vec2.x,
            y: vec1.y - vec2.y
        }
    }

    fn mul(vec1: &Vector2, vec2: &Vector2) -> Vector2 {
        Vector2 {
            x: vec1.x * vec2.x,
            y: vec1.y * vec2.y 
        }
    }

    fn lerp(vec1: &Vector2, vec2: &Vector2, t: f64) -> Vector2 {
        let x: f64 = vec1.x + ((vec2.x - vec1.x) * t);
        let y: f64 = vec1.y + ((vec2.y - vec1.y) * t);

        Vector2 {
            x,
            y
        }
    }


    fn dot(vec1: &Vector2, vec2: &Vector2) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }

    fn angle(vec1: &Vector2, vec2: &Vector2) -> f64 {
        let dot = Vector2::dot(&vec1, &vec2);
        let f = dot/(vec1.length() * vec2.length());

        return f.acos()
    }

    fn dist(vec1: &Vector2, vec2: &Vector2) -> f64 {
        let x: f64 = vec1.x - vec2.x;
        let y: f64 = vec1.y - vec2.y;

        return (x*x + y*y).sqrt()
    }

    fn getNormalized(v: &Vector2) -> Vector2 {
        let mut f: Vector2 = *v;
        f.normalize();
        return f;
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    fn normalize(&mut self) {
        let mag = self.length();

        self.x /= mag;
        self.y /= mag;
    }
}

// Operator overloading
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        <Vector2 as VectorProperties>::add(&self, &rhs)
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = Vector2::add(&self, &rhs);
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        <Vector2 as VectorProperties>::sub(&self, &rhs)
    }
}

impl ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        *self = Vector2::sub(&self, &rhs)
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Vector2) -> f64 {
        Vector2::dot(&self, &rhs)
    }
}