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
    LineReceived (String),
    Paused,
}

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
            delay_ms: 10,
        }
    }
}

// [ () -> T ] [ T -> () ]
// [ () -> T ] [ fn(T) -> U ] [ U -> () ]
// [ () -> T ] [ fn(T) -> U ] [ fn(U) -> T ] [ T -> () ]

impl<TContext, TSource> Runtime<TContext> for MockLineReader<TSource>
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    fn run(self, ctx: TContext) {
        let delay = self.delay_ms;
        let send = | value: String | {
            thread::sleep_ms(delay);
            ctx.dispatch(StdinEvents::Paused);
            ctx.dispatch(StdinEvents::LineReceived (value));
            ctx.dispatch(StdinEvents::Listening);
        };
        ctx.dispatch(trace!("producing mock lines"));
        ctx.dispatch(StdinEvents::Listening);
        send("foo".to_owned());
        send("bar".to_owned());
        send("baz".to_owned());
        for line in self.source.into_iter() {
            send(line.to_string());
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

    elm();
    // unthreaded();
    // threaded();
}

pub mod eventstore {
    use super::{ ElmModel };

    pub struct Model<TEvents> {
        inner: Vec<TEvents>
    }

    impl<TEvents> Default for Model<TEvents> {
        fn default() -> Self {
            Model {
                inner: Vec::new(),
            }
        }
    }

    impl<TEvents> ElmModel for Model<TEvents> {
        type TEvents = TEvents;

        fn update(&mut self, event: Self::TEvents) {
            self.inner.push(event);
        }
    }
}

pub mod concatview {
    use super::{ ElmModel };
    use std::marker::{ PhantomData };
    use std::fmt::{ Debug };

    #[derive(Debug)]
    pub enum Events {
        Foo (String),
    }

    pub struct Model<TEvents> {
        _events: PhantomData<TEvents>,
        value: String,
    }

    impl<TEvents> Default for Model<TEvents> {
        fn default() -> Self {
            Model {
                _events: PhantomData,
                value: String::default(),
            }
        }
    }

    impl<TEvents> ElmModel for Model<TEvents>
    where
        TEvents: Debug,
    {
        type TEvents = TEvents;

        fn update(&mut self, event: Self::TEvents) {
            self.value = format!("{}-{:?}", self.value, event);
        }
    }
}

pub struct Model {
    eventstore: eventstore::Model<StdinEvents>,// Vec<StdinEvents>,
    concatview: concatview::Model<StdinEvents>,// String::default(),
}

impl Default for Model {
    fn default() -> Self {
        Model {
            eventstore: eventstore::Model::default(),
            concatview: concatview::Model::default(),
        }
    }
}

pub trait ElmModel {
    type TEvents;

    fn update(&mut self, event: Self::TEvents);
}

// impl<TEvents, TModel> Sink for TModel
// where
//     TModel: ElmModel<TEvents=TEvents>,
// {
//     type TInput = TEvents;
//     type TResult = ();

//     fn send(&self, )
// }

impl ElmModel for Model {
    type TEvents = StdinEvents;

    fn update(&mut self, event: Self::TEvents) {
        self.eventstore.update(event.clone());
        self.concatview.update(event);
    }
}

type ElmRefCell<T: ElmModel> = RefCell<T>;

// impl<TModel> Sink for ElmRefCell<TModel>
// where
//     TModel: ElmModel,
// {
//     type TInput = TModel::TEvents;
//     type TResult = ();

//     fn send(&self, event: Self::TInput) -> Self::TResult {
//         self.borrow_mut().update(event);
//     }
// }

pub fn parse_stdin(input: StdinEvents) -> Option<concatview::Events> {
    match input {
        StdinEvents::LineReceived (line) => {
            if line.len() % 2 == 0 {
                None
            } else {
                Some (concatview::Events::Foo(line))
            }
        }
        _ => None
    }
}

pub struct UpdateViewRuntime {
}

impl UpdateViewRuntime {
    pub fn new() -> Self {
        UpdateViewRuntime {
        }
    }
}

impl Sink for UpdateViewRuntime {
    type TInput = StdinEvents;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        println!("UpdateView Runtime");
    }
}

fn elm() {
    let logging_sink = Logging::new();

    let model = RefCell::<Model>::default();

    // let event_sink = FnSink::new(|event: StdinEvents| {
    //     model.borrow_mut().update(event);
    // });
    let concat_event_sink = FnSink::new(|event: concatview::Events| {
        println!("Concat View: {:?}", event);
    });
    let event_sink = concat_event_sink
        .reduce()
        .map(parse_stdin)
        .map_result(|_| ());
    // let event_sink = FnSink::new(|event: StdinEvents| {
    //     eventstore_projection_sink.send(event.clone());
    //     concatview_projection_sink.send(event);
    // });
    
    // let source = StdinLineReader::new();
    let source = MockLineReader::new(&["foo", "bar", "fiz"]);

    source.bind(ctx! {
        logging: LoggingEvents = logging_sink,
        events: StdinEvents = event_sink,
        // events: StdinEvents = UpdateViewRuntime{},
    });
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
    });

    let writer = StdoutLineWriter::new();

    let arc_concatview = Arc::new(Mutex::new(String::default()));
    let mutex_concatview = arc_concatview.clone();
    let concatview_projection_sink = FnSink::new(move |event: StdinEvents| {
        let mut string = mutex_concatview.lock().unwrap();
        match event {
            StdinEvents::LineReceived (ref line) => {
                let value = format!("{}{}", string, line);
                *string = value.to_owned();
                writer.send(value);
            }
            _ => {}
        }
        Ok ((*string).to_owned())
    }).map_result(|value: Result<String, ()>| {
    });

    let event_sink = FnSink::new(move |event: StdinEvents| {
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