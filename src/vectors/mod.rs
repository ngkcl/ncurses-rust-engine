trait VectorProperties {
    fn length(&self) -> f64;

    // TODO: Look at overloading so we have instance and static methods
    fn add(vec1: &Self, vec2: &Self) -> Self;
    fn sub(vec1: &Self, vec2: &Self) -> Self; 
    fn mul(vec1: &Self, vec2: &Self) -> Self;
    fn lerp(vec1: &Self, vec2: &Self, t: f64) -> Self;

    fn dot(vec1: &Self, vec2: &Self) -> f64;
    fn angle(vec1: &Self, vec2: &Self) -> f64;
    fn dist(vec1: &Self, vec2: &Self) -> f64;
    
    fn getNormalized(v: &Self) -> Self;

    // Mutators
    fn scale(&mut self, factor: f64);
    fn normalize(&mut self);

    // Getters
}

// Vector2 --------------------------
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
// ==================================

// Vector 3 -------------------------
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

    fn cross(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
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
}
// ==================================

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
}
// ==================================