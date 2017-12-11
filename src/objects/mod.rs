use super::game::{Event};
use super::display::Display;
use std::sync::mpsc::{Sender};

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
            repr: vec![(1, 1)],
            tail: (0, 0),
            dx: 1,
            dy: 0,
            length: 1,
            evt: tx,
        }
    }

    pub fn mov(self, d: &Display) -> Snek {
        let (x, y) = self.repr[0];
        let new_x = x + self.dx;
        let new_y = y + self.dy;

        if !d.in_main_window(new_x, new_y) {
            self.evt.send(Event::Death).unwrap();
            Snek{repr: self.repr, tail: self.tail, dx: 0, dy: 0, length: self.length, evt: self.evt}
        } else {
            let mut r: Repr = vec![(new_x, new_y)];
            let mut old_repr = self.repr.clone();
            if old_repr.len() >= self.length {
                let (tail_x, tail_y) = old_repr.pop().unwrap();

                if old_repr.contains(&(new_x, new_y)) {
                    self.evt.send(Event::Death).unwrap();
                }

                r.extend(old_repr);
                Snek{repr: r, tail: (tail_x, tail_y), dx: self.dx, dy: self.dy, length: self.length, evt: self.evt}
            } else {
                if old_repr.contains(&(new_x, new_y)) {
                    self.evt.send(Event::Death).unwrap();
                }

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
        self.evt.send(Event::Debug(format!("UP"))).unwrap();
        Snek {repr: self.repr, tail: self.tail, dx: 0, dy: -1, length: self.length, evt: self.evt}
    }

    pub fn down(self) -> Snek {
        self.evt.send(Event::Debug(format!("DOWN"))).unwrap();
        Snek {repr: self.repr, tail: self.tail, dx: 0, dy: 1, length: self.length, evt: self.evt}
    }

    pub fn left(self) -> Snek {
        self.evt.send(Event::Debug(format!("LEFT"))).unwrap();
        Snek {repr: self.repr, tail: self.tail, dx: -1, dy: 0, length: self.length, evt: self.evt}
    }

    pub fn right(self) -> Snek {
        self.evt.send(Event::Debug(format!("RIGHT"))).unwrap();
        Snek {repr: self.repr, tail: self.tail, dx: 1, dy: 0, length: self.length, evt: self.evt}
    }

    pub fn grow(self) -> Snek {
        self.evt.send(Event::Debug(format!("GROW:{}", self.length+1))).unwrap();
        Snek {repr: self.repr, tail: self.tail, dx: self.dx, dy: self.dy, length: self.length+1, evt: self.evt}
    }

    pub fn score(self) -> i32 {
        self.length as i32
    }
}

type Pos = (i32, i32);
pub struct Pill {
    pos: Pos,
}

impl Pill {
    pub fn new(d: &Display) -> Pill {
        let x = d.get_x_in_main();
        let y = d.get_y_in_main();
        return Pill { pos: (x, y)}
    }

    pub fn pos(&self) -> Pos {
        self.pos.clone()
    }
}
