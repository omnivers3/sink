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
use std::io::{ BufRead };
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

// fn main() {
//     env::EnvConfigProvider::new();

//     let logging = Logging::new();

//     {
//         use product::*;

//         let system = Component::bind(context!{
//             logging: LoggingEvents | () = logging,
//         });

//         system.send(Commands::Register (RegistrationData { name: "foo" }));
//         system.send(Commands::Disable);
//         system.send(Commands::ReEnable);
//     }
// }

// pub struct CommandMeta<'a> {
//     key: &'a str,
// }

// #[derive(Debug)]
// pub enum StdinCommands {
//     Listen,
// }

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Listening,
    // Terminated,
    LineReceived (String),
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

pub struct StdinLineReader {}

impl StdinLineReader {
    pub fn new() -> Self {
        StdinLineReader {}
    }
}

impl<TContext> Runtime<TContext> for StdinLineReader
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
{
    type TResult = ();

    fn run(self, ctx: TContext) -> Self::TResult {
        let stdin = io::stdin();
        ctx.dispatch(trace!("blocking on stdin"));
        ctx.dispatch(StdinEvents::Listening);
        let lock = stdin.lock();
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
    }
}

pub struct MockLineReader<TSource>
where
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    source: TSource,
}

impl<TSource> MockLineReader<TSource>
where
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    pub fn new(source: TSource) -> Self {
        MockLineReader {
            source
        }
    }
}

impl<TContext, TSource> Runtime<TContext> for MockLineReader<TSource>
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    type TResult = ();

    fn run(self, ctx: TContext) -> Self::TResult {
        ctx.dispatch(trace!("producing mock lines"));
        ctx.dispatch(StdinEvents::LineReceived ("foo".to_owned()));
        ctx.dispatch(StdinEvents::LineReceived ("bar".to_owned()));
        ctx.dispatch(StdinEvents::LineReceived ("baz".to_owned()));
        for line in self.source.into_iter() {
            ctx.dispatch(StdinEvents::LineReceived (line.to_string()))
        }
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
        println!("Pushed event into index: [{:?}]", index.unwrap());
    });

    let concatview = RefCell::new(String::default());
    let concatview_projection_sink = FnSink::new(|event: StdinEvents| {
        let mut concatview = concatview.borrow_mut();
        match event {
            StdinEvents::LineReceived (ref line) => {
                *concatview = format!("{}{}", concatview, line);
            }
            _ => {}
        }
        Ok (concatview.to_owned())
    }).map_result(|value: Result<String, ()>| {
        println!("Appended event with resulting value: [{:?}]", value.unwrap());
    });

    let event_sink = FnSink::new(move |event: StdinEvents| {
        println!("Stdin Line Reader Event Sink: {:?}", event);
        eventstore_projection_sink.send(event.clone());
        concatview_projection_sink.send(event);
    });
    
    // let runtime = StdinLineReader::new();
    let runtime = MockLineReader::new(&["asdf", "blah", "bloo"]);

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
        println!("Pushed event into index: [{:?}]", index.unwrap());
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
        println!("Appended event with resulting value: [{:?}]", value.unwrap());
    });

    let event_sink = FnSink::new(move |event: StdinEvents| {
        println!("Stdin Line Reader Event Sink: {:?}", event);
        eventstore_projection_sink.send(event.clone());
        concatview_projection_sink.send(event);
    });
    
    // let runtime = StdinLineReader::new();
    let runtime = MockLineReader::new(&["asdf", "blah", "bloo"]);

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
