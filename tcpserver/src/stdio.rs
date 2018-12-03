use component::{ Runtime };
use logging::{ Data, LoggingEvents };
use sink::*;
use fnsink::{ FnSink };

use std::fmt;
use std::io;
use std::io::{ BufRead, Stdin, Stdout, Write };
use std::time::{ Duration };
use std::thread;

#[derive(Clone, Debug)]
pub enum StdinCommands {
    Start,
}

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Listening,
    LineReceived (String),
    Stopped,
}

#[derive(Clone, Debug)]
pub enum Events {
    Logging (LoggingEvents),
    Stdin (StdinEvents),
}

#[derive(Clone, Debug)]
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
    fn run(self, ctx: TContext) {
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
        ctx.dispatch(StdinEvents::Stopped);
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

impl<TContext, TSource> Runtime<TContext> for MockLineReader<TSource>
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
    TSource: IntoIterator,
    TSource::Item: ToString,
{
    fn run(self, ctx: TContext) {
        let delay = self.delay_ms;
        let send = | value: String | {
            thread::sleep(Duration::from_millis(delay.into()));
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
        ctx.dispatch(StdinEvents::Stopped);
    }
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

// pub struct Actor<'a, TContext> {
//     ctx: &'a TContext,
// }

// impl<'a, TContext> Actor<'a, TContext> {
//     pub fn new(ctx: &'a TContext) -> Self {
//         Actor {
//             ctx,
//         }
//     }
// }

// // pub trait ActorDef<TContext> {
// //     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext>;
// // }

// // impl<TContext> ActorDef<TContext> for StdinLineReader
// // where
// //     TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
// // {
// //     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext> {
// //         Actor::new(ctx)
// //     }
// // }

// pub trait ActorDef<TContext>
// where
//     Self: Sized,
// {
//     fn bind<'a>(self, ctx: &'a TContext) -> Actor<'a, TContext> {
//         Actor::new(ctx)
//     }

//     fn run<'a>(&self, ctx: &'a TContext);
// }

// impl<TContext> ActorDef<TContext> for StdinLineReader
// where
//     TContext: Dispatcher<LoggingEvents> + Dispatcher<StdinEvents>,
// {
//     fn run<'a>(&self, ctx: &'a TContext) {
//         ctx.dispatch(trace!("blocking on stdin"));
//         ctx.dispatch(StdinEvents::Listening);
//         let lock = self.stdin.lock();
//         for line in lock.lines() {
//             match line {
//                 Err (err) => {
//                     ctx.dispatch(error!("error reading stdin: {:?}", err));
//                     break;
//                 }
//                 Ok (line) => {
//                     ctx.dispatch(trace!("received line [{:?}]", line));
//                     ctx.dispatch(StdinEvents::LineReceived (line));
//                 }
//             }
//         }
//         ctx.dispatch(StdinEvents::Paused);
//     }
// }
