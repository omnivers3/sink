// use component::*;
use logging::{ Data, Logging, LoggingEvents };
// use sink::{ Dispatcher, Sink };
// use sink::{ Sink, Source };
use sink::{ Source };
// use sink::sink::{ Dispatcher };
// use stdio::*;

use std::fmt;
use std::cell::RefCell;
use std::io;
use std::io::{ BufRead, Stdin, Stdout, Write };

// pub trait RefSource<'a> {
//     type TInput;
//     type TResult;
//     type TOutput;

//     fn sink(&self) -> &'a Sink<TInput=Self::TInput, TResult=Self::TResult>;
//     fn source(&self) -> &'a Source<TOutput=Self::TOutput>;
//     fn duplex(&self) -> (&'a Source<TOutput=Self::TOutput>, &'a Sink<TInput=Self::TInput, TResult=Self::TResult>) {
//         (self.source(), self.sink())
//     }
// }

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Listening,
    LineReceived (String),
    Paused,
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

pub struct InProcRuntime {}

impl InProcRuntime {
    pub fn new() -> Self {
        InProcRuntime {}
    }
}

// pub trait Runtime {
//     fn bind<TContext, TSource, TSink, FMap>(ctx: TContext, source: TSource, sink: TSink, fmap: FMap) -> ()
//     where
//         T;
// }

// impl Runtime for InProcRuntime {
//     fn bind(ctx: TContext, source: TSource, sink: TSink, fmap: FMap) -> () {

//     }
// }

pub enum StdoutCommands {
    WriteLine (String),
}

pub fn parse_stdin(input: StdinEvents) -> Option<StdoutCommands> {
    use self::StdoutCommands::*;
    match input {
        StdinEvents::LineReceived (line) => {
            if line.len() % 10 == 0 {
                None
            } else if line.len() % 2 == 0 {
                Some (WriteLine(line))
            } else {
                Some (WriteLine(format!("foo: {}", line)))
            }
        }
        _ => None
    }
}

pub struct Alternate {
    value: RefCell<bool>,
}

impl Alternate {
    pub fn new() -> Self {
        Alternate {
            value: RefCell::new(false),
        }
    }
}

impl Source for Alternate {
    type TOutput = Option<()>;

    fn next(&self) -> Self::TOutput {
        let mut value = self.value.borrow_mut();
        *value = !*value;
        if *value {
            Some(())
        } else {
            None
        }
    }
}

pub fn main() {
    // Opaque bridge sink into local stdout logging runtime
    let logging_sink = Logging::new();

    // logging_sink.send(trace!("Starting Domain Definition"));

    let source = Alternate::new();

    let logging_ctx = ctx! {
        source: Option<()> = source,
        logging: LoggingEvents = logging_sink,
    };

    let t = logging_ctx.next();
    println!("Next: {:?}", t);

    let source = StdinLineReader::new();
    // let source = MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]);
    // let sink = StdoutLineWriter::new();

    let in_proc = InProcRuntime::new();

    // Runtime::new()
    //     .bind(
    //         ctx! {
    //             logging: LoggingEvents = logging_sink,//Logging::new(),
    //         },
    //         StdinLineReader::new(),
    //         // MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]),
    //         StdoutLineWriter::new(),
    //         |sink| sink
    //             .reduce() // skip broken entries
    //             .map(parse_stdin)
    //             .map_result(|_| ())
    //     )
    //     .run();

    // InProcRuntime::new()
    //     .bind(
    //         ctx! {
    //             logging: LoggingEvents = logging_sink,//Logging::new(),
    //         },
    //         StdinLineReader::new(),
    //         // MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]),
    //         StdoutLineWriter::new(),
    //         | sink | sink
    //             .reduce() // skip broken entries
    //             .map(parse_stdin)
    //             .map_result(|_| ())
    //     )
    //     .run();

    // let stdout = in_proc
    //     .bind(StdoutLineWriter::new(), logging_ctx);

    
    // stdin.run(| stdin: Sink<TInput | {
    //     stdout.run(| stdout | {
    //         stdout
    //             .reduce() // skip broken entries
    //             .map(parse_stdin)
    //             .map_result(|_| ())
    //             .map(stdin.send)
    //     })
    // })
    // InProcRuntime::new()
    //     .bind(source, ctx! {
    //         logging: LoggingEvents = Logging::new(),
    //     })
    //     .run(| stdin | {
    //         InProcRuntime::new()
    //             .bind(StdoutLineWriter::new(), ctx! {
    //                 logging: LoggingEvents = Logging::new(),
    //             })
    //             .run(| stdout | {
    //                 stdout
    //                     .reduce() // skip broken entries
    //                     .map(parse_stdin)
    //                     .map_result(|_| ())
    //             })
    //     });
}



// pub struct LocalThreadRuntime {}

// impl LocalThreadRuntime {
//     pub fn new() -> Self {
//         LocalThreadRuntime {}
//     }
// }

// pub struct RemoteHttpRuntime {}

// impl RemoteHttpRuntime {
//     pub fn new() -> Self {
//         RemoteHttpRuntime {}
//     }
// }

// pub struct RemoteGrpcRuntime {}

// impl RemoteGrpcRuntime {
//     pub fn new() -> Self {
//         RemoteGrpcRuntime {}
//     }
// }


// pub fn main2() {
//     // Opaque bridge sink into local stdout logging runtime
//     let logging_sink = Logging::new();

//     logging_sink.send(trace!("Starting Domain Definition"));

//     let local_thread_console = LocalThreadRuntime::new();
//     let remote_http_like_counter = RemoteHttpRuntime::new();
//     let remote_grpc_dislike_counter = RemoteGrpcRuntime::new();

//     // // Build refs to both the source and sink edges of the std console
//     // let (stdin_events_source, stdout_line_sink) = local_thread_console.duplex();

//     // // Build refs to both the source and sink edges of the like service over http
//     // let (like_source, like_sink) = remote_http_like_counter.duplex();

//     // // Build refs to both the source and sink edges of the dislike service over grpc
//     // let (dislike_source, dislike_sink) = remote_grpc_dislike_counter.duplex();

//     let in_proc_console = InProcRuntime::new();

//     // like_counter::Manifest
// }


// mod bounded_contexts {
//     use sink::{ Sink };

//     pub mod like_counter {
//         // use aggregates::counter::*;

//         // pub struct Manifest {}

//         // impl BoundedContextDef for Manifest {

//         // }
//     }

//     pub mod dislike_counter {
//         // use aggregates::counter::*;

//     }

//     // pub struct BoundedContext<'a> {
//     //     display_name: &'a str,

//     // }

//     // pub struct Manifest<TInput> {
//     //     input: TInput,
//     // }

//     pub trait SinkProvider<'a, TInput, TResult> {
//         fn sink(&self) -> &'a Sink<TInput=TInput, TResult=TResult>;
//     }

//     pub trait Manifest {
//         type TInput;
//         type TOutput;
//     }

//     pub mod server {
//         // use super::{ Manifest };
//         // use sink::sink::{ Dispatcher };

//         // pub enum Commands {
//         //     AddLike,
//         //     AddDislike,
//         // }

//         // pub struct Dependencies {}

//         // pub struct Context {}

//         // impl Manifest for Context {
//         //     type TInput = Commands;
//         //     type TOutput = ();
//         // }

//         // impl<TContext> BoundedContext for Manifest
//         // where
//         //     TContext: Dispatcher<Events>,
//         // impl<TContext> BoundedContext for Manifest
//         // where
//         //     TContext: Dispatcher<Events>,
//         //     // TContext: Dispatcher<Events>,
//         // {
//         //     type TContext = TContext;
//         // }
//     }

// }

// // use self::bounded_contexts::*;
