use std::io;
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs };
use std::vec;

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
        let addrs: Vec<SocketAddr> = self.to_owned().to_vec();
        Ok(addrs.into_iter())
    }
}