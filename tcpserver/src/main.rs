// #[macro_use]
// extern crate log;
extern crate logging;
#[macro_use]
extern crate sink;
extern crate tcp_server;

use component::*;
use logging::{ Logging, LoggingEvents };
// use env::*;
// use net::*;
use sink::*;
use sink::fnsink::{ FnSink };

use sink::vecsink::*;
use server::{Events, Errors};

use std::cell::RefCell;
use tcp_server::*;

// static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
// static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
// static HOST_PORT_KEY: &'static str = "HOST_PORT";
// static HOST_PORT_DEFAULT: &'static str = "8080";

fn main() {
    env::EnvConfigProvider::new();

    let logging_sink = Logging::new();
    // let event_sink = VecSink::new().map_result(|_| ());
    // let error_sink = VecSink::new().map_result(|_| ());
    let event_vec = VecSink::new();
    
    let event_sink = &event_vec.map(|event| format!("{:?}", event)).map_result(|_| ());
    // let event_sink = FnSink::new(|event: server::Events| {
    //     println!("Event Sink: {:?}", event);
    // });
    let error_sink = FnSink::new(|error: server::Errors| {
        println!("Error Sink: {:?}", error);
    });

    let ctx = context! {
        logging: LoggingEvents | () = logging_sink,
        events: Events | () = event_sink,
        errors: Errors | () = error_sink,
    };

    let system = RefCell::<server::Component>::bind(ctx);

    system.send(server::Commands::Socket(net::Commands::bind_addresses(
        "localhost:8080",
    )));
    // loop {
        system.send(server::Commands::Socket(net::Commands::Accept));
    // }

    // println!("EVENT SINK: {:?}", event_vec.data());
}



// #[derive(Debug)]
// pub enum AppCommands {
//     Foo,
//     Bar,
// }

// pub enum AppErrors {}

// pub struct CommandSource {
//     queue: RefCell<Vec<AppCommands>>,
// }

// impl CommandSource {
//     pub fn new(queue: Vec<AppCommands>) -> Self {
//         let queue = RefCell::new(queue);
//         CommandSource { queue }
//     }
// }

// impl Source for CommandSource {
//     type TOutput = Option<AppCommands>;

//     fn next(&self) -> Self::TOutput {
//         self.queue.borrow_mut().pop()
//     }
// }