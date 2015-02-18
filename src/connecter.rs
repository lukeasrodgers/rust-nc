use getopts::{Options,Matches};
use std::net::{TcpStream,SocketAddr};
use std::io::{BufStream,Write};
use std::old_io;

pub fn connect(matches: &Matches) {
    let host = matches.free.get(0).unwrap();
    let port = match matches.free.get(1).unwrap().parse::<u16>() {
        Ok(n) => { n }
        Err(_) => {
            panic!("not in possible range".to_string());
        }
    };
    match TcpStream::connect(&(host.as_slice(), port)) {
        Ok(stream) => {
            readwrite(&stream);
        },
        Err(f) => {
            // just exit
        }
    }
}

fn readwrite(stream: &TcpStream) {
    let mut buf_stream = BufStream::new(stream);
    let mut stdin_reader = old_io::stdin();
    let mut read_buf = [0, 4096];
    loop {
        match stream.peer_addr() {
            Ok(addr) => { /* */ },
            Err(f) => {
                // Hack for when other end closes socket, will exit.
                return;
            }
        }
        // Have to block here, so we can't responsively terminate if server closes socket.
        match stdin_reader.read(&mut read_buf) {
            Ok(n) => {
                buf_stream.write_all(&read_buf.slice_to(n)).unwrap();
                buf_stream.flush(); // overkill, probably
            },
            Err(f) => {
                panic!(f.to_string());
            }
        }
    }
}
