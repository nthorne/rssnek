extern crate rssnek;
#[macro_use]
extern crate slog;

use rssnek::display;
use rssnek::input;
use rssnek::logging;
use rssnek::events;
use rssnek::objects;
use rssnek::game;

use std::{thread, time};
use std::sync::mpsc::{channel, Sender, Receiver};

#[allow(dead_code)]

fn main() {
    display::init();
    input::init();

    let logger = logging::setup();
    info!(logger, "Started application");

    // create the event dispatcher
    let mut dispatcher = events::Dispatcher::<game::Event>::new(&logger);
    // .. used for the snek to send on
    let evt = dispatcher.msg_tx.clone();

    // create the god channel, and subscribe..
    let god_channel = game::god(dispatcher.msg_tx.clone());
    dispatcher.subscribe(god_channel);

    // used to terminate the application
    let (tx, rx) = channel();
    dispatcher.subscribe(tx);

    input::input_loop(dispatcher.msg_tx.clone());

    // event loop..
    thread::spawn(move || {
        dispatcher.start();
    });

    loop {
        match rx.try_recv() {
            Ok(game::Event::Terminate) => break,
            _ => {},
        }
    }

    thread::sleep(time::Duration::from_secs(3));

    display::deinit();
}
