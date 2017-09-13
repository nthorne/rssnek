// TODO: Not to be used here..
extern crate ncurses;
extern crate rand;

use self::ncurses::*;
use super::game::{Event};
use std::sync::mpsc::{Sender};
use std::collections::HashSet;

use self::rand::{thread_rng, Rng};

type EventChannel = Sender<Event>;
pub type Section = (i32, i32);
pub type Repr = Vec<Section>;
pub struct Snek {
    repr: Repr,
    tail: Section,
    dx: i32,
    dy: i32,
    length: usize,
    evt: EventChannel,
}

impl Snek {
    pub fn new(tx: EventChannel) -> Snek {
        Snek {
            repr: vec![(0, 0)],
            tail: (0, 0),
            dx: 1,
            dy: 0,
            length: 1,
            evt: tx,
        }
    }

    pub fn mov(self) -> Snek {
        let (x, y) = self.repr[0];
        let new_x = x + self.dx;
        let new_y = y + self.dy;

        /* Get the screen bounds. */
        let mut max_x = 0;
        let mut max_y = 0;

        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        mvprintw(max_y-1, 0, format!("({}, {})[{}, {}]", new_x, new_y, max_x, max_y).as_str());
        refresh();

        // TODO: Is this really the appropriate place? Perhaps
        // this is better as a part of the display module?
        if new_x < 0 {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else if new_y < 0 {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else if new_x >= max_x {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else if new_y >= max_y {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else if self.repr.contains(&(new_x, new_y)) {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else {
            let mut r: Repr = vec![(new_x, new_y)];
            let mut old_repr = self.repr.clone();
            if old_repr.len() >= self.length {
                let (tail_x, tail_y) = old_repr.pop().unwrap();
                r.extend(old_repr);
                Snek{repr: r, tail: (tail_x, tail_y), dx: self.dx, dy: self.dy, length: self.length, evt: self.evt}
            } else {
                r.extend(old_repr);
                Snek{repr: r, tail: self.tail, dx: self.dx, dy: self.dy, length: self.length, evt: self.evt}
            }
        }
    }

    pub fn repr(&self) -> Repr {
        self.repr.clone()
    }

    pub fn tail(&self) -> Section {
        self.tail.clone()
    }

    pub fn up(self) -> Snek {
        self.evt.send(Event::Debug(format!("UP")));
        Snek {repr: self.repr, tail: self.tail, dx: 0, dy: -1, length: self.length, evt: self.evt}
    }

    pub fn down(self) -> Snek {
        self.evt.send(Event::Debug(format!("DOWN")));
        Snek {repr: self.repr, tail: self.tail, dx: 0, dy: 1, length: self.length, evt: self.evt}
    }

    pub fn left(self) -> Snek {
        self.evt.send(Event::Debug(format!("LEFT")));
        Snek {repr: self.repr, tail: self.tail, dx: -1, dy: 0, length: self.length, evt: self.evt}
    }

    pub fn right(self) -> Snek {
        self.evt.send(Event::Debug(format!("RIGHT")));
        Snek {repr: self.repr, tail: self.tail, dx: 1, dy: 0, length: self.length, evt: self.evt}
    }

    pub fn grow(self) -> Snek {
        self.evt.send(Event::Debug(format!("GROW:{}", self.length+1)));
        Snek {repr: self.repr, tail: self.tail, dx: self.dx, dy: self.dy, length: self.length+1, evt: self.evt}
    }
}

type Pos = (i32, i32);
pub struct Pill {
    pos: Pos,
}

impl Pill {
    pub fn new(r: Repr) -> Pill {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let mut rng = thread_rng();

        let x = rng.gen_range(0, max_x);
        let y = rng.gen_range(0, max_y);

        Pill { pos: (x, y)}
    }

    pub fn pos(&self) -> Pos {
        self.pos.clone()
    }
}
