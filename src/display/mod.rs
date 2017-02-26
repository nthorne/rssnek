extern crate ncurses;

use self::ncurses::*;

pub fn init() {
    initscr();
}

pub fn deinit() {
    endwin();
}

pub fn show() {
    printw(".");

    refresh();
}
