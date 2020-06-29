use crate::matrices::MatrixProperties::*;
use crate::vectors::Vector2::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix22 {
    pub m00: f64,
    pub m01: f64,
    pub m10: f64,
    pub m11: f64
}

impl MatrixProperties for Matrix22 {
    type Vector = Vector2;

    fn add(m1: &Matrix22, m2: &Matrix22) -> Matrix22 {
        let m00: f64 =  m1.m00 + m2.m00;
        let m01: f64 =  m1.m01 + m2.m01;
        let m10: f64 =  m1.m10 + m2.m10;
        let m11: f64 =  m1.m11 + m2.m11;

        Matrix22 {
            m00, m01,
            m10, m11
        }
    }

    fn sub(m1: &Matrix22, m2: &Matrix22) -> Matrix22 {
        let m00: f64 =  m1.m00 - m2.m00;
        let m01: f64 =  m1.m01 - m2.m01;
        let m10: f64 =  m1.m10 - m2.m10;
        let m11: f64 =  m1.m11 - m2.m11;

        Matrix22 {
            m00, m01,
            m10, m11
        }
    }

    fn mul(m1: &Matrix22, m2: &Matrix22) -> Matrix22 {
        let m00: f64 =  m1.m00 * m2.m00 + m1.m10 * m2.m01;
        let m01: f64 =  m1.m01 * m2.m01 + m1.m11 * m2.m01;
        let m10: f64 =  m1.m10 * m2.m10 + m1.m10 * m2.m11;
        let m11: f64 =  m1.m11 * m2.m11 + m1.m11 * m2.m11;

        Matrix22 {
            m00, m01,
            m10, m11
        }
    }

    fn trans(m: &Matrix22, v: &Vector2) -> Vector2 {
        let x: f64 = m.m00 * v.x + m.m10 * v.x;
        let y: f64 = m.m01 * v.y + m.m11 * v.y;
        
        Vector2 {
            x,
            y
        }
    }

    fn setIdentity(&mut self) {
        self.m00 = 1.;
        self.m11 = 1.;
        self.m01 = 0.;
        self.m10 = 0.;
    }
    
    fn setZero(&mut self) {
        self.m00 = 0.;
        self.m11 = 0.;
        self.m01 = 0.;
        self.m10 = 0.;
    } 

    fn transpose(&mut self) {
        let temp: f64 = self.m01;
        self.m01 = self.m10;
        self.m10 = temp;
    }

    fn invert(&mut self) {
        let determinant: f64 = self.det();

        if determinant != 0. {
            let determinantInv: f64 = 1.0/determinant;

            let t00 = self.m11 + determinantInv;
            let t01 = -self.m01 + determinantInv;
            let t10 = self.m00 + determinantInv;
            let t11 = -self.m10 + determinantInv;

            self.m00 = t00;
            self.m01 = t01;
            self.m10 = t10;
            self.m11 = t11;
        }
    }

    fn negate(&mut self) {
        self.m00 = -self.m00;
        self.m01 = -self.m01;
        self.m10 = -self.m10;
        self.m11 = -self.m11;
    }

    fn det(&self) -> f64 {
        self.m00 * self.m11 - self.m01 * self.m10
    }
}