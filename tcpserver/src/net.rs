//! [TcpListener](https://doc.rust-lang.org/std/net/struct.TcpListener.html)

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::str::from_utf8;
// use std::thread;

use component::AggregateRoot;
use sink::Initializable;
use socket_addrs::*;

pub type Ttl = u32;

#[derive(Debug)]
pub enum Commands {
    /// Blocks the harness thread until a connection arrives
    Accept,
    /// Binds the listener to the provider address if it is not already bound
    BindAddresses(SocketAddrs),
    /// Attempts to create a clone of the listener
    CloneListener,
    /// Changes the blocking model for the listener
    SetNonBlocking(bool),
    /// This value sets the time-to-live field that is used in every packet sent from this socket
    SetTtl(Ttl),
}

impl Commands {
    pub fn bind_addresses<I: Iterator<Item = SocketAddr>, T: ToSocketAddrs<Iter = I>>(
        addrs: T,
    ) -> Self {
        addrs.into()
    }
}

impl<I: Iterator<Item = SocketAddr>, T: ToSocketAddrs<Iter = I>> From<T> for Commands {
    fn from(src: T) -> Self {
        Commands::BindAddresses(
            SocketAddrs::from(src).unwrap_or_else(|_| SocketAddrs::List(Vec::new())),
        )
    }
}

#[derive(Debug)]
pub enum Events {
    /// A client connection was established to the internal listener
    ConnectionEstablished(TcpStream, SocketAddr),
    /// Internal listener handle was cloned
    ListenerCloned(TcpListener),
    /// The blocking model for the listener has been modified
    NonBlockingSet(bool),
    /// Socket was opened with the provided listener handle
    SocketBound(TcpListener),
    /// Time-to-live field for every packet was set to the specified value
    TtlSet(Ttl),
}

#[derive(Debug)]
pub enum Errors {
    AcceptFailed(io::Error),
    BindFailed(io::Error),
    CloneFailed(io::Error),
    SetNonBlockingFailed(io::Error),
    SetTtlFailed(io::Error),
    SocketAlreadyBound,
    /// Results from a non-empty call to take_error on the listener between calls
    SocketError(String),
    SocketNotBound,
    StateUpdateFailed,
    TtlFailed(io::Error),
}

#[derive(Debug)]
pub struct State {
    blocking: bool,
    listener: TcpListener,
}

#[derive(Debug)]
pub struct Component {
    blocking: bool,
    listener: Option<TcpListener>,
    ttl: Option<u32>,
    buffer: Vec<u8>,
}

impl Default for Component {
    fn default() -> Self {
        Component {
            blocking: true,
            listener: None,
            ttl: None,
            buffer: Vec::with_capacity(2048),
        }
    }
}

impl Initializable for Component {
    type TState = State;

    fn apply(&mut self, state: State) {
        let ttl = state.listener.ttl().ok();
        self.blocking = state.blocking;
        self.listener = Some(state.listener);
        self.ttl = ttl;
    }
}

impl AggregateRoot for Component {
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn update(&mut self, event: Self::TEvents) {
        match event {
            Events::SocketBound(listener) => self.listener = Some(listener),
            Events::ConnectionEstablished(mut socket, _addr) => {
                // thread::spawn(move || {
                {
                    let mut reader = BufReader::new(&socket);
                    // let mut buff = Vec::new();
                    let mut read_bytes = reader.read_until(b'\n', &mut self.buffer).unwrap();
                    while read_bytes > 0 {
                        read_bytes = reader.read_until(b'\n', &mut self.buffer).unwrap();
                        if read_bytes == 2 && &self.buffer[(self.buffer.len() - 2)..] == b"\r\n" {
                            break;
                        }
                    }
                    warn!("\n{:?}\n", from_utf8(self.buffer.as_slice()).unwrap());
                }
                self.buffer = Vec::with_capacity(2048);
                // return buff;
                // let mut buffer = String::default();
                // let len = socket.read_to_string(&mut buffer);
                // warn!("Got data [{:?}]: {:?}", len, buffer);
                // let response = b"HTTP/1.1 202 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
                let response = b"HTTP/1.1 202 OK\r\nContent-Length=20\r\nETag=47feba42\r\n";
                let result = socket.write(response).expect("Write failed");
                warn!("Result: {:?}", result);
                // });
                // warn!("Connection was established: {:?} - {:?}", socket, addr);
            }
            Events::ListenerCloned(_listener) => {
                // warn!("Listener cloned");
            }
            Events::NonBlockingSet(value) => {
                self.blocking = value;
            }
            Events::TtlSet(ttl) => {
                self.ttl = Some(ttl);
            }
        }
    }

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
        match command {
            Commands::Accept => match &self.listener {
                None => Err(Errors::SocketNotBound),
                Some(listener) => {
                    let (stream, addr) = listener.accept().map_err(Errors::AcceptFailed)?;
                    Ok(Events::ConnectionEstablished(stream, addr))
                }
            },
            Commands::BindAddresses(addr) => match &self.listener {
                Some(_) => Err(Errors::SocketAlreadyBound),
                None => TcpListener::bind(addr)
                    .map_err(Errors::BindFailed)
                    .map(Events::SocketBound),
            },
            Commands::CloneListener => match &self.listener {
                None => Err(Errors::SocketNotBound),
                Some(listener) => listener
                    .try_clone()
                    .map_err(Errors::CloneFailed)
                    .map(Events::ListenerCloned),
            },
            Commands::SetNonBlocking(value) => match &self.listener {
                None => Err(Errors::SocketNotBound),
                Some(listener) => listener
                    .set_nonblocking(value)
                    .map_err(Errors::SetNonBlockingFailed)
                    .map(|_| Events::NonBlockingSet(value)),
            },
            Commands::SetTtl(ttl) => match &self.listener {
                None => Err(Errors::SocketNotBound),
                Some(listener) => listener
                    .set_ttl(ttl)
                    .map_err(Errors::SetTtlFailed)
                    .map(|_| Events::TtlSet(ttl)),
            },
        }
    }
}
