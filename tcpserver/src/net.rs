use std::cell::{ Cell };
use std::io;
use std::marker::PhantomData;
use std::net::{AddrParseError, Incoming, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::num::{ParseIntError};

use sink::*;

use super::{ IConfigProvider };

static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
static HOST_PORT_KEY: &'static str = "HOST_PORT";
static HOST_PORT_DEFAULT: &'static str = "8080";

#[derive(Debug)]
pub enum ServerError {
    AddrParseError (AddrParseError),
    IoError (io::Error),
    PortParseError (ParseIntError),
    // TcpListenerSink (tcp_listener::SinkErrors),
    UnexpectedErr (&'static str),
}

fn parse_ipaddr(ip: String, port: String) -> Result<SocketAddr, ServerError> {
    ip.parse()
        .map_err(ServerError::AddrParseError)
        .and_then(|ip| {
            port.parse()
                .map_err(ServerError::PortParseError)
                .map(|port| {
                    SocketAddr::new(ip, port)
                })
        })
}

pub mod tcp_listener {
    // use std::cell::{ Cell };
    // use std::fmt;
    use std::io;
    use std::marker::PhantomData;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, ToSocketAddrs};
    // use std::net::option;
    use std::vec::IntoIter;

    // use sink::*;

    // pub trait IDataSink<'a>: ISink<TInput=&'a [u8], TResult=()> + Default {}

    // impl<'a, T> IDataSink<'a> for T where T: ISink<TInput=&'a [u8], TResult=()> + Default {}

    #[derive(Debug)]
    pub enum ContextErrors {
        IoError (io::Error),
    }

    pub enum SocketAddrs<'a> {
        FromAddr (SocketAddr),
        FromAddrV4 (SocketAddrV4),
        FromAddrV6 (SocketAddrV6),
        FromIpPort (IpAddr, u16),
        FromIpV4Port (Ipv4Addr, u16),
        FromIpV6Port (Ipv6Addr, u16),
        FromStr (&'a str),
        FromString (String),
        FromTuple (&'a str, u16),
    }

    impl<'a> ToSocketAddrs for SocketAddrs<'a> {
        type Iter = IntoIter<SocketAddr>;

        fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
            match self {
                SocketAddrs::FromAddr (t) => t,
                SocketAddrs::FromAddrV4 (t) => t,
                SocketAddrs::FromAddrV6 (t) => t,
                SocketAddrs::FromIpPort (t) => t,
                SocketAddrs::FromIpV4Port (t) => t,
                SocketAddrs::FromIpV6Port (t) => t,
                SocketAddrs::FromStr (t) => t,
                SocketAddrs::FromString (t) => t,
                SocketAddrs::FromTuple (t) => t,
            }
        }
    }

    #[derive(Debug)]
    pub enum Commands<TIter, TToSocketAddrs>
    where
        TIter: Iterator<Item=SocketAddr>,
        TToSocketAddrs: ToSocketAddrs<Iter=TIter>,
    {
        Accept,
        // BindSocket (TToSocketAddrs),
        BindSocket (Vec<SocketAddr>),
    }

    #[derive(Debug)]
    pub enum Events {
        SocketBound (TcpListener),
        ConnectionEstablished (TcpStream, SocketAddr),
    }

    #[derive(Debug)]
    pub enum Errors {
        AcceptFailed (io::Error),
        BindFailed (io::Error),
        SocketAlreadyBound,
        SocketNotBound,
        StateUpdateFailed,
    }

    #[derive(Debug)]
    pub struct State
    {
        listener: Option<TcpListener>,
    }

    impl Default for State {
        fn default() -> Self {
            State {
                listener: None,
            }
        }
    }

    pub struct Component<TIter, TToSocketAddrs> {
        _iter: PhantomData<TIter>,
        _toSocketAddr: PhantomData<TToSocketAddrs>,
        listener: Option<TcpListener>,
    }

    pub trait IComponent {
        type TState: Default;
        type TCommands;
        type TEvents;
        type TErrors;

        fn init(state: Option<Self::TState>) -> Self;
        fn update(&mut self, event: Self::TEvents);
        fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
    }

    impl<TIter, TToSocketAddrs> IComponent for Component<TIter, TToSocketAddrs>
    where
        TIter: Iterator<Item=SocketAddr>,
        TToSocketAddrs: ToSocketAddrs<Iter=TIter>,
    {
        type TState = State;
        type TCommands = Commands<TIter, TToSocketAddrs>;
        type TEvents = Events;
        type TErrors = Errors;

        fn init(state: Option<State>) -> Self {
            let state = state.unwrap_or_default();
            Component {
                _iter: PhantomData,
                _toSocketAddr: PhantomData,
                listener: state.listener,
            }
        }

        fn update(&mut self, event: Self::TEvents) {
            match event {
                Events::SocketBound (listener) => self.listener = Some(listener),
                Events::ConnectionEstablished (_, addr) => {
                    println!("connection established: {:?}", addr);
                }
            }
        }

        fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
            match command {
                Commands::BindSocket (addr) => {
                    if let Some(_) = self.listener { return Err(Errors::SocketAlreadyBound); }
                    let listener = TcpListener::bind(addr)
                        .map_err(Errors::BindFailed)?;
                    Ok (Events::SocketBound (listener))
                },
                Commands::Accept => {
                    match &self.listener {
                        None => Err(Errors::SocketNotBound),
                        Some (listener) => {
                            println!("Accept");
                            let (stream, addr) = listener.accept()
                                .map_err(Errors::AcceptFailed)?;
                            println!("Got stream");
                            Ok (Events::ConnectionEstablished (stream, addr))
                        }
                    }
                }
            }
        }
    }

    pub struct Runtime<TComponent> {
        component: TComponent,
    }

    impl<TComponent> Runtime<TComponent> {
        pub fn new() -> Self
        where
            TComponent: IComponent,
        {
            let component = TComponent::init(None);
            Runtime {
                component,
            }
        }
    }


    // impl<TIter, TToSocketAddrs> Component<TIter, TToSocketAddrs> {
    //     pub fn new() -> Self {
    //         System {
    //             _iter: PhantomData,
    //             _toSocketAddr: PhantomData,
    //             state: RefCell::new(SystemState::default()),
    //         }
    //     }
    // }

    // impl<TIter, TToSocketAddrs> ISink for Component<TIter, TToSocketAddrs>
    // where
    //     TIter: Iterator<Item=SocketAddr>,
    //     TToSocketAddrs: ToSocketAddrs<Iter=TIter>,
    // {
    //     type TInput = Commands<TIter, TToSocketAddrs>;
    //     type TResult = ();//Result<(), ContextErrors>;

    //     fn send(&self, input: Self::TInput) -> Self::TResult {
    //         match input {
    //             Commands::BindSocket (addr) => {
    //                 println!("BindSocket");
    //                 // TcpListener::bind(addr)
    //                 //     .map_err(ContextErrors::IoError)?;
    //                 // self.dispatch(Events::Bound)
    //             }
    //         }
    //         // *self.listener.borrow_mut() = Some(TcpListener::bind(input).map_err(SinkErrors::IoError)?);
    //         // Ok(())
    //     }
    // }

    // pub trait IAutoSource {
    //     type TOutput;
    // }

    // impl<T> ISource for T
    // where
    //     T: IAutoSource + IDispatcher,
    // {
    //     type TOutput = TOutput;

    //     fn next(&self) -> Self::TOutput {
    //         TOutput::default()
    //     }
    // }

    // impl<'a, TIter, TToSocketAddrs, TOutput> IAutoSource for &'a System<TIter, TToSocketAddrs, TOutput>
    // where
    //     TOutput: fmt::Debug,
    // {
    //     type TOutput = TOutput;
    // }

    // pub trait IDispatcher<TOutput> {
    //     type TResult;

    //     fn dispatch(&self, value: TOutput) -> Self::TResult;
    // }

    // impl<'a, TOutput> IDispatcher<TOutput> for &'a IAutoSource<TOutput=TOutput>
    // where
    //     TOutput: fmt::Debug,
    // {
    //     type TResult = ();

    //     fn dispatch(&self, value: TOutput) -> Self::TResult {
    //         println!("Dispatch: {:?}", value);
    //         ()
    //     }
    // }


    // impl<TIter, TToSocketAddrs, TSink> ISource for System<TIter, TToSocketAddrs> {
    //     fn bind(self, sink: impl ISink<TInput=Events, TResult=TResult>)
    // }

    // impl<TIter, TToSocketAddrs> ISource for System<TIter, TToSocketAddrs> {
    //     type TOutput = Result<(TcpStream, SocketAddr), Errors>;

    //     fn next(&self) -> Self::TOutput {
    //         match *self.listener.borrow() {
    //             Some (ref listener) => listener
    //                 .accept()
    //                 .map_err(Errors::BindFailed),
    //             None => Err (Errors::UnboundListener)
    //         }
    //     }
    // }
}

#[derive(Debug)]
pub struct Server {
    // addr: SocketAddr,
}

#[derive(Debug)]
pub enum ServerResult {
    GracefulShutdown,
}

pub struct SocketContext {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn bind<TSocketAddress: ToSocketAddrs, FSocketBuilder>(self, addr: TSocketAddress, builder: FSocketBuilder) -> Result<Self, ServerError>
    where
        FSocketBuilder: Fn(SocketAddr) -> SocketContext,
    {
        addr
            .to_socket_addrs()
            .map(|_addrs| self)
            .map_err(ServerError::IoError)
        // builder(addr)
    }

    pub fn start(self) -> Result<ServerResult, ServerError> {
        Err (ServerError::UnexpectedErr("failed"))
    }
}


//     pub fn new(mut config: impl IConfigProvider) -> Result<Self, ServerError> {
//         let ip = config.get(HOST_ADDR_KEY).unwrap_or(HOST_ADDR_DEFAULT.to_owned());
//         let port = config.get(HOST_PORT_KEY).unwrap_or(HOST_PORT_DEFAULT.to_owned());
//         parse_ipaddr(ip, port)
//             .map(|addr| {
//                 Server {
//                     addr,
//                 }
//             })
//     }
// }

// impl IService for Server {
//     fn bind()
// }