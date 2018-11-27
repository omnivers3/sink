// #[macro_use]
// extern crate logging;

use std::fmt::Debug;
use std::io::{ Stderr, Stdin, Stdout };

pub struct StdIOLines {
    buffer: Option<String>,
    stderr: Stderr,
    stdin: Stdin,
    stdout: Stdout,
}

impl StdIOLines {
    pub fn new() -> Self {
        StdIOLines {
            buffer: None,
            stderr: io::stderr(),
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }
}

impl Sink for StdIOLines {
    type TInput = String;
    type TResult = Result<(), io::Error>;

    fn send(&self, value: Self::TInput) -> Self::TResult {
        let mut lock = self.stdout.lock();
        let result = lock.write_all(value.as_bytes());
        return result;
    }
}

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Listening,
    LineReceived (String),
    Paused,
}

impl Source for StdIOLines {
    type TOutput = StdinEvents;

    fn next(&self) -> Self::TOutput {
        self.send(StdinEvents::Listening);
        let lock = stdin.lock();
        for line in lock.lines() {
            match line {
                Err (err) => {
                    // ctx.dispatch(error!("error reading stdin: {:?}", err));
                    break;
                }
                Ok (line) => {
                    // ctx.dispatch(trace!("received line [{:?}]", line));
                    // ctx.dispatch(StdinEvents::LineReceived (line));
                    self.send(StdinEvents::LineReceived (line));
                }
            }
        }
        self.send(StdinEvents::Paused);
    }
}

pub trait Environment {
    type TEvents;

    fn view(&self);
    fn update(&mut self, event: Self::TEvents);
}

impl Environment for StdIOLines {
    type TEvents = String;

    fn view(&self) {
        match self.buffer {
            None => return,
            Some (line) => self.send(line).unwrap(),
        }
    }

    fn update(&mut self, event: Self::TEvents) {
        self.buffer = Some(format!("Event: {}", event));
    }
}

pub trait Runtime {
    type TSignal;

    fn run<FUpdate>(self, update: FUpdate) where FUpdate: Fn(Self::TSignal);
}

impl Runtime for StdIOLines {
    type TSignal = String;

    fn run<FUpdate>(&mut self, update: FUpdate)
    where
        FUpdate: FnMut(Self::TSignal),
    {
        let mut counter = 0;
        loop {
            update(format!("{}", counter));
            counter += 1;
        }
    }
}

pub struct RuntimeContainer {
    inner: TInner,
}

// impl Runtime {
//     pub fn run(self, )
//}

fn main() {
    StdIOLines::new()
        .run(| state, event | {
            format!("line: {}", line)
        });
}
