#![allow(non_snake_case)]
extern crate pancurses;

use pancurses::{initscr, endwin};

mod vectors;
mod matrices;
mod quaternions;
mod math_utils;

mod defs;

fn main() {
    let window = initscr();
    window.printw("Hello World!");
    window.refresh();
    window.getch();
    endwin();
}
