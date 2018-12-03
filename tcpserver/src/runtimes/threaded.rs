use sink::*;
use sink::fnsink::{ FnSink };

use component::*;
use stdio::*;

use logging::{ Logging, LoggingEvents };

use std::sync::{ Arc, Mutex };
use std::thread;

pub fn main() {
    let logging_sink = Logging::new();

    let arc_eventstore = Arc::new(Mutex::new(Vec::new()));
    let mutex_eventstore = arc_eventstore.clone();
    let eventstore_projection_sink = FnSink::new(move |event: StdinEvents| {
        let mut eventstore = mutex_eventstore.lock().unwrap();
        eventstore.push(event);
        Ok (eventstore.len())
    }).map_result(|_index: Result<usize, ()>| {
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
                writer.send(value).unwrap();
            }
            _ => {}
        }
        Ok ((*string).to_owned())
    }).map_result(|_value: Result<String, ()>| {
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
