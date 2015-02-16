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
            write_to_stream(&stream);
            println!("ok");
        },
        Err(f) => {
            // just exit
        }
    }
}

fn write_to_stream(stream: &TcpStream) {
    let mut buf_stream = BufStream::new(stream);
    let mut stdin = old_io::stdin();
    for line in stdin.lock().lines() {
        buf_stream.write_all(line.unwrap().as_bytes()).unwrap();
        buf_stream.flush();
    }
}
