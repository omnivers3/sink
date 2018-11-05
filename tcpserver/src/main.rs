// #[macro_use]
// extern crate log;
extern crate logging;
extern crate sink;
extern crate tcp_server;

use component::*;
use logging::{ Logging, LoggingEvents };
// use env::*;
// use net::*;
use sink::*;
use server::{Events, Errors};
// use sink::vecsink::*;
use std::cell::RefCell;
use tcp_server::*;

// static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
// static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
// static HOST_PORT_KEY: &'static str = "HOST_PORT";
// static HOST_PORT_DEFAULT: &'static str = "8080";

#[derive(Debug)]
pub enum AppCommands {
    Foo,
    Bar,
}

pub enum AppErrors {}

pub struct CommandSource {
    queue: RefCell<Vec<AppCommands>>,
}

impl CommandSource {
    pub fn new(queue: Vec<AppCommands>) -> Self {
        let queue = RefCell::new(queue);
        CommandSource { queue }
    }
}

impl Source for CommandSource {
    type TOutput = Option<AppCommands>;

    fn next(&self) -> Self::TOutput {
        self.queue.borrow_mut().pop()
    }
}

macro_rules! as_item { ($i:item) => { $i }; }

macro_rules! as_expr { ($x:expr) => ($x) }

macro_rules! as_ident { ($i:ident) => { $i }; }

macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => { as_expr!($tuple.$idx) }
}

// macro_rules! sinks {
//     ($(($input, $result))*) => (
        
//     )
// }
macro_rules! context_struct_entry {
    ($($input:ty,>,$result:ty,:,$sink:ident)+) => {
        {
            t: $input,
        }
    }
}

// macro_rules! context_struct {
//     // (sink($input:type, $result:type)) => {
//     //     &'a Sink<TInput==$input, TResult=$result>,
//     // }
//     // ($($entry:expr)+) => {

//         // struct Context<'a> {
//         //     context_struct_entry!($($entry)*)
//         //     // context!
//         //     // context_struct!
//         // }
//     // (@item $input:ty > $result:ty : $sink:ident) => {
//     //     {
//     //         &'a Sink<TInput=$input, TResult=()>,
//     //     }
//     // };
//     // // ($($input:ty,>,$result:ty,:,$sink:ident)+) => {
//     // ($($exp:expr)+) => {
//     //     {
//     //         // $(context_struct!($input:ty, >, $result:ty : $sink:ident))+
//     //         context_struct!(@body $(context_struct!(@item $exp))+)
//     //     }
//     // };
//     // ($($input:ty > $result:ty : $sink:expr)+,) => {
//     //     #[derive(Debug)]
//     //     struct Context<'a> {
//     //         // $($body)*
//     //         $(&'a Sink<TInput=$input, TResult=()>,)+
//     //     }
//     // }
//     // input is empty: time to output
//     (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($ty:ty))*}) => {
//         $(#[$attr])* struct $name($($ty),*);
//     };
    
//     // branch off to generate an inner struct
//     (@munch {$lifetime:tt} ($id:ident: struct $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct<$lifetime:tt> $($output:tt)*}) => {
//         context_struct!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
//         context_struct!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)*<$lifetime> ($id: $name)});
//     };
    
//     // // throw on the last field
//     // (@munch ($id:ident: $ty:ty) -> {$($output:tt)*}) => {
//     //     context_struct!(@munch () -> {$($output)* ($id: $ty)});
//     // };
//     // throw on the last field
//     (@munch {$lifetime:tt} ($input:ty > $result:ty : $sink:expr) -> {$($output:tt)*}) => {
//         context_struct!(@munch () -> {$($output)* (&$lifetime Sink<TInput=$input, TResult=$result>)});
//     };
    
//     // // throw on another field (not the last one)
//     // (@munch ($id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
//     //     context_struct!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
//     // };
//     // throw on another field (not the last one)
//     (@munch {$lifetime:tt} ($id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
//         context_struct!(@munch ($($next)*) -> {$($output)* (&$lifetime Sink<TInput=$input, TResult=$result>)});
//     };
    
//     // // entry point (this is where a macro call starts)
//     // ($(#[$attr:meta])* struct $name:ident { $($input:tt)*} ) => {
//     //     context_struct!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
//     //     //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     //     //                     input       output
//     // }
//     // entry point (this is where a macro call starts)
//     (<$lifetime:tt> $($input:ty > $result:ty : $sink:expr)+,) => {
//         context_struct!(@munch {'a} ($($input > $result : $sink)+) -> {struct Context});
//         //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
//         //                     input       output
//     }

//     // ($($input:tt)+) => {
//     //     context_struct!(@inner $($input:tt)+ -> )
//     // }
// }
macro_rules! context_struct {
    // (@struct_item <$lifetime:tt>)
    // (@item <$lifetime:tt> $input:ty | $result:ty) => {
    //     &$lifetime Sink<TInput=$input, TResult=$result>
    // };

    // (@struct <$lifetime:tt> () -> {$(($input:ty | $result:ty))*}) => {
    //     // struct Context<$lifetime>( $(&$lifetime Sink<TInput=$input, TResult=$result>),* );

    //     // #[derive(Debug)]
    //     struct Context<'a>( $(&$lifetime Sink<TInput=$input, TResult=$result>),* );
    // };

    // (@struct <$lifetime:tt> ($input:ty | $result:ty = $t:expr) -> {$($output:tt)*}) => {
    //     context_struct!(@struct <$lifetime> () -> {$($output)* ($input|$result)})
    // };

    // (@struct <$lifetime:tt> ($input:ty | $result:ty = $t:expr, $($next:tt)*) -> {$($output:tt)*}) => {
    //     context_struct!(@struct <$lifetime> ($($next)*) -> {$($output)* ($input | $result)})
    // };
    // (@struct $_index:expr, () -> {$(($input:ty | $result:ty | $handler:expr))*}) => {
        // struct Context<$lifetime>( $(&$lifetime Sink<TInput=$input, TResult=$result>),* );

        // #[derive(Debug)]
        // struct Context<'a>( $(&'a Sink<TInput=$input, TResult=$result>),* );
    (@struct $_index:expr, () -> {$(($index:expr, $input:ty | $result:ty | $handler:expr))*}) => {
        struct Context<'a>( $(&'a Sink<TInput=$input, TResult=$result>),* );
        // struct Context<'a> {
        //     $($index: &'a Sink<TInput=$input, TResult=$result>),*
        // }
        // struct Context<'a> {

        // }

        // $(
        //     impl<'a> Dispatcher<$input, $result> for Context<'a> {
        //         fn dispatch(&self, input: $input) {
        //             self
        //         }
        //     }
        // )*
    };

    // (@dispatcher {})

    (@struct $index:expr, ($input:ty | $result:ty = $handler:expr) -> {$($output:tt)*}) => {
        context_struct!(@struct $index + 1usize, () -> {$($output)* ($index, $input | $result | $handler)})
    };

    (@struct $index:expr, ($input:ty | $result:ty = $handler:expr, $($next:tt)*) -> {$($output:tt)*}) => {
        context_struct!(@struct $index + 1usize, ($($next)*) -> {$($output)* ($index, $input | $result | $handler)})
    };

    // (@item $index:expr, $input:ty | $result:ty | $handler:expr) => {
    //     impl<'a> Dispatcher<$input, $result> for Context<'a> {
    //         fn dispatch(&self, input: $input) {
    //             self.$index.send(input)
    //         }
    //     }
    // };

    (@item $index:expr, $input:ty | $result:ty = $handler:expr) => {{
        impl<'a> Dispatcher<$input, $result> for Context<'a> {
            fn dispatch(&self, input: $input) {
                println!("Dispatcher[{:?}]: {:?}", $index, input);
                // tuple_index!(self, as_ident!($index))//.send(input)
                // as_expr!(self.$index.send(input))
                // self.$index.send(input)
                ()
                // as_expr!(self.$index.send(input))
                // as_expr!(self.as_expr!($index).send(input))
            }
        }
    }};

    (@disp $_index:expr, ()) => {
        // $($dispatcher)*
        // $(
        //     impl<'a> Dispatcher<$input, $result> for Context<'a> {
        //         fn dispatch(&self, input: $input) {
        //             self
        //         }
        //     }
        // )*
    };

    (@disp $index:expr, ($input:ty | $result:ty = $handler:expr)) => {
        context_struct!(@item $index, $input | $result = $handler);
        context_struct!(@disp $index + 1usize, ())
    };

    (@disp $index:expr, ($input:ty | $result:ty = $handler:expr, $($next:tt)*)) => {
        context_struct!(@item $index, $input | $result = $handler);
        context_struct!(@disp $index + 1usize, ($($next)*))
    };
}

macro_rules! context {
    // (struct $name:ident <$lifetime:tt> ($($input:tt)*)) => {
    ($($input:tt)*) => {
        // context_struct!(@struct <$lifetime> ($($input)*) -> {});
        context_struct!(@struct 0usize, ($($input)*) -> {});

        context_struct!(@disp 0usize, ($($input)*));

        // &$lifetime Sink<TInput=LoggingEvents, TResult=()>,
        // &$lifetime Sink<TInput=server::Events, TResult=()>,
        // &$lifetime Sink<TInput=server::Errors, TResult=()>,

        // impl<'a> Dispatcher<LoggingEvents, ()> for Context<'a> {
        //     fn dispatch(&self, input: LoggingEvents) {
        //         self.0.send(input)
        //     }
        // }

        // impl<'a> Dispatcher<server::Events, ()> for Context<'a> {
        //     fn dispatch(&self, input: server::Events) {
        //         self.1.send(input)
        //     }
        // }

        // impl<'a> Dispatcher<server::Errors, ()> for Context<'a> {
        //     fn dispatch(&self, input: server::Errors) {
        //         self.2.send(input)
        //     }
        // }
    };
}

fn main() {
    env::EnvConfigProvider::new();
    let logging_sink = Logging::new();
    let event_sink = sink::fnsink::FnSink::new(|event: server::Events| {
        println!("Event Sink: {:?}", event);
    });
    let error_sink = sink::fnsink::FnSink::new(|error: server::Errors| {
        println!("Error Sink: {:?}", error);
    });

    context! {
        LoggingEvents | () = logging_sink,
        Events | () = event_sink,
        Errors | () = error_sink,
    }

    let ctx = Context(&logging_sink, &event_sink, &error_sink);

    // println!("Temp: {:?}", );
    //     LoggingEvents>():(),//logging_sink,
    //     // Events>():(),//event_sink,
    //     // server::Errors>():(),//error_sink,
    // ];
    // println!("Context: {:?}", Context( &logging_sink ));
    // let ctx = Context( &logging_sink );

    // struct Context<'a>(
    //     &'a Sink<TInput=LoggingEvents, TResult=()>,
    //     &'a Sink<TInput=server::Events, TResult=()>,
    //     &'a Sink<TInput=server::Errors, TResult=()>,
    // );

    // impl<'a> SinkContainer<'a, LoggingEvents, ()> for Context<'a> {
    //     fn sink(&'a self) -> &'a Sink<TInput=LoggingEvents, TResult=()> {
    //         self.0
    //     }
    // }

    // impl<'a> SinkContainer<'a, server::Events, ()> for Context<'a> {
    //     fn sink(&'a self) -> &'a Sink<TInput=server::Events, TResult=()> {
    //         self.1
    //     }
    // }

    // impl<'a> SinkContainer<'a, server::Errors, ()> for Context<'a> {
    //     fn sink(&'a self) -> &'a Sink<TInput=server::Errors, TResult=()> {
    //         self.2
    //     }
    // }

    // impl<'a> Dispatcher<LoggingEvents, ()> for Context<'a> {
    //     fn dispatch(&self, input: LoggingEvents) {
    //         self.0.send(input)
    //     }
    // }

    // impl<'a> Dispatcher<server::Events, ()> for Context<'a> {
    //     fn dispatch(&self, input: server::Events) {
    //         self.1.send(input)
    //     }
    // }

    // impl<'a> Dispatcher<server::Errors, ()> for Context<'a> {
    //     fn dispatch(&self, input: server::Errors) {
    //         self.2.send(input)
    //     }
    // }

    
    let system = RefCell::<server::Component>::bind(ctx);
    system.send(server::Commands::Socket(net::Commands::bind_addresses(
        "localhost:8080",
    )));
    // loop {
        system.send(server::Commands::Socket(net::Commands::Accept));
    // }
}
