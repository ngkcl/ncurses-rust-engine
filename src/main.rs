extern crate pancurses;

use pancurses::{initscr, endwin};

mod vectors;

fn main() {
    let window = initscr();
    window.printw("Hello World!");
    window.refresh();
    window.getch();
    endwin();
}
