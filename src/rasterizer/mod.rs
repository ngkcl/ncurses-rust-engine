use crate::matrices::Matrix44::*;
use crate::vectors::{Vector2::*, Vector3::*, Vector4::*};
use crate::defs::*;
use pancurses::Window;

pub struct FrameBuffer {
    width: i32,
    height: i32
}

impl FrameBuffer {
    fn new(w: i32, h:i32) -> FrameBuffer {
        FrameBuffer {
            width: w,
            height: h
        }
    }
}

pub struct Rasterizer<'a> {
    fb: FrameBuffer,
    window: &'a Window
}

// impl Default for Rasterizer< 'static> {
//     fn default() -> Rasterizer<'static>{
//         Rasterizer::new(500, 500)
//     }
// }

impl Rasterizer<'_> {
    pub fn new<'a>(width: i32, height: i32, window: &'a Window) -> Rasterizer<'a> {
        Rasterizer {
            fb: FrameBuffer::new(width, height),
            window: window
        }
    }

    fn getFrameBuffer(&self) -> &FrameBuffer {
        &self.fb
    }

    pub fn rasterizeTriangle(&self, v1: &Vector2, v2: &Vector2, v3: &Vector2) {
        let minX: i32;
        let maxX: i32;
        let minY: i32;
        let maxY: i32; 

        minX = 0.max(v1.x.min(v2.x.min(v3.x)) as i32);
        minY = 0.max(v1.y.min(v2.y.min(v3.y)) as i32);

        maxX = self.fb.width.min(v1.x.max(v2.x.max(v3.x)) as i32 + 1);
        maxY = self.fb.height.min(v1.y.max(v2.y.max(v3.y)) as i32 + 1);

        for j in minY..maxY {
            for i in minX..maxX {
                if Rasterizer::isPointInTriangle(i, j, &v1, &v2, &v3) {
                    self.window.mvprintw(j, i, "#");
                } else {
                    self.window.mvprintw(j, i, ".");
                }
            }
        }
    }

    pub fn isPointInTriangle(ptX: i32, ptY: i32, v1: &Vector2, v2: &Vector2, v3: &Vector2) -> bool {
        let wv1: f64 = ((v2.y - v3.y) * (ptX as f64 - v3.x) +
            (v3.x - v2.x) * (ptY as f64 - v3.y)) /
        ((v2.y - v3.y) * (v1.x - v3.x) +
        (v3.x - v2.x) * (v1.y - v3.y));

        let wv2: f64 = ((v3.y - v1.y) * (ptX as f64 - v3.x) +
            (v1.x - v3.x) * (ptY as f64 - v3.y)) /
        ((v2.y - v3.y) * (v1.x - v3.x) +
        (v3.x - v2.x) * (v1.y - v3.y));
        
        let wv3: f64 = 1f64 - wv1 -wv2;

        let one: bool = wv1 < -0.001;
        let two: bool = wv2 < -0.001;
        let three: bool = wv3 < -0.001;

        (one == two) && (two == three)
    }


}