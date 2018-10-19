use sink::{ ISink };

#[derive(Debug)]
pub enum LoggingEvents {
    Error (String),
    Info (String),
    Warning (String),
}

pub trait ILoggingSink:
    ISink<TInput=LoggingEvents, TResult=()>
{}

impl<T> ILoggingSink for T
where
    T: ISink<TInput=LoggingEvents, TResult=()>
{}

#[derive(Clone)]
pub struct Logging {}

impl ISink for Logging {
    type TInput = LoggingEvents;
    type TResult = ();

    fn send(&self, input: LoggingEvents) -> () {
        match input {
            LoggingEvents::Info (msg) => println!("info: {}", msg),
            _ => {}
        }
    }
}