use crate::matrices::MatrixProperties::*;
use crate::vectors::{Vector4::*, Vector3::*};
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix44 {
    pub m00: f64, 
    pub m01: f64, 
    pub m02: f64, 
    pub m03: f64,

    pub m10: f64, 
    pub m11: f64, 
    pub m12: f64, 
    pub m13: f64,

    pub m20: f64, 
    pub m21: f64, 
    pub m22: f64, 
    pub m23: f64,

    pub m30: f64, 
    pub m31: f64, 
    pub m32: f64, 
    pub m33: f64
}

impl Default for Matrix44 {
    fn default() -> Matrix44 {
        Matrix44 {
            m00: 1., m01: 0., m02: 0., m03: 0.,
            m10: 0., m11: 1., m12: 0., m13: 0.,
            m20: 0., m21: 0., m22: 1., m23: 0.,
            m30: 0., m31: 0., m32: 0., m33: 1.,
        }
    }
}

impl Matrix44 {
    pub fn scale(&mut self, v: &Vector3) {
        self.m00 *= v.x;
        self.m01 *= v.x;
        self.m02 *= v.x;
        self.m03 *= v.x;
    
        self.m10 *= v.y;
        self.m11 *= v.y;
        self.m12 *= v.y;
        self.m13 *= v.y;

        self.m20 *= v.z;
        self.m21 *= v.z;
        self.m22 *= v.z;
        self.m23 *= v.z;
    }

    pub fn rotate(&mut self, eulerAxis: &Vector3, angle: f64) {
        let c: f64 = angle.cos();
        let s: f64 = angle.sin();

        let oneMinusC = 1.0 - c;

        let xy: f64 = eulerAxis.x * eulerAxis.y;
        let yz: f64 = eulerAxis.y * eulerAxis.z;
        let xz: f64 = eulerAxis.x * eulerAxis.z;
        let xs: f64 = eulerAxis.x * s;
        let ys: f64 = eulerAxis.y * s;
        let zs: f64 = eulerAxis.z * s;

        let f00: f64 = eulerAxis.x * eulerAxis.x * oneMinusC + c ;
        let f01: f64 = xy * oneMinusC + zs;
        let f02: f64 = xz * oneMinusC - ys;
        let f10: f64 = xy * oneMinusC - zs;
        let f11: f64 = eulerAxis.y * eulerAxis.y * oneMinusC + c;
        let f12: f64 = yz * oneMinusC + xs;
        let f20: f64 = xz * oneMinusC + ys;
        let f21: f64 = yz * oneMinusC - xs;
        let f22: f64 = eulerAxis.z * eulerAxis.z * oneMinusC + c;

        let t00: f64 = self.m00 * f00 + self.m10 * f01 + self.m20 * f02;
        let t01: f64 = self.m01 * f00 + self.m11 * f01 + self.m21 * f02;
        let t02: f64 = self.m02 * f00 + self.m12 * f01 + self.m22 * f02;
        let t03: f64 = self.m03 * f00 + self.m13 * f01 + self.m23 * f02;
        let t10: f64 = self.m00 * f10 + self.m10 * f11 + self.m20 * f12;
        let t11: f64 = self.m01 * f10 + self.m11 * f11 + self.m21 * f12;
        let t12: f64 = self.m02 * f10 + self.m12 * f11 + self.m22 * f12;
        let t13: f64 = self.m03 * f10 + self.m13 * f11 + self.m23 * f12;
    
        self.m20 = self.m00 * f20 + self.m10 * f21 + self.m20 * f22;
        self.m21 = self.m01 * f20 + self.m11 * f21 + self.m21 * f22;
        self.m22 = self.m02 * f20 + self.m12 * f21 + self.m22 * f22;
        self.m23 = self.m03 * f20 + self.m13 * f21 + self.m23 * f22;
        self.m00 = t00;
        self.m01 = t01;
        self.m02 = t02;
        self.m03 = t03;
        self.m10 = t10;
        self.m11 = t11;
        self.m12 = t12;
        self.m13 = t13;
    }

    pub fn translate(&mut self, v: &Vector3) {
        self.m30 += self.m00 * v.x + self.m10 * v.y + self.m20 * v.z;
        self.m31 += self.m01 * v.x + self.m11 * v.y + self.m21 * v.z;
        self.m32 += self.m02 * v.x + self.m12 * v.y + self.m22 * v.z;
        self.m33 += self.m03 * v.x + self.m13 * v.y + self.m23 * v.z;
    }
}

fn DET33(t00: f64, t01: f64, t02: f64, t10: f64, t11: f64, t12: f64, t20: f64, t21: f64, t22: f64) -> f64 {
    (t00 * (t11 * t22 - t12 * t21)) + (t01 * (t12 * t20 - t10 * t22)) + (t02 * (t10 * t21 - t11 * t20))
}

impl MatrixProperties for Matrix44 {
    type Vector = Vector4;

    fn add(m1: &Matrix44, m2: &Matrix44) -> Matrix44 {
        let m00: f64 =  m1.m00 + m2.m00;
        let m01: f64 =  m1.m01 + m2.m01;
        let m02: f64 =  m1.m02 + m2.m02;
        let m03: f64 =  m1.m03 + m2.m03;
        

        let m10: f64 =  m1.m10 + m2.m10;
        let m11: f64 =  m1.m11 + m2.m11;
        let m12: f64 =  m1.m12 + m2.m12;
        let m13: f64 =  m1.m13 + m2.m13;

        let m20: f64 =  m1.m20 + m2.m20;
        let m21: f64 =  m1.m21 + m2.m21;
        let m22: f64 =  m1.m22 + m2.m22;
        let m23: f64 =  m1.m23 + m2.m23;


        let m30: f64 =  m1.m30 + m2.m30;
        let m31: f64 =  m1.m31 + m2.m31;
        let m32: f64 =  m1.m32 + m2.m32;
        let m33: f64 =  m1.m33 + m2.m33;

        
        Matrix44 {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33
        }
    }

    fn sub(m1: &Matrix44, m2: &Matrix44) -> Matrix44 {
        let m00: f64 =  m1.m00 - m2.m00;
        let m01: f64 =  m1.m01 - m2.m01;
        let m02: f64 =  m1.m02 - m2.m02;
        let m03: f64 =  m1.m03 - m2.m03;
        

        let m10: f64 =  m1.m10 - m2.m10;
        let m11: f64 =  m1.m11 - m2.m11;
        let m12: f64 =  m1.m12 - m2.m12;
        let m13: f64 =  m1.m13 - m2.m13;

        let m20: f64 =  m1.m20 - m2.m20;
        let m21: f64 =  m1.m21 - m2.m21;
        let m22: f64 =  m1.m22 - m2.m22;
        let m23: f64 =  m1.m23 - m2.m23;


        let m30: f64 =  m1.m30 - m2.m30;
        let m31: f64 =  m1.m31 - m2.m31;
        let m32: f64 =  m1.m32 - m2.m32;
        let m33: f64 =  m1.m33 - m2.m33;

        Matrix44 {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33
        }
    }

    fn mul(m1: &Matrix44, m2: &Matrix44) -> Matrix44 {
        let m00: f64 = m1.m00 * m2.m00 + m1.m10 * m2.m01 + m1.m20 * m2.m02 + m1.m30 * m2.m03;
        let m01: f64 = m1.m01 * m2.m00 + m1.m11 * m2.m01 + m1.m21 * m2.m02 + m1.m31 * m2.m03;
        let m02: f64 = m1.m02 * m2.m00 + m1.m12 * m2.m01 + m1.m22 * m2.m02 + m1.m32 * m2.m03;
        let m03: f64 = m1.m03 * m2.m00 + m1.m13 * m2.m01 + m1.m23 * m2.m02 + m1.m33 * m2.m03;
        let m10: f64 = m1.m00 * m2.m10 + m1.m10 * m2.m11 + m1.m20 * m2.m12 + m1.m30 * m2.m13;
        let m11: f64 = m1.m01 * m2.m10 + m1.m11 * m2.m11 + m1.m21 * m2.m12 + m1.m31 * m2.m13;
        let m12: f64 = m1.m02 * m2.m10 + m1.m12 * m2.m11 + m1.m22 * m2.m12 + m1.m32 * m2.m13;
        let m13: f64 = m1.m03 * m2.m10 + m1.m13 * m2.m11 + m1.m23 * m2.m12 + m1.m33 * m2.m13;
        let m20: f64 = m1.m00 * m2.m20 + m1.m10 * m2.m21 + m1.m20 * m2.m22 + m1.m30 * m2.m23;
        let m21: f64 = m1.m01 * m2.m20 + m1.m11 * m2.m21 + m1.m21 * m2.m22 + m1.m31 * m2.m23;
        let m22: f64 = m1.m02 * m2.m20 + m1.m12 * m2.m21 + m1.m22 * m2.m22 + m1.m32 * m2.m23;
        let m23: f64 = m1.m03 * m2.m20 + m1.m13 * m2.m21 + m1.m23 * m2.m22 + m1.m33 * m2.m23;
        let m30: f64 = m1.m00 * m2.m30 + m1.m10 * m2.m31 + m1.m20 * m2.m32 + m1.m30 * m2.m33;
        let m31: f64 = m1.m01 * m2.m30 + m1.m11 * m2.m31 + m1.m21 * m2.m32 + m1.m31 * m2.m33;
        let m32: f64 = m1.m02 * m2.m30 + m1.m12 * m2.m31 + m1.m22 * m2.m32 + m1.m32 * m2.m33;
        let m33: f64 = m1.m03 * m2.m30 + m1.m13 * m2.m31 + m1.m23 * m2.m32 + m1.m33 * m2.m33;


        Matrix44 {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33
        }
    }

    fn trans(m: &Matrix44, v: &Vector4) -> Vector4 {
        let x: f64 = m.m00 * v.x + m.m10 * v.y + m.m20 * v.z + m.m30 * v.w;
        let y: f64 = m.m01 * v.x + m.m11 * v.y + m.m21 * v.z + m.m31 * v.w;
        let z: f64 = m.m02 * v.x + m.m12 * v.y + m.m22 * v.z + m.m32 * v.w;
        let w: f64 = m.m03 * v.x + m.m13 * v.y + m.m23 * v.z + m.m33 * v.w;
        
        Vector4 {
            x,
            y,
            z,
            w
        }
    }

    fn setIdentity(&mut self) {
        self.m00 = 1.;
        self.m01 = 0.;
        self.m02 = 0.;
        self.m03 = 0.;

        self.m10 = 0.;
        self.m11 = 1.;
        self.m12 = 0.;
        self.m13 = 0.;
        
        self.m20 = 0.;
        self.m21 = 0.;
        self.m22 = 1.;
        self.m23 = 0.;

        self.m30 = 0.;
        self.m31 = 0.;
        self.m32 = 0.;
        self.m33 = 1.;
    }
    
    fn setZero(&mut self) {
        self.m00 = 0.;
        self.m01 = 0.;
        self.m02 = 0.;
        self.m03 = 0.;

        self.m10 = 0.;
        self.m11 = 0.;
        self.m12 = 0.;
        self.m13 = 0.;
        
        self.m20 = 0.;
        self.m21 = 0.;
        self.m22 = 0.;
        self.m23 = 0.;

        self.m30 = 0.;
        self.m31 = 0.;
        self.m32 = 0.;
        self.m33 = 0.;
    } 

    fn transpose(&mut self) {
        let m01: f64 = self.m10;
        let m02: f64 = self.m20;
        let m03: f64 = self.m30;

        let m10: f64 = self.m01;
        let m12: f64 = self.m21;
        let m13: f64 = self.m31;

        let m20: f64 = self.m02;
        let m21: f64 = self.m12;
        let m23: f64 = self.m32;

        let m30: f64 = self.m03;
        let m31: f64 = self.m13;
        let m32: f64 = self.m23;

        self.m01 = m01;
        self.m02 = m02;
        self.m03 = m03;

        self.m10 = m10;
        self.m12 = m12;
        self.m13 = m13;
     
        self.m20 = m20;
        self.m21 = m21;
        self.m23 = m23;
     
        self.m30 = m30;
        self.m31 = m31;
        self.m32 = m32;
    }

    fn invert(&mut self) {
        let determinant: f64 = self.det();

        if determinant != 0. {
            let determinantInv: f64 = 1.0/determinant;

            let t00: f64 =  DET33(self.m11, self.m12, self.m13, self.m21, self.m22, self.m23, self.m31, self.m32, self.m33);
            let t01: f64 = -DET33(self.m10, self.m12, self.m13, self.m20, self.m22, self.m23, self.m30, self.m32, self.m33);
            let t02: f64 =  DET33(self.m10, self.m11, self.m13, self.m20, self.m21, self.m23, self.m30, self.m31, self.m33);
            let t03: f64 = -DET33(self.m10, self.m11, self.m12, self.m20, self.m21, self.m22, self.m30, self.m31, self.m32);

            let t10: f64 = -DET33(self.m01, self.m02, self.m03, self.m21, self.m22, self.m23, self.m31, self.m32, self.m33);
            let t11: f64 =  DET33(self.m00, self.m02, self.m03, self.m20, self.m22, self.m23, self.m30, self.m32, self.m33);
            let t12: f64 = -DET33(self.m00, self.m01, self.m03, self.m20, self.m21, self.m23, self.m30, self.m31, self.m33);
            let t13: f64 =  DET33(self.m00, self.m01, self.m02, self.m20, self.m21, self.m22, self.m30, self.m31, self.m32);

            let t20: f64 =  DET33(self.m01, self.m02, self.m03, self.m11, self.m12, self.m13, self.m31, self.m32, self.m33);
            let t21: f64 = -DET33(self.m00, self.m02, self.m03, self.m10, self.m12, self.m13, self.m30, self.m32, self.m33);
            let t22: f64 =  DET33(self.m00, self.m01, self.m03, self.m10, self.m11, self.m13, self.m30, self.m31, self.m33);
            let t23: f64 = -DET33(self.m00, self.m01, self.m02, self.m10, self.m11, self.m12, self.m30, self.m31, self.m32);

            let t30: f64 = -DET33(self.m01, self.m02, self.m03, self.m11, self.m12, self.m13, self.m21, self.m22, self.m23);
            let t31: f64 =  DET33(self.m00, self.m02, self.m03, self.m10, self.m12, self.m13, self.m20, self.m22, self.m23);
            let t32: f64 = -DET33(self.m00, self.m01, self.m03, self.m10, self.m11, self.m13, self.m20, self.m21, self.m23);
            let t33: f64 =  DET33(self.m00, self.m01, self.m02, self.m10, self.m11, self.m12, self.m20, self.m21, self.m22);

            self.m00 = t00*determinantInv;
            self.m11 = t11*determinantInv;
            self.m22 = t22*determinantInv;
            self.m33 = t33*determinantInv;
            self.m01 = t10*determinantInv;
            self.m10 = t01*determinantInv;
            self.m20 = t02*determinantInv;
            self.m02 = t20*determinantInv;
            self.m12 = t21*determinantInv;
            self.m21 = t12*determinantInv;
            self.m03 = t30*determinantInv;
            self.m30 = t03*determinantInv;
            self.m13 = t31*determinantInv;
            self.m31 = t13*determinantInv;
            self.m32 = t23*determinantInv;
            self.m23 = t32*determinantInv;
        }
    }

    fn negate(&mut self) {
        self.m00 = -self.m00;
        self.m01 = -self.m01;
        self.m02 = -self.m02;
        self.m03 = -self.m03;

        self.m10 = -self.m10;
        self.m11 = -self.m11;
        self.m12 = -self.m12;
        self.m13 = -self.m13;

        self.m20 = -self.m20;
        self.m21 = -self.m21;
        self.m22 = -self.m22;
        self.m23 = -self.m23;

        self.m30 = -self.m30;
        self.m31 = -self.m31;
        self.m32 = -self.m32;
        self.m33 = -self.m33;
    }

    fn det(&self) -> f64 {
        let mut f: f64 = self.m00 * ((self.m11 * self.m22 * self.m33 + self.m12 * self.m23 * self.m31 + self.m13 * self.m21 * self.m32)
            - self.m13 * self.m22 * self.m31
            - self.m11 * self.m23 * self.m32
            - self.m12 * self.m21 * self.m33);
        f -= self.m01 * ((self.m10 * self.m22 * self.m33 + self.m12 * self.m23 * self.m30 + self.m13 * self.m20 * self.m32)
            - self.m13 * self.m22 * self.m30
            - self.m10 * self.m23 * self.m32
            - self.m12 * self.m20 * self.m33);
        f += self.m02 * ((self.m10 * self.m21 * self.m33 + self.m11 * self.m23 * self.m30 + self.m13 * self.m20 * self.m31)
            - self.m13 * self.m21 * self.m30
            - self.m10 * self.m23 * self.m31
            - self.m11 * self.m20 * self.m33);
        f -= self.m03 * ((self.m10 * self.m21 * self.m32 + self.m11 * self.m22 * self.m30 + self.m12 * self.m20 * self.m31)
            - self.m12 * self.m21 * self.m30
            - self.m10 * self.m22 * self.m31
            - self.m11 * self.m20 * self.m32);
        return f;
    }

    fn print(&self) {
        println!("{} {} {} {}", self.m00, self.m01, self.m02, self.m03);
        println!("{} {} {} {}", self.m10, self.m11, self.m12, self.m13);
        println!("{} {} {} {}", self.m20, self.m21, self.m22, self.m23);
        println!("{} {} {} {}", self.m30, self.m31, self.m32, self.m33);
    }
}

// Operator overloading
impl ops::Add<Matrix44> for Matrix44 {
    type Output = Matrix44;

    fn add(self, rhs: Matrix44) -> Matrix44 {
        <Matrix44 as MatrixProperties>::add(&self, &rhs)
    }
}

impl ops::AddAssign<Matrix44> for Matrix44 {
    fn add_assign(&mut self, rhs: Matrix44) {
        *self = Matrix44::add(&self, &rhs);
    }
}

impl ops::Sub<Matrix44> for Matrix44 {
    type Output = Matrix44;

    fn sub(self, rhs: Matrix44) -> Matrix44 {
        <Matrix44 as MatrixProperties>::sub(&self, &rhs)
    }
}

impl ops::SubAssign<Matrix44> for Matrix44 {
    fn sub_assign(&mut self, rhs: Matrix44) {
        *self = Matrix44::sub(&self, &rhs)
    }
}

impl ops::Mul<Matrix44> for Matrix44 {
    type Output = Matrix44;

    fn mul(self, rhs: Matrix44) -> Matrix44 {
        <Matrix44 as MatrixProperties>::mul(&self, &rhs)
    }
}

impl ops::Mul<Vector4> for Matrix44 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        <Matrix44 as MatrixProperties>::trans(&self, &rhs)
    }
}

impl ops::MulAssign<Matrix44> for Matrix44 {
    fn mul_assign(&mut self, rhs: Matrix44) {
        *self = Matrix44::mul(&self, &rhs)
    }
}