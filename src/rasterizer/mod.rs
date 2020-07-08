use crate::matrices::Matrix44::*;
use crate::vectors::{Vector2::*, Vector3::*, Vector4::*};

struct FrameBuffer {
    width: i64,
    height: i64
};

impl FrameBuffer {
    fn new(w: i64, h:i64) -> FrameBuffer {
        FrameBuffer {
            width: w,
            height: h
        }
    }
}

struct Rasterizer<'a> {
    fb: &'a FrameBuffer
}

// impl Default for Rasterizer<'_> {
//     fn default() -> Rasterizer<'_> {
        
//     }
// }

impl<'a> Rasterizer<'a> {
    fn new(width: i64, height: i64) -> Rasterizer<'a> {
        let fb: FrameBuffer = FrameBuffer::new(width, height);

        Rasterizer {
            fb: &fb
        }
    }
 
    fn initializeFrameBuffer(width: i64, height: i64) {

    }

    fn getFrameBuffer(&self) -> &'a FrameBuffer {
        self.fb
    }

    fn rasterizeTriangle(v1: &Vector2, v2: &Vector2, v3: &Vector2) {

    }


}