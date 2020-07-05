use crate::vectors::VectorProperties::*;

// Vector4 --------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vector4 {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
}

impl VectorProperties for Vector4 {
    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    fn add(vec1: &Vector4, vec2: &Vector4) -> Vector4 {
        Vector4 {
            x: vec1.x + vec2.x,
            y: vec1.y + vec2.y,
            z: vec1.z + vec2.z,
            w: vec1.w + vec2.w
        }
    }
    
    fn sub(vec1: &Vector4, vec2: &Vector4) -> Vector4 {
        Vector4 {
            x: vec1.x - vec2.x,
            y: vec1.y - vec2.y,
            z: vec1.z - vec2.z,
            w: vec1.w - vec2.w
        }
    }

    fn mul(vec1: &Vector4, vec2: &Vector4) -> Vector4 {
        Vector4 {
            x: vec1.x * vec2.x,
            y: vec1.y * vec2.y,
            z: vec1.z * vec2.z,
            w: vec1.w * vec2.w
        }
    }
    
    fn lerp(vec1: &Vector4, vec2: &Vector4, t: f64) -> Vector4 {
        let x: f64 = vec1.x + ((vec2.x - vec1.x) * t);
        let y: f64 = vec1.y + ((vec2.y - vec1.y) * t);
        let z: f64 = vec1.z + ((vec2.z - vec1.z) * t);
        let w: f64 = vec1.w + ((vec2.w - vec1.w) * t);

        Vector4 {
            x,
            y,
            z,
            w
        }
    }

    fn dot(vec1: &Vector4, vec2: &Vector4) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z + vec1.w * vec2.w
    }

    fn angle(vec1: &Vector4, vec2: &Vector4) -> f64 {
        let dot = Vector4::dot(&vec1, &vec2);
        let f = dot/(vec1.length() * vec2.length());

        return f.acos()
    }

    fn dist(vec1: &Vector4, vec2: &Vector4) -> f64 {
        let x: f64 = vec1.x - vec2.x;
        let y: f64 = vec1.y - vec2.y;
        let z: f64 = vec1.z - vec2.z;
        let w: f64 = vec1.w - vec2.w;

        return (x*x + y*y + z*z + w*w).sqrt()
    }

    fn getNormalized(v: &Vector4) -> Vector4 {
        let mut f: Vector4 = *v;
        f.normalize();
        return f;
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        self.w *= factor;
    }

    fn normalize(&mut self) {
        let mag = self.length();

        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
    }
    
    fn print(&self) {
        println!("{} {} {} {}", self.x, self.y, self.z, self.w)
    }
}
// ==================================