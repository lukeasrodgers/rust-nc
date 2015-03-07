extern crate mio;
use getopts::{Options,Matches};
use std::net::{TcpStream,SocketAddr};
use std::io::{BufStream,Write};
use std::old_io;

use self::mio::*;
use self::mio::net::tcp::{TcpSocket, TcpListener};
use self::mio::buf::{ByteBuf};

const CLIENT: Token = Token(1);

struct ClientHandler {
    sock: TcpSocket
}

impl Handler for ClientHandler {
    type Timeout = usize;
    type Message = ();

    fn readable(&mut self, event_loop: &mut EventLoop<ClientHandler>, token: Token, _: ReadHint) {
        match token {
            CLIENT => {
                let mut read_buf = ByteBuf::mut_with_capacity(2048);
                match self.sock.read(&mut read_buf) {
                    Ok(None) => {
                        panic!("Read operation would block, bailing cuz this shouldn't happen.");
                    }
                    Ok(Some(r)) => {
                        // `_` would be the number of bytes read.
                        // `flip` will return a `ByteBuf` on which we can call
                        // `read_slice` to get the data available to be read.
                        // See http://carllerche.github.io/bytes/bytes/struct.ByteBuf.html
                        let mut buf = read_buf.flip();
                        let mut sl = [0; 2048];
                        buf.read_slice(&mut sl);
                        // Assuming what was written was encoded as UTF8, print what
                        // was read to STDOUT.
                        print!("{}", String::from_utf8(sl.to_vec()).unwrap());
                    }
                    Err(e) => {
                        if e.is_eof() {
                            println!("Client closed connection, shutting down.");
                            event_loop.shutdown();
                        }
                        else {
                            panic!(e);
                        }
                    }
                }
            },
            _ => {
                panic!("unepxected token!".to_string());
            }
        }
    }
}

pub fn connect(matches: &Matches) {
    let host = matches.free.get(0).unwrap();
    let port = matches.free.get(1).unwrap();
    let mut addr = String::new();
    addr.push_str(host.as_slice());
    addr.push_str(":");
    addr.push_str(port.as_slice());
    let sock = TcpSocket::v4().unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    match sock.connect(&addr) {
        Ok(stream) => {
            let mut client = ClientHandler { sock: stream };
            event_loop.register(&client.sock, CLIENT).unwrap();
            let _ = event_loop.run(&mut client);
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
        // Have to block here, so we can't immediately terminate if server closes socket.
        match stdin_reader.read(&mut read_buf) {
            Ok(n) => {
                buf_stream.write_all(&read_buf.slice_to(n)).unwrap();
                match buf_stream.flush() {
                    Ok(_) => { /* */ },
                    Err(f) => {
                        // other end closed socket
                        return;
                    }
                }
            },
            Err(f) => {
                panic!(f.to_string());
            }
        }
    }
}
