extern crate mio;
use getopts::{Options,Matches};
use std::io::{Read,BufStream,BufRead};
use std::old_io::{LineBufferedWriter,stdio};

use self::mio::*;
use self::mio::net::tcp::*;
use self::mio::buf::{ByteBuf};

const SERVER: Token = Token(1);
const CLIENT: Token = Token(2);

struct ServerHandler {
    conn: Option<NonBlock<TcpStream>>,
    sock: NonBlock<TcpListener>
}

impl ServerHandler {
    fn accept(&mut self, event_loop: &mut EventLoop<ServerHandler>) {
        match self.conn {
            Some(_) => {
                // already have connection, do nothing
            },
            None => {
                let conn = self.sock.accept();
                let stream = conn.unwrap().unwrap();
                self.conn = Some(stream);
                match self.conn {
                    Some(ref c) => {
                        event_loop.register(c, CLIENT);
                    },
                    None => { panic!("no ref??".to_string());
                    }
                }
            }
        }
    }
}

impl Handler for ServerHandler {
    type Timeout = usize;
    type Message = ();

    fn readable(&mut self, event_loop: &mut EventLoop<ServerHandler>, token: Token, _: ReadHint) {
        match token {
            SERVER => {
                // Call `accept` on our `tcp_handler`.
                self.accept(event_loop);
            },
            CLIENT => {
                let mut read_buf = ByteBuf::mut_with_capacity(2048);
                match self.conn {
                    Some(ref mut c) => {
                        match c.read(&mut read_buf) {
                            Ok(n) => {
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
                                // Probably because connection could not be established.
                                event_loop.shutdown();
                                panic!(e.to_string());
                            }
                        }
                    },
                    None => {
                        panic!("can't read".to_string());
                    }
                }
            },
            _ => {
                panic!("unexpected token".to_string());
            }
        }
    }
}

pub fn nc_listen(matches: &Matches) {
    // append port to localhost IP address
    let mut s = String::from_str("127.0.0.1:");
    s.push_str(matches.free[0].as_slice());
    let listener = TcpListener::bind(s.as_slice()).unwrap();
    // block until we get connection
    let mut writer = LineBufferedWriter::new(stdio::stdout());
    match listener.accept() {
        Ok((stream, socket_addr)) => {
            // nc doesn't handle multiple connections, so no need to do `handle_client`
            // in thread.
            handle_client(stream, &mut writer);
        },
        Err(f) => {
            panic!(f.to_string());
        }
    }
}

fn handle_client(stream: TcpStream, writer: &mut Writer) {
    let mut buf_stream = BufStream::new(stream);
    let mut read_buf = [0; 4096];
    loop {
        match(buf_stream.read(&mut read_buf)) {
            Ok(n) => {
                if n > 0 {
                    writer.write_all(&read_buf.slice_to(n)).unwrap();
                }
                else {
                    return;
                }
            }
            Err(f) => {
                panic!(f.to_string());
            }
        }
    }
}
