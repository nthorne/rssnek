extern crate ncurses;
extern crate rand;

use self::ncurses::*;
use objects::{Snek, Pill};

use self::rand::{thread_rng, Rng};
use super::game::Score;
use std::cmp::max;
use std::char;

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

    pub fn create_dialog(&self, width: i32, height: i32, title: &str) -> WINDOW {

        let y = (self.max_y - height)/2;
        let x = (self.max_x - width)/2;

        self.show_text(format!("Dialog: {:?},{:?}@{:?},{:?}", width, height, x, y));

        let dialog = newwin(height, width, y, x);

        box_(dialog, 0, 0);
        scrollok(dialog, true);
        wclear(dialog);
        wrefresh(dialog);

        box_(dialog, 0, 0);
        mvwprintw(dialog, 0, 1, title);
        dialog
    }

    pub fn show_highscore_list(&self, s: &Vec<Score>) {
        let min_window_width = 25;
        let min_window_height = (s.len()+4) as i32;

        let max_name_width = s.into_iter().map(|e| e.name.len()).max().unwrap_or(0) as i32;
        let list_items = s.len() as i32;

        let window_width = gt_but_no_more(max_name_width, min_window_width, self.max_x-10);
        let window_height = gt_but_no_more(list_items, min_window_height, self.max_y-10);

        let highscore_win = self.create_dialog(window_width, window_height, "Highscores");

        for (i, e) in s.iter().enumerate() {
            mvwprintw(highscore_win, (i+2) as i32, 3, format!("{:2}. {:?}  {:?}", (i+1), e.name, e.score).as_str());
        }
        wrefresh(highscore_win);
    }

    pub fn input_dialog(&self, title: &str, length: i32) -> String {
        let min_window_width = 25;

        let window_width = gt_but_no_more(length, min_window_width, self.max_x-10);
        let window_height = 5;

        let highscore_win = self.create_dialog(window_width, window_height, title);

        wrefresh(highscore_win);

        let mut res = String::new();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        noecho();
        timeout(-1); // timeout waiting for input
        flushinp();
        mvwprintw(highscore_win, 2, 3, "");
        wrefresh(highscore_win);
        while 3 > res.len() {
            let c = char::from_u32(getch() as u32).unwrap_or(' ');
            if c.is_digit(10) || c.is_alphabetic() {
                res.push_str(&c.to_uppercase().to_string());

                mvwprintw(highscore_win, 2, 3, res.as_str());
                wrefresh(highscore_win);
            }
        }
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        res.truncate(length as usize);
        res
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

fn gt_but_no_more(v: i32, lo: i32, hi: i32) -> i32 {
    let min = max(v, lo);
    if max(v, lo) > hi {
        hi
    } else {
        min
    }
}
