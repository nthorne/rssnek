extern crate ncurses;
extern crate rand;

use self::ncurses::*;
use objects::{Snek, Pill};

use self::rand::{thread_rng, Rng};

pub type WindowSize = i32;
pub struct Display {
    pub max_x: WindowSize,  // TODO: Keep if I need to handle resise, otherwise drop.
    pub max_y: WindowSize,  // TODO: Keep if I need to handle resize, otherwise drop.
    main: WINDOW,
    log: WINDOW,
}

impl Display {
    pub fn new() -> Display {
        let mut max_screen_x = 0;
        let mut max_screen_y = 0;
        getmaxyx(stdscr(), &mut max_screen_y, &mut max_screen_x);

        let main_win = create_main_window(max_screen_x, max_screen_y);
        let mut mx = 0;
        let mut my = 0;
        getmaxyx(main_win, &mut my, &mut mx);

        refresh();

        Display {
            max_x: mx,
            max_y: my,
            main: main_win,
            log: create_log_window(max_screen_x, max_screen_y),
        }
    }

    pub fn show_snek(&self, snek: &Snek) {

        for section in snek.repr() {
            let (x, y) = section;
            mvwaddch(self.main, y, x, 'O' as chtype);
        }

        mvwaddch(self.main, snek.tail().1, snek.tail().0, ' ' as chtype);
        wrefresh(self.main);
    }

    pub fn show_pill(&self, pill: &Pill) {
        mvwaddch(self.main, pill.pos().1, pill.pos().0, '*' as chtype);
        wrefresh(self.main);
    }

    pub fn show_text(&self, t: String) {
        mvwprintw(self.log, getcury(self.log), 1, format!("{}\n", t).as_str());
        box_(self.log, 0, 0);
        wrefresh(self.log);
    }

    pub fn in_main_window(&self, x: WindowSize, y: WindowSize) -> bool {
        x >= 0 && x <= self.max_x && y >= 0 && y <= self.max_y
    }

    pub fn dump(&self) {
        self.show_text(format!("{}x{}", self.max_x, self.max_y));
    }

    pub fn get_x_in_main(&self) -> WindowSize {
        let mut rng = thread_rng();
        rng.gen_range(0, self.max_x)
    }

    pub fn get_y_in_main(&self) -> WindowSize {
        let mut rng = thread_rng();
        rng.gen_range(0, self.max_y)
    }
}

pub fn init() {
    initscr();
}

pub fn deinit() {
    endwin();
}

fn create_main_window(max_x: WindowSize, max_y: WindowSize) -> WINDOW {
    let width = max_x;
    let height = max_y - 10;
    let x = 0;
    let y = 0;

    let main_win = newwin(height, width, y, x);
    scrollok(main_win, false);
    wclear(main_win);
    wrefresh(main_win);

    main_win
}

fn create_log_window(max_x: WindowSize, max_y: WindowSize) -> WINDOW {
    let width = max_x;
    let height = 10;
    let x = 0;
    let y = max_y - 10;

    let log_win = newwin(height, width, y, x);

    box_(log_win, 0, 0);
    scrollok(log_win, true);
    wclear(log_win);
    wrefresh(log_win);

    log_win
}
