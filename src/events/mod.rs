extern crate slog;

use std::fmt::Debug;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


use slog::Logger;


#[allow(dead_code)]
/// A simple message dispatcher; broadcasts received messages
/// onto all subscribing channels.
pub struct Dispatcher<T> {
    /// The channels to broadcast messages onto.
    channels: Vec<Sender<T>>,
    logger: slog::Logger,
    /// The channel to clone and broadcast messages on.
    msg_tx: Sender<T>,
    msg_rx: Receiver<T>,
    /// If a unit is send on this channel, then the message dispatcher will
    /// exit its loop.
    term_tx: Sender<()>,
    term_rx: Receiver<()>,
}

impl <T>Dispatcher<T> where T: Send + Clone + Debug + 'static {
    /// Construct the Dispatcher, and its message, and termination channels.
    pub fn new(root_logger: &Logger) -> Dispatcher<T> {
        let (tx, rx) = mpsc::channel();
        let (ttx, trx) = mpsc::channel();

        Dispatcher {
            channels: Vec::new(),
            logger: root_logger.clone(),
            msg_tx: tx,
            msg_rx: rx,
            term_tx: ttx,
            term_rx: trx,
        }
    }

    /// Start the Dispatcher's event loop.
    ///
    /// The dispatcher will try non-blocking reads on both the termination channel,
    /// and the message channel. If a unit is received on the termination channel,
    /// then the dispatcher will leave exit the event loop without handling any
    /// pending message. If, however, a Event message is read, then that message
    /// will be broadcasted on all (non-closed; should never happen, but) subscribing
    /// channels. If a send fails, then that channel is dropped from the list of
    /// subscribing channels.
    pub fn start(&mut self) {
        info!(self.logger, "Starting message dispatcher loop..");

        loop {
            if let Ok(_) = self.term_rx.try_recv() {
                info!(self.logger, "Got termination signal. Shutting down event loop.");
                break;
            }
            if let Ok(msg) = self.msg_rx.try_recv() {
                info!(self.logger, "Got message"; "msg" => format!("{:?}", msg));

                let current_channels = self.channels.clone();
                self.channels.clear();

                for c in current_channels {
                    if let Ok(_) = c.send(msg.clone()) {
                        info!(self.logger, "Sending to channel"; "chan" => format!("{:?}", c));
                        self.channels.push(c);
                    } else {
                        info!(self.logger, "Channel closed. Dropping it."; "chan" => format!("{:?}", c));
                    }
                }
            }
        }
        info!(self.logger, "Exited event loop..");
    }

    /// Add a subscribing channel.
    pub fn subscribe(&mut self, channel: Sender<T>) {
        info!(self.logger, "Adding subscriber"; "channel" => format!("{:?}", channel));

        self.channels.push(channel);
    }
}


#[allow(unused_variables)]
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    extern crate slog;
    extern crate slog_term;

    use std::sync::mpsc::channel;
    use events::{Dispatcher};
    use slog::{DrainExt, Logger};
    use std::thread;

    #[derive(Clone, Debug, PartialEq)]
    enum TestEvent {
        Event1,
        Event2,
        Event3,
    }

    struct TestSubscriber {
        pub result: bool,
    }

    fn construct_dispatcher() -> Dispatcher<TestEvent> {
        let logger = Logger::root(slog_term::streamer().build().fuse(),
                                  o!());
        Dispatcher::<TestEvent>::new(&logger)
    }

    #[test]
    fn test_start_loop() {
        let mut dis = construct_dispatcher();

        let tx = dis.msg_tx.clone();
        let term = dis.term_tx.clone();

        let chld = thread::spawn(move || {
            dis.start();
        });

        assert_eq!(true, tx.send(TestEvent::Event1).is_ok());

        assert_eq!(true, term.send(()).is_ok());
    }

    #[test]
    fn test_subscribe() {
        let mut dis = construct_dispatcher();

        let tx = dis.msg_tx.clone();
        let term = dis.term_tx.clone();

        let (sub1_tx, sub1_rx) = channel();
        dis.subscribe(sub1_tx);

        let chld = thread::spawn(move || {
            dis.start();
        });

        tx.send(TestEvent::Event1).unwrap();
        assert_eq!(Ok(TestEvent::Event1), sub1_rx.recv());

        assert_eq!(true, term.send(()).is_ok());
    }

    #[test]
    fn test_subscribing_to_multiple_messages() {
        let mut dis = construct_dispatcher();

        let tx = dis.msg_tx.clone();
        let term = dis.term_tx.clone();

        let (sub1_tx, sub1_rx) = channel();
        let (sub2_tx, sub2_rx) = channel();

        dis.subscribe(sub1_tx);
        dis.subscribe(sub2_tx);

        let chld = thread::spawn(move || {
            dis.start();
        });

        tx.send(TestEvent::Event1).unwrap();

        assert_eq!(Ok(TestEvent::Event1), sub1_rx.recv());
        assert_eq!(Ok(TestEvent::Event1), sub2_rx.recv());

        assert_eq!(true, term.send(()).is_ok());
    }

    #[test]
    fn test_dispatching_multiple_messages() {
        let mut dis = construct_dispatcher();

        let tx = dis.msg_tx.clone();
        let term = dis.term_tx.clone();

        let (sub1_tx, sub1_rx) = channel();
        dis.subscribe(sub1_tx);

        let chld = thread::spawn(move || {
            dis.start();
        });

        tx.send(TestEvent::Event1).unwrap();
        assert_eq!(Ok(TestEvent::Event1), sub1_rx.recv());

        tx.send(TestEvent::Event2).unwrap();
        assert_eq!(Ok(TestEvent::Event2), sub1_rx.recv());

        tx.send(TestEvent::Event3).unwrap();
        assert_eq!(Ok(TestEvent::Event3), sub1_rx.recv());

        assert_eq!(true, term.send(()).is_ok());
    }
}
