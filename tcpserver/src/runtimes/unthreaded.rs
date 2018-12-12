use sink::*;
use sink::fnsink::{ FnSink };

use component::*;
use stdio::*;

use logging::{ Logging, LoggingEvents };

use std::cell::RefCell;

pub fn main() {
    let logging_sink = Logging::new();

    let eventstore = RefCell::new(Vec::new());
    let eventstore_projection_sink = FnSink::new(|event: StdinEvents| {
        let mut eventstore = eventstore.borrow_mut();
        eventstore.push(event);
        Ok (eventstore.len())
    }).map_result(|_index: Result<usize, ()>| {
    });

    let writer = StdoutLineWriter::new();

    let concatview = RefCell::new(String::default());
    let concatview_projection_sink = FnSink::new(|event: StdinEvents| {
        let mut concatview = concatview.borrow_mut();
        match event {
            StdinEvents::LineReceived (ref line) => {
                let value = format!("{}{}", concatview, line);
                *concatview = value.to_owned();
                writer.send(value).unwrap();
            }
            _ => {}
        }
        Ok (concatview.to_owned())
    }).map_result(|_value: Result<String, ()>| {
    });

    let event_sink = FnSink::new(|event: StdinEvents| {
        eventstore_projection_sink.send(event.clone());
        concatview_projection_sink.send(event);
    });
    
    // let runtime = StdinLineReader::new();
    let runtime = MockLineReader::new(&["foo", "bar", "fiz"]);

    let def = runtime.bind(ctx! {
        logging: LoggingEvents = logging_sink,
        events: StdinEvents = event_sink,
    });
    // def.run();

    // runtime.run(ctx! {
    //     logging: LoggingEvents = logging_sink,
    //     events: StdinEvents = event_sink,
    // });

    println!("EventStore: {:?}", eventstore.borrow());
    println!("ConcatView: {:?}", concatview.borrow());
}