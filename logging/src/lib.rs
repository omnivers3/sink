extern crate log;
extern crate sink;

#[macro_use]
mod macros;

pub use log::{ logger, Level, Record };
use sink::Sink;

#[derive(Clone, Debug)]
pub struct Data {
    target: String,
    value: String,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
}

impl Data {
    pub fn full(target: String, value: String, module_path: String, file: String, line: u32) -> Self {
        Data {
            target,
            value,
            module_path: Some(module_path),
            file: Some(file),
            line: Some(line),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoggingEvents {
    Debug(Data),
    Error(Data),
    Info(Data),
    Trace(Data),
    Warning(Data),
}

impl LoggingEvents {
    pub fn log<'a>(&'a self) {
        let (level, data) = match self {
            LoggingEvents::Debug(data) => (Level::Debug, data),
            LoggingEvents::Info(data) => (Level::Info, data),
            LoggingEvents::Error(data) => (Level::Error, data),
            LoggingEvents::Trace(data) => (Level::Trace, data),
            LoggingEvents::Warning(data) => (Level::Warn, data),
        };
        logger().log(&Record::builder()
            .level(level)
            .target(&data.target)
            .module_path(data.module_path.as_ref().map(|x| &**x))
            .file(data.file.as_ref().map(|x| &**x))
            .line(data.line)
            .args(format_args!("{}", data.value))
            .build()
        );
    }
}

pub trait LoggingSink: Sink<TInput = LoggingEvents, TResult = ()> {}

impl<T> LoggingSink for T where T: Sink<TInput = LoggingEvents, TResult = ()> {}

#[derive(Clone)]
pub struct Logging {}

impl<'a> Logging {
    pub fn new() -> Self {
        Logging {}
    }
}

impl Sink for Logging {
    type TInput = LoggingEvents;
    type TResult = ();

    fn send(&self, input: LoggingEvents) -> () {
        input.log();
    }
}