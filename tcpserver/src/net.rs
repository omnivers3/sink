use std::io;
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, ToSocketAddrs };
use std::vec;
use component::IComponent;

#[derive(Debug)]
pub enum ContextErrors {
    IoError(io::Error),
}

#[derive(Clone, Debug)]
pub enum SocketAddrs {
    FromAddr(SocketAddr),
    FromAddrV4(SocketAddrV4),
    FromAddrV6(SocketAddrV6),
    FromIpPort((IpAddr, u16)),
    FromIpV4Port((Ipv4Addr, u16)),
    FromIpV6Port((Ipv6Addr, u16)),
    List(Vec<SocketAddrs>),
}

impl SocketAddrs {
    pub fn from<I: Iterator<Item = SocketAddr>, T: ToSocketAddrs<Iter = I>>(
        src: T,
    ) -> Result<Self, io::Error> {
        src.to_socket_addrs().map(|mut addrs| {
            match (
                addrs.next().map(SocketAddrs::FromAddr),
                addrs.next().map(SocketAddrs::FromAddr)
            ) {
                (None, _) => SocketAddrs::List(Vec::new()),
                (Some (first), None) => first,
                (Some (first), Some (second)) => {
                    let mut result = vec![first, second];
                    for addr in addrs {
                        result.push(SocketAddrs::FromAddr(addr));
                    }
                    SocketAddrs::List(result)
                }
            }
            // if let Some(first) = addrs.next() {
            //     let first = SocketAddrs::FromAddr(first);
            //     if let Some(second) = addrs.next() {
            //         let mut result = vec![first, SocketAddrs::FromAddr(second)];
            //         for addr in addrs {
            //             result.push(SocketAddrs::FromAddr(addr));
            //         }
            //         SocketAddrs::List(result)
            //     } else {
            //         first
            //     }
            // } else {
            //     SocketAddrs::List(Vec::new())
            // }
        })
    }

    fn to_vec(self) -> Vec<SocketAddr> {
        let mut result = Vec::new();
        match self {
            SocketAddrs::FromAddr(t) => result.push(t),
            SocketAddrs::FromAddrV4(t) => result.push(t.into()),
            SocketAddrs::FromAddrV6(t) => result.push(t.into()),
            SocketAddrs::FromIpPort(t) => result.push(t.into()),
            SocketAddrs::FromIpV4Port(t) => result.push(t.into()),
            SocketAddrs::FromIpV6Port(t) => result.push(t.into()),
            SocketAddrs::List(l) => {
                for i in l {
                    if let Ok(addrs) = i.to_socket_addrs() {
                        for j in addrs {
                            result.push(j);
                        }
                    }
                }
            }
        }
        result
    }
}

impl From<SocketAddrs> for Vec<SocketAddr> {
    fn from(addrs: SocketAddrs) -> Vec<SocketAddr> {
        addrs.to_vec()
    }
}

impl<'a> From<&'a SocketAddrs> for Vec<SocketAddr> {
    fn from(addrs: &SocketAddrs) -> Vec<SocketAddr> {
        addrs.into()
    }
}

impl ToSocketAddrs for SocketAddrs {
    type Iter = vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        println!("ToSocketAddrs: {:?}", self);
        let addrs: Vec<SocketAddr> = self.to_owned().to_vec();
        Ok(addrs.into_iter())
    }
}

#[derive(Debug)]
pub enum Commands {
    Accept,
    BindAddresses(SocketAddrs),
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
    SocketBound(TcpListener),
    ConnectionEstablished(TcpStream, SocketAddr),
}

#[derive(Debug)]
pub enum Errors {
    AcceptFailed(io::Error),
    BindFailed(io::Error),
    SocketAlreadyBound,
    SocketNotBound,
    StateUpdateFailed,
}

#[derive(Debug)]
pub struct Component {
    listener: Option<TcpListener>,
}

impl Component {
    pub fn new(listener: TcpListener) -> Self {
        Component {
            listener: Some(listener),
        }
    }
}

impl Default for Component {
    fn default() -> Self {
        Component {
            listener: None,
        }
    }
}

impl IComponent for Component {
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn update(&mut self, event: Self::TEvents) {
        match event {
            Events::SocketBound(listener) => self.listener = Some(listener),
            Events::ConnectionEstablished(_, addr) => {
                println!("connection established: {:?}", addr);
            }
        }
    }

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
        match command {
            Commands::BindAddresses(addr) => {
                if let Some(_) = self.listener {
                    return Err(Errors::SocketAlreadyBound);
                }
                let listener = TcpListener::bind(addr).map_err(Errors::BindFailed)?;
                Ok(Events::SocketBound(listener))
            }
            Commands::Accept => match &self.listener {
                None => Err(Errors::SocketNotBound),
                Some(listener) => {
                    println!("Accept");
                    let (stream, addr) = listener.accept().map_err(Errors::AcceptFailed)?;
                    println!("Got stream: {:?} - {:?}", stream, addr);
                    Ok(Events::ConnectionEstablished(stream, addr))
                }
            }
        }
    }
}
