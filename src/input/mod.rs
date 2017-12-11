extern crate ncurses;

use self::ncurses::*;
use std::sync::mpsc::{channel, Sender};
use super::game::{Event};
use std::thread;

pub fn init() {
    // keypad(...) is needed for ncurses to recognize e.g. KEY_UP
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn input_loop(dis: Sender<Event>) -> Sender<bool> {
    cbreak();
    noecho();

    timeout(1);

    const Q: i32 = 'q' as i32;
    const G: i32 = 'g' as i32;

    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            match getch() {
                KEY_UP => dis.send(Event::Up).unwrap(),
                KEY_DOWN => dis.send(Event::Down).unwrap(),
                KEY_LEFT => dis.send(Event::Left).unwrap(),
                KEY_RIGHT => dis.send(Event::Right).unwrap(),
                Q => dis.send(Event::Terminate).unwrap(),
                G => dis.send(Event::Grow).unwrap(),
                _ => (),
            }

            if let Ok(_)  = rx.try_recv() {
                dis.send(Event::Debug("Terminating input loop".to_owned())).unwrap();
                break
            }
        }
    });

    tx
}
