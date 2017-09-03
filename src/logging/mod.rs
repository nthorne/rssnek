extern crate slog;
extern crate slog_stream;
extern crate slog_json;
//extern crate slog_term;

use std::fs::File;

use slog::{DrainExt, Logger};

pub fn setup() -> slog::Logger {
    let file = File::create("rssnek.log").expect("Could not open log file.");
    let file_drain = slog_stream::stream(file, slog_json::default()).fuse();
    //let drain = slog_term::streamer().compact().build().fuse();
    Logger::root(file_drain, o!("version" => "0.0.1"))
}
