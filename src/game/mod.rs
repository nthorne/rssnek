use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time;

use std::io;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::prelude::*;

use display::Display;
use objects::{Snek, Pill};

use serde_json;
use serde_json::{from_str, to_string};

use std::string;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Score {
    pub name: String,
    pub score: i32,
}

pub struct HighscoreList {
    scores: Vec<Score>,
    max_entries: i32,
}

impl HighscoreList {
    pub fn new() -> HighscoreList {
        let mut buf = Vec::new();

        if let Ok(j) = File::open("highscore.json").map_err(ErrSum::FileError)
            .and_then({|mut s| s.read_to_end(&mut buf).map_err(ErrSum::FileError)})
            .and_then({|_| String::from_utf8(buf).map_err(ErrSum::ParseError)})
            .and_then({|s| (from_str(s.as_str()) as Result<Vec<Score>, serde_json::Error>)
                .map_err(ErrSum::SerdeError)}) {

                HighscoreList { scores: j, max_entries: 10 }
        } else {
            HighscoreList { scores: Vec::new(), max_entries: 10 }
        }
    }

    fn fits(&self, s: &i32) -> bool {
        self.scores.len() < self.max_entries as usize
            || self.scores.iter().any(|&ref e| e.score < *s)
    }


    fn update(&self, s: Score) -> HighscoreList {
        let mut new_scores = self.scores.clone();
        new_scores.push(s);
        new_scores.sort_by(|a, b| a.score.cmp(&b.score));

        if new_scores.len() > self.max_entries as usize {
            new_scores.reverse();
            new_scores.resize(self.max_entries as usize, Score{name: format!(""), score: 0});
        }

        HighscoreList { scores: new_scores, max_entries: 10 }
    }

    fn write(&self) {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("highscore.json")
            .and_then(
            {|mut s|
                s.write_all(to_string(&self.scores).unwrap_or(format!("")).as_bytes())
            }).unwrap();
    }

    fn show(&self, d: &Display) {
        d.show_highscore_list(&self.scores);
        thread::sleep(time::Duration::from_secs(3));
    }

    fn debug_dump(&self, d: &Display) {
        d.show_text(format!("{:?}", self.scores));
    }
}


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


#[derive(Debug)]
// Simple sum type used to map errors occurring during json parsing.
enum ErrSum {
    FileError(io::Error),
    ParseError(string::FromUtf8Error),
    SerdeError(serde_json::Error),
}

/// Responsible for life and death..
pub fn god(dis: Sender<Event>, inp_term: Sender<bool>) -> Sender<Event> {
    let (tx, rx) = channel();

    let delay = time::Duration::from_millis(50);


    thread::spawn(move || {
        let d = Display::new();
        d.dump();
        d.dump();

        let mut snek = Snek::new(dis.clone());

        let mut pill = Pill::new(&d);

        loop {
            if snek.repr().contains(&pill.pos()) {
                snek = snek.grow();
                pill = Pill::new(&d);
                dis.send(Event::Ate).unwrap();
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
                    inp_term.send(true).unwrap();
                    d.show_text(format!("SMITTEN"));

                    let score_list = HighscoreList::new();
                    let score = snek.score();
                    if score_list.fits(&score)
                    {
                        score_list.debug_dump(&d);
                        let name = d.input_dialog("Enter name", 30);
                        score_list.debug_dump(&d);
                        let new_score_list = score_list.update(
                            Score{name: name, score: score}
                            );
                        new_score_list.write();
                        new_score_list.show(&d);
                    }
                    else
                    {
                        score_list.show(&d);
                    }

                    dis.send(Event::Terminate).unwrap();
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
