use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::process;
use std::time;

use objects::{Snek, Pill};
use display;

// TODO: not to be used here..
extern crate ncurses;
use self::ncurses::*;


#[derive(PartialEq, Debug, Clone)]
pub enum Event {
    Up,
    Down,
    Left,
    Right,
    Tick,
    Death,
    Terminate,
    Grow,
    Ate,
    Debug(String),
}

/// Responsible for life and death..
pub fn god(dis: Sender<Event>) -> Sender<Event> {
    let (tx, rx) = channel();

    let delay = time::Duration::from_millis(50);


    let chld = thread::spawn(move || {
        let mut snek = Snek::new(dis.clone());
        let mut pill = Pill::new(snek.repr());


        loop {
            if snek.repr().contains(&pill.pos()) {
                snek = snek.grow();
                pill = Pill::new(snek.repr());
                dis.send(Event::Ate);
            }

            display::show_pill(&pill);

            snek = snek.mov();
            display::show_snek(&snek);
            thread::sleep(delay);

            match rx.try_recv() {

                Ok(Event::Up) => snek = snek.up(),
                Ok(Event::Down) => snek = snek.down(),
                Ok(Event::Left) => snek = snek.left(),
                Ok(Event::Right) => snek = snek.right(),
                Ok(Event::Grow) => snek = snek.grow(),
                Ok(Event::Death) => {
                    mvprintw(getmaxy(stdscr())/2, getmaxx(stdscr())/2, "SMITTEN!");
                    refresh();
                    dis.send(Event::Terminate);
                    break;
                },
                Ok(Event::Terminate) => break,
                Ok(Event::Debug(s)) => {
                    mvprintw(getmaxy(stdscr())-2, 0, s.as_str());
                }
                _ => ()

            }
        }
    });

    tx
}
