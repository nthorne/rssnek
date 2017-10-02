use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::process;
use std::time;

use display::Display;
use objects::{Snek, Pill};


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
        let d = Display::new();
        d.dump();
        d.dump();

        let mut snek = Snek::new(dis.clone(), &d);

        let mut pill = Pill::new(&d);

        loop {
            if snek.repr().contains(&pill.pos()) {
                snek = snek.grow();
                pill = Pill::new(&d);
                dis.send(Event::Ate);
            }

            d.show_pill(&pill);

            snek = snek.mov(&d);
            d.show_snek(&snek);
            thread::sleep(delay);

            match rx.try_recv() {

                Ok(Event::Up) => snek = snek.up(),
                Ok(Event::Down) => snek = snek.down(),
                Ok(Event::Left) => snek = snek.left(),
                Ok(Event::Right) => snek = snek.right(),
                Ok(Event::Grow) => snek = snek.grow(),
                Ok(Event::Death) => {
                    d.show_text(format!("SMITTEN"));
                    dis.send(Event::Terminate);
                    break;
                },
                Ok(Event::Terminate) => break,
                Ok(Event::Debug(s)) => {
                    d.show_text(s);
                }
                _ => ()

            }
        }
    });

    tx
}
