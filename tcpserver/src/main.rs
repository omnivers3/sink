// #[macro_use]
// extern crate log;
extern crate logging;
extern crate sink;
extern crate tcp_server;

use component::*;
use logging::Logging;
// use env::*;
// use net::*;
use sink::*;
use sink::vecsink::*;
use std::cell::RefCell;
// use std::fmt;
// use std::marker::PhantomData;
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
        // let mut queue: &Vec<AppCommands> = &*self.queue.borrow_mut();
        // queue.pop()
    }
}

fn main() {
    env::EnvConfigProvider::new();

    let system = RefCell::<server::Component>::bind(Logging::new());
    system.send(server::Commands::Socket(net::Commands::bind_addresses(
        "localhost:8080",
    )));
    loop {
        system.send(server::Commands::Socket(net::Commands::Accept));
    }

    // // let context = Context::new();
    // // let context = logging::Logging::new();
    // let context = Logging::new();
    // // let harness = server::Component::to_system(context);
    // let harness = server::Component::bind(context);
    // harness.1.send(server::Commands::Socket(net::Commands::bind_addresses(
    //     "localhost:8080",
    // )));
    // loop {
    //     harness.1.send(server::Commands::Socket(net::Commands::Accept));
    // }

    // let harness = net::Component::to_harness();
    // harness.send(net::Commands::bind_addresses("localhost:8080"));
    // harness.send(net::Commands::bind_addresses("localhost:8080"));
    // harness.send(net::Commands::Accept);
}

// let _logging = Logging {};
// let _cmd = CommandSource::new(vec![AppCommands::Foo, AppCommands::Bar]);

// pub struct App<TCommandSource, TLoggingSink> {
//     _cmd: PhantomData<TCommandSource>,
//     _log: PhantomData<TLoggingSink>,
// }

// impl<TCommandSource, TLoggingSink> App<TCommandSource, TLoggingSink> {
//     pub fn new() -> Self {
//         App {
//             _cmd: PhantomData,
//             _log: PhantomData,
//         }
//     }
// }

// impl<TCommandSource, TLoggingSink> IService for App<TCommandSource, TLoggingSink>
// where
//     TCommandSource: ISource<TOutput=Option<AppCommands>>,
//     TLoggingSink: ISink<TInput=LoggingEvents, TResult=()>,
// {
//     type TContext = (TCommandSource, TLoggingSink);

//     fn bind(self, (cmd, log): Self::TContext) {
//         loop {
//             if let Some(command) = cmd.next() {
//                 log.send(LoggingEvents::Info(format!("Got command {:?}", command)));
//             } else {
//                 break;
//             }
//         }
//     }
// }

// mod string_source {
//     use sink::{ ISink, ISource };

//     // type StringSink = ISink<TInput=String, TResult=()>;

//     impl<T> ISource for T
//     where
//         // T: IContext<T=String>,
//         T: ISink<TInput=String, TResult=()>,
//     {
//         type TOutput = String;

//         fn next(&self) -> Self::TOutput {
//             "asdf".to_owned()
//         }
//     }
// }

// system.run(|tx| {
//     tx.send
// });

// Server::new()
//     .bind("localhost:8080", |addr| {
//         SocketContext {}
//     })
//     .and_then(|server| {
//         server.start()
//     })
//     .map(|result| {
//         println!("Server result: {:?}", result);
//     })
//     .map_err(|err| {
//         println!("Server error: {:?}", err);
//     });
// }
// .map(|req| { // Would be tcp packets
//     println!("Server Request: {:?}", req);
//     req
// })
// .start();

// App::new()
// Should fail to compile due to missing source of T

// let server = Server::run((), (logging.to_owned(), logging));
