use crate::defs::*;

pub struct GeneralMath {
    pub fn interpolate(a: f64, b: f64, t: f64) -> f64 {
        a + ((b - a ) * t)
    }

    pub fn toRadians(degrees: f64) -> f64 {
        (PI/180.) * degrees
    }

    pub fn toDegrees(radians: f64) -> f64 {
        (180.0/PI) * radians
    }
}