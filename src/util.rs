use getopts::{Options,Matches};
use std::str::FromStr;
use std::net::{SocketAddr,ToSocketAddrs};

// pretty ugly...
pub fn parse_addr(matches: &Matches) -> SocketAddr {
    let host = matches.free.get(0).unwrap();
    let port_string = matches.free.get(1).unwrap();
    let port: u16 = FromStr::from_str(port_string.as_slice()).unwrap();
    return (host.as_slice(), port).to_socket_addrs().unwrap().next().unwrap();
}
