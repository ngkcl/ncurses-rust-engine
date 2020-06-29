use crate::matrices::MatrixProperties::*;
use crate::vectors::Vector3::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix33 {
    pub m00: f64, 
    pub m01: f64, 
    pub m02: f64,

    pub m10: f64, 
    pub m11: f64, 
    pub m12: f64,

    pub m20: f64, 
    pub m21: f64, 
    pub m22: f64
}

impl MatrixProperties for Matrix33 {
    type Vector = Vector3;

    fn add(m1: &Matrix33, m2: &Matrix33) -> Matrix33 {
        let m00: f64 =  m1.m00 + m2.m00;
        let m01: f64 =  m1.m01 + m2.m01;
        let m02: f64 =  m1.m02 + m2.m02;

        let m10: f64 =  m1.m10 + m2.m10;
        let m11: f64 =  m1.m11 + m2.m11;
        let m12: f64 =  m1.m12 + m2.m12;

        let m20: f64 =  m1.m20 + m2.m20;
        let m21: f64 =  m1.m21 + m2.m21;
        let m22: f64 =  m1.m22 + m2.m22;

        Matrix33 {
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22
        }
    }

    fn sub(m1: &Matrix33, m2: &Matrix33) -> Matrix33 {
        let m00: f64 =  m1.m00 - m2.m00;
        let m01: f64 =  m1.m01 - m2.m01;
        let m02: f64 =  m1.m02 - m2.m02;

        let m10: f64 =  m1.m10 - m2.m10;
        let m11: f64 =  m1.m11 - m2.m11;
        let m12: f64 =  m1.m12 - m2.m12;

        let m20: f64 =  m1.m20 - m2.m20;
        let m21: f64 =  m1.m21 - m2.m21;
        let m22: f64 =  m1.m22 - m2.m22;

        Matrix33 {
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22
        }
    }

    fn mul(m1: &Matrix33, m2: &Matrix33) -> Matrix33 {
        let m00: f64 = m1.m00 * m2.m00 + m1.m10 * m2.m01 + m1.m20 * m2.m02;
        let m01: f64 = m1.m01 * m2.m00 + m1.m11 * m2.m01 + m1.m21 * m2.m02;
        let m02: f64 = m1.m02 * m2.m00 + m1.m12 * m2.m01 + m1.m22 * m2.m02;
        let m10: f64 = m1.m00 * m2.m10 + m1.m10 * m2.m11 + m1.m20 * m2.m12;
        let m11: f64 = m1.m01 * m2.m10 + m1.m11 * m2.m11 + m1.m21 * m2.m12;
        let m12: f64 = m1.m02 * m2.m10 + m1.m12 * m2.m11 + m1.m22 * m2.m12;
        let m20: f64 = m1.m00 * m2.m20 + m1.m10 * m2.m21 + m1.m20 * m2.m22;
        let m21: f64 = m1.m01 * m2.m20 + m1.m11 * m2.m21 + m1.m21 * m2.m22;
        let m22: f64 = m1.m02 * m2.m20 + m1.m12 * m2.m21 + m1.m22 * m2.m22;


        Matrix33 {
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22
        }
    }

    fn trans(m: &Matrix33, v: &Vector3) -> Vector3 {
        let x: f64 = m.m00 * v.x + m.m10 * v.y + m.m20 * v.z;
        let y: f64 = m.m01 * v.x + m.m11 * v.y + m.m21 * v.z;
        let z: f64 = m.m02 * v.x + m.m12 * v.y + m.m22 * v.z;

        Vector3 {
            x,
            y,
            z
        }
    }

    fn setIdentity(&mut self) {
        self.m00 = 1.;
        self.m01 = 0.;
        self.m02 = 0.;

        self.m10 = 0.;
        self.m11 = 1.;
        self.m12 = 0.;

        self.m20 = 0.;
        self.m21 = 0.;
        self.m22 = 1.;
    }
    
    fn setZero(&mut self) {
        self.m00 = 0.;
        self.m01 = 0.;
        self.m02 = 0.;

        self.m10 = 0.;
        self.m11 = 0.;
        self.m12 = 0.;

        self.m20 = 0.;
        self.m21 = 0.;
        self.m22 = 0.;
    } 

    fn transpose(&mut self) {
        let m01: f64 = self.m10;
        let m02: f64 = self.m20;
        
        let m10: f64 = self.m01;
        let m12: f64 = self.m12;
        
        let m20: f64 = self.m02;
        let m21: f64 = self.m12;

        self.m01 = m01;
        self.m02 = m02;
        
        self.m10 = m10;
        self.m12 = m12;

        self.m20 = m20;
        self.m21 = m21;
    }

    fn invert(&mut self) {
        let determinant: f64 = self.det();

        if determinant != 0. {
            let determinantInv: f64 = 1.0/determinant;

            let t00: f64 = self.m11 * self.m22 - self.m12* self.m21;
            let t01: f64 = - self.m10 * self.m22 + self.m12 * self.m20;
            let t02: f64 = self.m10 * self.m21 - self.m11 * self.m20;

            let t10: f64 = - self.m01 * self.m22 + self.m02 * self.m21;
            let t11: f64 = self.m00 * self.m22 - self.m02 * self.m20;
            let t12: f64 = - self.m00 * self.m21 + self.m01 * self.m20;

            let t20: f64 = self.m01 * self.m12 - self.m02 * self.m11;
            let t21: f64 = -self.m00 * self.m12 + self.m02 * self.m10;
            let t22: f64 = self.m00 * self.m11 - self.m01 * self.m10;

            self.m00 = t00 * determinantInv;
            self.m01 = t01 * determinantInv;
            self.m02 = t02 * determinantInv;

            self.m10 = t10 * determinantInv;
            self.m11 = t11 * determinantInv;
            self.m12 = t12 * determinantInv;

            self.m20 = t20 * determinantInv;
            self.m21 = t21 * determinantInv;
            self.m22 = t22 * determinantInv;
        }
    }

    fn negate(&mut self) {
        self.m00 = -self.m00;
        self.m01 = -self.m01;
        self.m02 = -self.m02;

        self.m10 = -self.m10;
        self.m11 = -self.m11;
        self.m12 = -self.m12;

        self.m20 = -self.m20;
        self.m21 = -self.m21;
        self.m22 = -self.m22;
    }

    fn det(&self) -> f64 {
        return self.m00 * (self.m11 * self.m22 - self.m12 * self.m21)
                + self.m01 * (self.m12 * self.m20 - self.m10 * self.m22)
                + self.m02 * (self.m10 * self.m21 - self.m11 * self.m20);
    }

}