// #[macro_use]
// extern crate log;
#[macro_use]
extern crate logging;
#[macro_use]
extern crate sink;
extern crate tcp_server;

use component::*;

use std::fmt;
use logging::{ Data, Logging, LoggingEvents };
// use env::*;
// use net::*;
use sink::*;
use sink::fnsink::{ FnSink };

// use sink::vecsink::*;
// use server::{Events, Errors};

use std::cell::RefCell;
use std::io;
// use std::io::prelude::*;
// use std::io::{IoError, IoErrorKind};
// use std::io::{ BufRead, Read, Write };
use std::io::{ BufRead, Stdin, Stdout, Write };
use std::iter::{ IntoIterator };
use std::sync::{ Arc, Mutex };
// use byteorder::{LittleEndian, ReadBytesExt};
use tcp_server::*;
use std::thread;
// use std::thread::{ JoinHandle };
use std::marker::{ PhantomData };

// static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
// static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
// static HOST_PORT_KEY: &'static str = "HOST_PORT";
// static HOST_PORT_DEFAULT: &'static str = "8080";

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Listening,
    // Terminated,
    LineReceived (String),
    Paused,
}

// #[derive(Debug)]
// pub enum StdinErrors {
//     AlreadyListening,
// }

// pub struct StdinBufferReader {

// }

// impl StdinBufferReader {
//     pub fn new()
// }

pub struct StdoutLineWriter {
    stdout: Stdout,
}

impl StdoutLineWriter {
    pub fn new() -> Self {
        StdoutLineWriter {
            stdout: io::stdout(),
        }
    }
}

impl Sink for StdoutLineWriter {
    type TInput = String;
    type TResult = Result<(), io::Error>;

    fn send(&self, value: Self::TInput) -> Self::TResult {
        let mut lock = self.stdout.lock();
        write!(lock, "{}\n", value)
    }
}

pub struct StdinLineReader {
    stdin: Stdin,
}

impl StdinLineReader {
    pub fn new() -> Self {
        StdinLineReader {
            stdin: io::stdin(),
        }
    }
}

impl<TContext> Runtime<TContext> for StdinLineReader
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
{
    fn run(self, ctx: TContext) {
        loop {
            ctx.dispatch(trace!("blocking on stdin"));
            ctx.dispatch(StdinEvents::Listening);
            let lock = self.stdin.lock();
            for line in lock.lines() {
                match line {
                    Err (err) => {
                        ctx.dispatch(error!("error reading stdin: {:?}", err));
                        break;
                    }
                    Ok (line) => {
                        ctx.dispatch(trace!("received line [{:?}]", line));
                        ctx.dispatch(StdinEvents::LineReceived (line));
                    }
                }
            }
            ctx.dispatch(StdinEvents::Paused);
        }
    }
}

pub struct MockLineReader<TSource>
where
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    source: TSource,
    delay_ms: u32,
}

impl<TSource> MockLineReader<TSource>
where
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    pub fn new(source: TSource) -> Self {
        MockLineReader {
            source,
            delay_ms: 1,
        }
    }
}

impl<TContext, TSource> Runtime<TContext> for MockLineReader<TSource>
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    fn run(self, ctx: TContext) {
        ctx.dispatch(trace!("producing mock lines"));
        thread::sleep_ms(self.delay_ms);
        ctx.dispatch(StdinEvents::LineReceived ("foo".to_owned()));
        thread::sleep_ms(self.delay_ms);
        ctx.dispatch(StdinEvents::LineReceived ("bar".to_owned()));
        thread::sleep_ms(self.delay_ms);
        ctx.dispatch(StdinEvents::LineReceived ("baz".to_owned()));
        for line in self.source.into_iter() {
            thread::sleep_ms(self.delay_ms);
            ctx.dispatch(StdinEvents::LineReceived (line.to_string()))
        }
    }
}

pub struct FooSource {
    counter: RefCell<u32>,
}

impl FooSource {
    pub fn new() -> Self {
        FooSource {
            counter: RefCell::default(),
        }
    }
}

impl Source for FooSource {
    type TOutput = String;

    fn next(&self) -> Self::TOutput {
        thread::sleep_ms(1000);
        let mut counter = self.counter.borrow_mut();
        *counter += 1;
        format!("foo {}\n", counter)
    }
}

fn main() {
    env::EnvConfigProvider::new();

    unthreaded();
    // threaded();
}

fn unthreaded() {
    let logging_sink = Logging::new();

    let eventstore = RefCell::new(Vec::new());
    let eventstore_projection_sink = FnSink::new(|event: StdinEvents| {
        let mut eventstore = eventstore.borrow_mut();
        eventstore.push(event);
        Ok (eventstore.len())
    }).map_result(|index: Result<usize, ()>| {
    });

    let writer = StdoutLineWriter::new();

    let concatview = RefCell::new(String::default());
    let concatview_projection_sink = FnSink::new(|event: StdinEvents| {
        let mut concatview = concatview.borrow_mut();
        match event {
            StdinEvents::LineReceived (ref line) => {
                let value = format!("{}{}", concatview, line);
                *concatview = value.to_owned();
                writer.send(value);
            }
            _ => {}
        }
        Ok (concatview.to_owned())
    }).map_result(|value: Result<String, ()>| {
    });

    let event_sink = FnSink::new(|event: StdinEvents| {
        //println!("Stdin Line Reader Event Sink: {:?}", event);
        eventstore_projection_sink.send(event.clone());
        concatview_projection_sink.send(event);
    });
    
    // let runtime = StdinLineReader::new();
    let runtime = MockLineReader::new(&["foo", "bar", "fiz"]);

    runtime.run(ctx! {
        logging: LoggingEvents = logging_sink,
        events: StdinEvents = event_sink,
    });

    println!("EventStore: {:?}", eventstore.borrow());
    println!("ConcatView: {:?}", concatview.borrow());
}

fn threaded() {
    let logging_sink = Logging::new();

    let arc_eventstore = Arc::new(Mutex::new(Vec::new()));
    let mutex_eventstore = arc_eventstore.clone();
    let eventstore_projection_sink = FnSink::new(move |event: StdinEvents| {
        let mut eventstore = mutex_eventstore.lock().unwrap();
        eventstore.push(event);
        Ok (eventstore.len())
    }).map_result(|index: Result<usize, ()>| {
        //println!("Pushed event into index: [{:?}]", index.unwrap());
    });

    let arc_concatview = Arc::new(Mutex::new(String::default()));
    let mutex_concatview = arc_concatview.clone();
    let concatview_projection_sink = FnSink::new(move |event: StdinEvents| {
        let mut string = mutex_concatview.lock().unwrap();
        match event {
            StdinEvents::LineReceived (ref line) => {
                *string += line;
            }
            _ => {}
        }
        Ok ((*string).to_owned())
    }).map_result(|value: Result<String, ()>| {
        //println!("Appended event with resulting value: [{:?}]", value.unwrap());
    });

    // eventstore_projection_sink.lift(|state, event| {
    //     let mut eventstore = state.lock().unwrap();
    //     eventstore.push(event);
    //     Ok (eventstore.len())
    // });

    let event_sink = FnSink::new(move |event: StdinEvents| {
        //println!("Stdin Line Reader Event Sink: {:?}", event);
        eventstore_projection_sink.send(event.clone());
        concatview_projection_sink.send(event);
    });
    
    // let runtime = StdinLineReader::new();
    let runtime = MockLineReader::new(&["foo", "bar", "fiz"]);

    let handle = thread::spawn(move || {
        runtime.run(ctx! {
            logging: LoggingEvents = logging_sink,
            events: StdinEvents = event_sink,
        });
    });
    let result = handle.join();

    let mutex_eventstore = arc_eventstore.clone();
    let eventstore = mutex_eventstore.lock().unwrap();

    let mutex_concatview = arc_concatview.clone();
    let concatview = mutex_concatview.lock().unwrap();

    println!("Thread Result: {:?}", result);
    println!("EventStore: {:?}", *eventstore);
    println!("ConcatView: {:?}", *concatview);
}


// impl<TContext> Runtime<TContext> for StdoutLineWriter
// where
//     TContext: Source<TOutput=String>,
// {
//     type TResult = io::Result<()>;

//     fn run(self, ctx: TContext) {//} -> Self::TResult {
//         loop {
//             println!("blocking on write receive");
//             let value = ctx.next();
//             let stdout = io::stdout();
//             let mut lock = stdout.lock();
//             let result = lock.write_all(value.as_bytes());
//             if result.is_ok() { continue; }
//             return result;
//         }
//     }
// }

// pub trait IORuntime {
//     fn run(self) -> Self::TResult;
// }

// pub struct StdIORuntime {}

// impl Source for StdIORuntime {
//     type TOutput = String;

//     fn next(&self) -> Self::TOutput {
//         let stdin = io::stdin();
//         self.send(StdinEvents::Listening);
//         let lock = stdin.lock();
//         for line in lock.lines() {
//             match line {
//                 Err (err) => {
//                     // ctx.dispatch(error!("error reading stdin: {:?}", err));
//                     break;
//                 }
//                 Ok (line) => {
//                     // ctx.dispatch(trace!("received line [{:?}]", line));
//                     // ctx.dispatch(StdinEvents::LineReceived (line));
//                     self.send(StdinEvents::LineReceived (line));
//                 }
//             }
//         }
//         self.send(StdinEvents::Paused);
//     }
// }

// impl Sink for StdIORuntime {
//     type TInput = String;
//     type TResult = Result<(), io::Error>;

//     fn send(&self, value: Self::TInput) -> Self::TResult {
//         println!("blocking on write receive");
//         let stdout = io::stdout();
//         let mut lock = stdout.lock();
//         lock.write_all(value.as_bytes())
//         // let result = lock.write_all(value.as_bytes());
//         // if result.is_ok() { continue; }
//         // return result;
//     }
// }

// impl Runtime for StdIORuntime {
// impl StdIORuntime {
//     pub fn run(self) {
//         loop {
//             self.send(self.next());
//         }
//     }
// }

// impl<TContext> Runtime<TContext> for EventLoop
// where
//     TContext: Source<TOutput=T, TResult=()>, Dispa
// {
//     type TResult = ();

//     fn run(self, ctx: TContext) -> Self::TResult {
//         loop {
//             ctx.dispatch(ctx.next());
//         }
//     }
// }
