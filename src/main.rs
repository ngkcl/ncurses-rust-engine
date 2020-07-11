#![allow(non_snake_case)]
extern crate pancurses;

use pancurses::{initscr, endwin, raw, noecho};

mod vectors;
mod matrices;
mod quaternions;
mod rasterizer;
mod math_utils;

mod defs;

use self::rasterizer::*;
use self::matrices::Matrix44::*;
use self::vectors::{Vector2::*, Vector3::*, Vector4::*};
use self::math_utils::GeneralMath;

fn main() {
    let window = initscr();
    raw();
    noecho();

    let LINES = window.get_max_y();
    let COLS = window.get_max_x();
    println!("{}", LINES*3);
    println!("{}", COLS);

    let rast = Rasterizer::new(LINES*10, COLS*10, &window);

    let mut transformation: Matrix44 = Default::default();
    transformation.scale(&Vector3{
        x: 4.0,
        y: 2.0,
        z: 1.0 
    });

    transformation.rotate(&Vector3{
        x: 0.,
        y: 0.,
        z: 1.
    }, GeneralMath::toRadians(20.0));

    let mut v1: Vector4 = Vector4 {
        x: 10.0,
        y: 10.0,
        z: 1.0,
        w: 1.0
    };

    let mut v2: Vector4 = Vector4 {
        x: 20.0,
        y: 10.0,
        z: 1.0,
        w: 1.0
    };

    let mut v3: Vector4 = Vector4 {
        x: 15.0,
        y: 20.0,
        z: 1.0,
        w: 1.0
    };

    v1 = transformation * v1;
    v2 = transformation * v2;
    v3 = transformation * v3;

    rast.rasterizeTriangle(&Vector2{
        x: v1.x,
        y: v1.y
    }, &Vector2{
        x: v2.x,
        y: v2.y
    }, &Vector2 {
        x: v3.x,
        y: v3.y 
    });

    // window.printw("Hello World!");
    // window.refresh();
    window.getch();
    window.clear();

    endwin();
}
