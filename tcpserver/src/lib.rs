#[macro_use]
extern crate log;
extern crate env_logger;

// #[macro_use]
extern crate logging;
extern crate sink;

pub mod component;
// pub mod logging;
pub mod env;
pub mod http;
pub mod net;
pub mod server;
pub mod socket_addrs;

use sink::*;
use std::io;
use std::net::{AddrParseError, SocketAddr, TcpListener};
use std::num::ParseIntError;

pub trait IConfigProvider {
    fn get(&mut self, key: &'static str) -> Option<String>;
}

// use Commands::*;
#[derive(Debug)]
pub enum Commands {
    Bind(SocketAddr),
}

// use Events::*;
#[derive(Debug)]
pub enum Events {
    Listening(TcpListener, SocketAddr),
    // Logging (LoggingEvents),
}

#[derive(Debug)]
pub enum SocketAddrParseError {
    AddrParseError(AddrParseError),
    PortParseError(ParseIntError),
    HostAddressInUse(SocketAddr),
}

// use Errors::*;
#[derive(Debug)]
pub enum Errors {
    IoError(io::Error),
    ParseError(SocketAddrParseError),
}

pub trait TcpServerSink: Sink<TInput = Commands, TResult = Result<Events, Errors>> {}

impl<T> TcpServerSink for T where T: Sink<TInput = Commands, TResult = Result<Events, Errors>> {}

// pub struct Server<TServer, TLogging> {
//     // ctx: (),
//     _server: PhantomData<TServer>,
//     _logging: PhantomData<TLogging>,
// }

// impl<TServer, TLogging> Server<TServer, TLogging> {
//     pub fn new() -> Self {
//         Server {
//             // ctx,
//             _server: PhantomData,
//             _logging: PhantomData,
//         }
//     }
// }

// impl<TServer, TLogging> IService for Server<TServer, TLogging>
// where
//     TServer: ILoggingSink,
//     TLogging: ILoggingSink,
// {
//     type TInput = ();
//     type TOutput = (TServer, TLogging);
//     type THandle = Self;

//     fn run(_ctx: Self::TInput, (_server, logging): Self::TOutput) -> Self::THandle {
//         logging.send(LoggingEvents::Info("sfutt".to_owned()));
//         Server::new()
//     }
// }

// impl<TServer, TLogging> Sink for Server<TServer, TLogging>
// where
//     TServer: ILoggingSink,
//     TLogging: ILoggingSink,
// {
//     type TInput = Commands;
//     type TResult = Result<Events, Errors>;

//     fn send(&self, input: Commands) -> Result<Events, Errors> {
//         match input {
//             Bind (addr) =>
//                 TcpListener::bind(addr)
//                     .map_err(IoError)
//                     .and_then(|listener|
//                         listener
//                             .local_addr()
//                             .map(|addr| Listening (listener, addr))
//                             .map_err(IoError)
//                     )
//         }
//     }
// }
