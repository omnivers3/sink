use sink::*;
use component::*;
use stdio::*;
use super::{ parse_stdin, ElmModelRefCell };
use super::concatview;

use logging::{ Logging, LoggingEvents };

pub fn main() {
    // let logging_sink = Logging::new();

    // let model = RefCell::<Model>::default();

    // let event_sink = FnSink::new(|event: StdinEvents| {
    //     model.borrow_mut().update(event);
    // });
    // let concat_event_sink = FnSink::new(|event: concatview::Events| {
    //     println!("Concat View: {:?}", event);
    // });
    // let event_sink = concat_event_sink
    //     .reduce()
    //     .map(parse_stdin)
    //     .map_result(|_| ());
    // let event_sink = FnSink::new(|event: StdinEvents| {
    //     eventstore_projection_sink.send(event.clone());
    //     concatview_projection_sink.send(event);
    // });
    
    let ref_model = ElmModelRefCell::<concatview::Model>::default();
    // let source = StdinLineReader::new();
    let source = MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]);

    source.run(ctx! {
        logging: LoggingEvents = Logging::new(),//logging_sink,
        events: StdinEvents = StdoutLineWriter::new()
            // .map(|event: concatview::Events| {
            //     match event {
            //         concatview::Events::Foo (value) => format!("foo: {:?}", value),
            //         concatview::Events::Bar (value) => format!("bar: {:?}", value),
            //     }
            // })
            .map(|model: concatview::Model| {
                format!("view: {:?}", model.value())
            })
            .map(|event: concatview::Events| {
                println!("to model: {:?}", event);
                (*ref_model.model.borrow()).clone()
            })
            .map(|event: concatview::Events| {
                println!("update: {:?}", event);
                ref_model.send(event.clone());
                event
                // model.model.value.into_inner()
                // *ref_model.model.clone()
                // (*ref_model.model.as_ptr()).clone()
                
                // model.model.borrow().to_owned()
                // event
            })
            .reduce() // skip broken entries
            .map(parse_stdin)
            .map_result(|_| ())
    });

    // Runtime::new((source, sink))
    //     .run(|sink| {
    //         sink
    //             .reduce()
    //             .option_map(|event: concatview::Events| {
    //                 match event {
    //                     Foo (value) => value
    //                 }
    //             })
    //             .option_map(|event: concatview::Events| {
    //                 println!("option map: {:?}", event);
    //                 event
    //             })
    //             .map(parse_stdin)
    //             .map_result(|_| ())
    //     })
}
