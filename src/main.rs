extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    printw("Hello, world! (press any key to terminate)");

    refresh();

    getch();

    endwin();
}
