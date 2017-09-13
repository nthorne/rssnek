extern crate ncurses;

use self::ncurses::*;
use std::collections::HashSet;
use objects::{Snek, Pill};

pub fn init() {
    initscr();
}

pub fn deinit() {
    endwin();
}

pub fn show_snek(snek: &Snek) {

    for section in snek.repr() {
        let (x, y) = section;
        mvaddch(y, x, 'O' as chtype);
    }

    mvaddch(snek.tail().1, snek.tail().0, ' ' as chtype);

    refresh();
}

pub fn show_pill(pill: &Pill) {
    mvaddch(pill.pos().1, pill.pos().0, '*' as chtype);
    refresh();
}
