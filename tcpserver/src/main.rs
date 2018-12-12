// #[macro_use]
// extern crate log;
// #[macro_use]
extern crate logging;
// #[macro_use]
extern crate sink;
extern crate tcp_server;

pub mod runtimes;

use tcp_server::*;

fn main() {
    env::EnvConfigProvider::new();

    runtimes::domain::main();
    // elm();
    // unthreaded();
    // threaded();
}

// static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
// static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
// static HOST_PORT_KEY: &'static str = "HOST_PORT";
// static HOST_PORT_DEFAULT: &'static str = "8080";


// use runtimes::elm;

// use component::*;

// use std::fmt;
// use logging::{ Data, Logging, LoggingEvents };
// use env::*;
// use net::*;
// use sink::*;
// use sink::fnsink::{ FnSink };
// use stdio::*;

// use sink::vecsink::*;
// use server::{Events, Errors};

// use std::cell::RefCell;
// use std::io;
// use std::io::prelude::*;
// use std::io::{IoError, IoErrorKind};
// use std::io::{ BufRead, Read, Write };
// use std::io::{ BufRead, Stdin, Stdout, Write };
// use std::iter::{ IntoIterator };
// use std::sync::{ Arc, Mutex };
// use byteorder::{LittleEndian, ReadBytesExt};
// use std::thread;
// use std::thread::{ JoinHandle };
// use std::marker::{ PhantomData };


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


// pub struct UpdateViewRuntime {
// }

// impl UpdateViewRuntime {
//     pub fn new() -> Self {
//         UpdateViewRuntime {
//         }
//     }
// }

// impl Sink for UpdateViewRuntime {
//     type TInput = StdinEvents;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         println!("UpdateView Runtime");
//     }
// }


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