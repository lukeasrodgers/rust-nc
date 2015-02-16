use getopts::{Options,Matches};
use std::net::{TcpStream,SocketAddr};

pub fn connect(matches: &Matches) {
    let host = matches.free.get(0).unwrap();
    let port = match matches.free.get(1).unwrap().parse::<u16>() {
        Ok(n) => { n }
        Err(_) => {
            panic!("not in possible range".to_string());
        }
    };
    // let connect_addr = SocketAddr::new(host, port);
    match TcpStream::connect(&(host.as_slice(), port)) {
        Ok(stream) => {
            println!("ok");
        },
        Err(f) => {
            // just exit
        }
    }
}
