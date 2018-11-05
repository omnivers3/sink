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

// macro_rules! as_item { ($i:item) => { $i }; }

// macro_rules! as_expr { ($x:expr) => ($x) }

// macro_rules! as_ident { ($i:ident) => { $i }; }

// macro_rules! tuple_index {
//     ($tuple:expr, $idx:tt) => { as_expr!($tuple.$idx) }
// }

// macro_rules! context_struct_entry {
//     ($($input:ty,>,$result:ty,:,$sink:ident)+) => {
//         {
//             t: $input,
//         }
//     }
// }

macro_rules! _context_struct {
    // Completed macro accumulation
    (@struct $_index:expr, () -> {$(($index:expr, $name:ident | $input:ty | $result:ty | $handler:expr))*}) => {
        struct Context<'a> {
            $($name: &'a Sink<TInput=$input, TResult=$result>),*
        }

        impl<'a> Context<'a> {
            pub fn new($($name: &'a Sink<TInput=$input, TResult=$result>),*) -> Self {
                Context {
                    $($name),*
                }
            }
        }
    };

    (@struct $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr) -> {$($output:tt)*}) => {
        _context_struct!(@struct $index + 1usize, () -> {$($output)* ($index, $name | $input | $result | $handler)})
    };

    (@struct $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr, $($next:tt)*) -> {$($output:tt)*}) => {
        _context_struct!(@struct $index + 1usize, ($($next)*) -> {$($output)* ($index, $name | $input | $result | $handler)})
    };

    (@item $index:expr, $name: ident | $input:ty | $result:ty = $handler:expr) => {{
        impl<'a> Dispatcher<$input, $result> for Context<'a> {
            fn dispatch(&self, input: $input) {
                println!("Dispatcher[{:?} | {:?}]: {:?}", $index, stringify!($name), input);
                self.$name.send(input)
            }
        }
    }};

    // Fall out of macro recursion
    (@disp $_index:expr, ()) => {};

    // Last element in the recursion
    (@disp $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr)) => {
        _context_struct!(@item $index, $name | $input | $result = $handler);
        _context_struct!(@disp $index + 1usize, ())
    };

    // Element with subsequent elements
    (@disp $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr, $($next:tt)*)) => {
        _context_struct!(@item $index, $name | $input | $result = $handler);
        _context_struct!(@disp $index + 1usize, ($($next)*))
    };
}

macro_rules! context_struct {
    ($($input:tt)*) => {

        _context_struct!(@struct 0usize, ($($input)*) -> {});

        _context_struct!(@disp 0usize, ($($input)*));
    };
}

macro_rules! _context {
    (@ctx $_index:expr, () -> {$(($index:expr, $name:ident | $input:ty | $result:ty | $handler:expr))*}) => {
        Context::new($(&$handler),*)
    };

    (@ctx $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr) -> {$($output:tt)*}) => {
        _context!(@ctx $index + 1usize, () -> {$($output)* ($index, $name | $input | $result | $handler)})
    };

    (@ctx $index:expr, ($name: ident: $input:ty | $result:ty = $handler:expr, $($next:tt)*) -> {$($output:tt)*}) => {
        _context!(@ctx $index + 1usize, ($($next)*) -> {$($output)* ($index, $name | $input | $result | $handler)})
    };
}

macro_rules! context {
    ($($input:tt)*) => {
        {
            context_struct!($($input)*);

            _context!(@ctx 0usize, ($($input)*) -> {})
        }
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

    let ctx = context! {
        // logging: LoggingEvents | () = logging_sink,
        // events: Events | () = event_sink,
        // errors: Errors | () = error_sink,
    };

    let system = RefCell::<server::Component>::bind(ctx);
    system.send(server::Commands::Socket(net::Commands::bind_addresses(
        "localhost:8080",
    )));
    // loop {
        system.send(server::Commands::Socket(net::Commands::Accept));
    // }
}
