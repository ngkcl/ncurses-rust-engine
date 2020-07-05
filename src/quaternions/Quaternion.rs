use crate::vectors::{VectorProperties::* ,Vector3::Vector3, Vector4::*};
use crate::matrices::{Matrix44::*};
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Default for Quaternion {
    fn default() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        }
    }
}

impl Quaternion {
    fn setIdentity(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.w = 1.0;
    }

    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    fn normalize(&mut self) {
        let mag: f64 = self.length();

        if mag == 0f64 {
            return;
        }

        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
    }

    fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
    }

    fn toMatrix(&self) -> Matrix44 {
        let mut matrix: Matrix44 = Default::default();

        let xy: f64 = self.x * self.y;
        let xz: f64 = self.x * self.z;
        let xw: f64 = self.x * self.w;
        let yz: f64 = self.y * self.z;
        let yw: f64 = self.y * self.w;
        let zw: f64 = self.z * self.w;

        matrix.m00 = 1.0 - 2.0 * (self.y*self.y + self.z*self.z);
        matrix.m01 = 2.0 * (xy - zw);
        matrix.m02 = 2.0 * (xz + yw);
        matrix.m03 = 0.0;
        matrix.m10 = 2.0 * (xy + zw);
        matrix.m11 = 1.0 - 2.0 * (self.x*self.x + self.z*self.z);
        matrix.m12 = 2.0 * (yz - xw);
        matrix.m13 = 0.0;
        matrix.m20 = 2.0 * (xz - yw);
        matrix.m21 = 2.0 * (yz + xw);
        matrix.m22 = 1.0 - 2.0 * (self.x*self.x + self.y*self.y);
        matrix.m23 = 0.0;
        matrix.m30 = 0.0;
        matrix.m31 = 0.0;
        matrix.m32 = 0.0;
        matrix.m33 = 1.0;

        return matrix;
    }

    fn setToAxisAngle(&mut self, axis: &Vector3, angle: f64) {
        let mut rot: Matrix44 = Default::default();

        rot.rotate(&axis, angle);
        
        self.setMatrix(&rot);
        self.normalize();
    }

    fn slerp(a: &Quaternion, b: &Quaternion, blend: f64) -> Quaternion {
        let mut result: Quaternion = Default::default();

        let dot = a.w *b.w + a.x*b.x + a.y*b.y + a.z*b.z;
        let blendI = 1.0 - blend;

        if dot < 0.0 {
            result.w = blendI * a.w + blend * -b.w;
            result.x = blendI * a.x + blend * -b.x;
            result.y = blendI * a.y + blend * -b.y;
            result.z = blendI * a.z + blend * -b.z;
        } else {
            result.w = blendI * a.w + blend * b.w;
            result.x = blendI * a.x + blend * b.x;
            result.y = blendI * a.y + blend * b.y;
            result.z = blendI * a.z + blend * b.z;
        }

        result.normalize();
        return result;
    }

    pub fn lookRotation(&mut self, f: &Vector3, u: &Vector3) {
        // Copy (should work)
        let mut forward: Vector3 = *f;
        let mut up: Vector3 = *u;

        forward.normalize();
        up.normalize();
        
        let right = Vector3::cross(&forward, &up);

        let mut rot: Matrix44 = Default::default();

        rot.m00 = right.x;
        rot.m10 = right.y;
        rot.m20 = right.z;

        rot.m01 = up.x;
        rot.m11 = up.y;
        rot.m21 = up.z;

        rot.m02 = forward.x;
        rot.m12 = forward.y;
        rot.m22 = forward.z;

        self.setMatrix(&rot);
        self.normalize();
    }

    fn setMatrix(&mut self, matrix: &Matrix44) {
        let m00 = matrix.m00;
        let m01 = matrix.m01;
        let m02 = matrix.m02;
   
        let m10 = matrix.m10;
        let m11 = matrix.m11;
        let m12 = matrix.m12;
   
        let m20 = matrix.m20;
        let m21 = matrix.m21;
        let m22 = matrix.m22;

        let mut s: f64;
        let tr = m00 + m11 + m22;

        if tr >= 0.0 {
            s = (tr + 1.0).sqrt();
            self.w = s * 0.5;
            s = 0.5 / s;
            self.x = (m21 - m12) * s;
            self.y = (m02 - m20) * s;
            self.z = (m10 - m01) * s;
        } else {
            let max: f64 = m00.max(m11).max(m22);
            if max == m00 {
                s = (m00 - (m11 + m22) + 1.0).sqrt();
                self.x = s * 0.5;
                s = 0.5 / s;
                self.y = (m01 + m10) * s;
                self.z = (m20 + m02) * s;
                self.w = (m21 - m12) * s;
            } else if max == m11 {
                s = (m11 - (m22 + m00) + 1.0).sqrt();
                self.y = s * 0.5;
                s = 0.5 / s;
                self.z = (m12 + m21) * s;
                self.x = (m01 + m10) * s;
                self.w = (m02 - m20) * s;
            } else {
                s = (m22 - (m00 + m11) + 1.0).sqrt();
                self.z = s * 0.5;
                s = 0.5 / s;
                self.x = (m20 + m02) * s;
                self.y = (m12 + m21) * s;
                self.w = (m10 - m01) * s;
            }
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z
            - self.z * rhs.y, 
            
            y: self.y * rhs.w + self.w * rhs.y
            + self.z * rhs.x - self.x * rhs.z, 
            
            z: self.z * rhs.w
            + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,

            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y
            - self.z * rhs.z
        }
    }
}

impl ops::Mul<Vector3> for Quaternion {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        let trans: Matrix44 = self.toMatrix();

        let r: Vector4 = trans * Vector4{x: rhs.x, y: rhs.y, z: rhs.z, w: 0f64};

        Vector3 {
            x: r.x,
            y: r.y,
            z: r.z
        }
    }
}

impl ops::Mul<Vector4> for Quaternion {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        let trans: Matrix44 = self.toMatrix();
        trans * rhs
    }
}

impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        self.x = self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y;
        self.y = self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z;
        self.z = self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x;
        self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
    }
}