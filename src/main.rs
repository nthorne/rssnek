extern crate rssnek;

use rssnek::display;
use rssnek::input;


/* TODO:
 *   Create an event dispatcher; this dispatcher should dispatch Event to any
 *   subscribers. API needs to contain subscribe, unsubscribe and dispatch.
 *   On Event, the subscriber needs the instance of the dispatcher in order
 *   to be able to dispatch new events.
 */

fn main() {
    display::init();

    loop {
        input::process_input();
        display::show();
    }

    display::deinit();
}
