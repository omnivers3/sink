pub mod component;
pub mod env;
pub mod log;
pub mod net;

extern crate env_logger;
extern crate sink;

use std::io;
// use std::marker::PhantomData;
// use std::net::{ TcpListener, ToSocketAddrs };
use std::net::{AddrParseError, SocketAddr, TcpListener};
use std::num::ParseIntError;

// use env_logger::{init};
// use env::*;
use sink::*;

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

pub trait ITcpServerSink: ISink<TInput = Commands, TResult = Result<Events, Errors>> {}

impl<T> ITcpServerSink for T where T: ISink<TInput = Commands, TResult = Result<Events, Errors>> {}

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

// impl<TServer, TLogging> ISink for Server<TServer, TLogging>
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
