extern crate rssnek;
#[macro_use]
extern crate slog;

use rssnek::display;
use rssnek::input;
use rssnek::logging;
use rssnek::events;


/* TODO:
 *   Create an event dispatcher; this dispatcher should dispatch Event to any
 *   subscribers. API needs to contain subscribe, unsubscribe and dispatch.
 *   On Event, the subscriber needs the instance of the dispatcher in order
 *   to be able to dispatch new events.
 *
 *   All game events should be triggered by Tick events. The Tick events should
 *   be dispatched by a timer thread.
 */

#[allow(dead_code)]
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Event {
    Tick,
}

fn main() {
display::init();

let logger = logging::setup();
info!(logger, "Started application");

let mut dispatcher = events::Dispatcher::<Event>::new(&logger);
dispatcher.start();

loop {
    input::process_input();
    display::show();
}

//display::deinit();
}
