#[macro_use]
extern crate log;
extern crate sink;

use std::fmt;
use std::marker::{ PhantomData };

use log::*;
use sink::Sink;

#[derive(Clone, Debug)]
pub struct Data<'a> {
    target: &'a str,
    args: fmt::Arguments<'a>,
    module_path: Option<&'a str>,
    file: Option<&'a str>,
    line: Option<u32>,
}

#[derive(Clone, Debug)]
pub enum LoggingEvents<'a> {
    Debug(Data<'a>),
    Error(Data<'a>),
    Info(Data<'a>),
    Trace(Data<'a>),
    Warning(Data<'a>),
}

impl<'a> From<LoggingEvents<'a>> for Record<'a> {
    fn from(event: LoggingEvents<'a>) -> Record<'a> {
        let mut builder = Record::builder();
        let (level, data) = match event {
            LoggingEvents::Debug(data) => (Level::Debug, data),
            LoggingEvents::Info(data) => (Level::Info, data),
            LoggingEvents::Error(data) => (Level::Error, data),
            LoggingEvents::Trace(data) => (Level::Trace, data),
            LoggingEvents::Warning(data) => (Level::Warn, data),
        };
        builder.args(data.args);
        builder.level(level);
        builder.target(data.target);
        builder.file(data.file);
        builder.line(data.line);
        builder.build()
    }
}

pub trait LoggingSink<'a>: Sink<TInput = LoggingEvents<'a>, TResult = ()> {}

impl<'a, T> LoggingSink<'a> for T where T: Sink<TInput = LoggingEvents<'a>, TResult = ()> {}

#[derive(Clone)]
pub struct Logging<'a> {
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Logging<'a> {
    pub fn new() -> Self {
        Logging {
            _lifetime: PhantomData,
        }
    }
}

impl<'a> Sink for Logging<'a> {
    type TInput = LoggingEvents<'a>;
    type TResult = ();

    fn send(&self, input: LoggingEvents<'a>) -> () {
        let record: Record = input.into();
        logger().log(&record);
    }
}