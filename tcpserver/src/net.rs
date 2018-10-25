//! [TcpListener](https://doc.rust-lang.org/std/net/struct.TcpListener.html)

use socket_addrs::*;
use component::{ IAggregate, IInitialized };
use std::io;
use std::net::{ SocketAddr, TcpListener, TcpStream, ToSocketAddrs };

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
    pub fn bind_addresses<I: Iterator<Item = SocketAddr>, T: ToSocketAddrs<Iter=I>>(addrs: T) -> Self {
        addrs.into()
    }
}

impl<I: Iterator<Item = SocketAddr>, T: ToSocketAddrs<Iter=I>> From<T> for Commands {
    fn from(src: T) -> Self {
        Commands::BindAddresses(
            SocketAddrs::from(src)
                .unwrap_or_else(|_| SocketAddrs::List(Vec::new()))
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
    SocketError (String),
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
}

impl Default for Component {
    fn default() -> Self {
        Component {
            blocking: true,
            listener: None,
            ttl: None,
        }
    }
}

impl IInitialized for Component {
    type TState = State;

    fn apply(&mut self, state: State) {
        let ttl = state.listener.ttl().ok();
        self.blocking = state.blocking;
        self.listener = Some(state.listener);
        self.ttl = ttl;
    }
}

impl IAggregate for Component {
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn update(&mut self, event: Self::TEvents) {
        match event {
            Events::SocketBound(listener) => 
                self.listener = Some(listener),
            Events::ConnectionEstablished(socket, addr) => {
                // self.send(Debug(format!("Connection was established: {:?} - {:?}", socket, addr)));
                debug!("Connection was established: {:?} - {:?}", socket, addr);
            }
            Events::ListenerCloned(_) => {
                // self.send(Debug(format!("Listener cloned")));
                debug!("Listener cloned");
            }
            Events::NonBlockingSet(value) => {
                // self.send(Debug(format!("NonBlocking set to {:?}", value)));
                 debug!("NonBlocking set to {:?}", value);
                // self.handle = value;
                self.blocking = value;
            }
            Events::TtlSet(ttl) => {
                self.ttl = Some(ttl);
            }
        }
    }

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
        match command {
            Commands::Accept =>
                match &self.listener {
                    None => Err(Errors::SocketNotBound),
                    Some(listener) => {
                        let (stream, addr) = listener.accept().map_err(Errors::AcceptFailed)?;
                        Ok(Events::ConnectionEstablished(stream, addr))
                    }
                }
            Commands::BindAddresses(addr) =>
                match &self.listener {
                    Some(_) => Err(Errors::SocketAlreadyBound),
                    None => TcpListener::bind(addr)
                        .map_err(Errors::BindFailed)
                        .map(Events::SocketBound)
                }
            Commands::CloneListener =>
                match &self.listener {
                    None => Err(Errors::SocketNotBound),
                    Some(listener) => {
                        listener
                            .try_clone()
                            .map_err(Errors::CloneFailed)
                            .map(Events::ListenerCloned)
                    }
                }
            Commands::SetNonBlocking(value) =>
                match &self.listener {
                    None => Err(Errors::SocketNotBound),
                    Some(listener) => {
                        listener
                            .set_nonblocking(value)
                            .map_err(Errors::SetNonBlockingFailed)
                            .map(|_| Events::NonBlockingSet(value))
                    }
                }
            Commands::SetTtl(ttl) =>
                match &self.listener {
                    None => Err(Errors::SocketNotBound),
                    Some(listener) => {
                        listener
                            .set_ttl(ttl)
                            .map_err(Errors::SetTtlFailed)
                            .map(|_| Events::TtlSet(ttl))
                    }
            }
        }
    }
}
