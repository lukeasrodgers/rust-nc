extern crate mio;
use getopts::{Options,Matches};
use std::io::{Read,BufStream,BufRead};
use std::old_io;

use self::mio::*;
use self::mio::net::tcp::*;
use self::mio::buf::{ByteBuf};

use std::net::{SocketAddr,ToSocketAddrs};
use std::str::FromStr;

use std::thread;

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
    type Message = String;

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

    fn notify(&mut self, event_loop: &mut EventLoop<ServerHandler>, msg: String) {
        match self.conn {
            Some(ref mut c) => {
                c.write_slice(msg.as_bytes()).unwrap();
            },
            None => {
                panic!("??".to_string());
            }
        }
    }
}

pub fn nc_listen(matches: &Matches) {
    // append port to localhost IP address
    let mut s = String::from_str("127.0.0.1:");
    s.push_str(matches.free[0].as_slice());
    let addr: SocketAddr = FromStr::from_str(s.as_slice()).unwrap();

    println!("addr: {}", addr);
    let listener = v4().unwrap();
    listener.bind(&addr).unwrap();
    let listener = listener.listen(1).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&listener, SERVER).unwrap();

    let mut server_handler = ServerHandler {
        conn: None,
        sock: listener
    };

    let sender = event_loop.channel();
    thread::spawn(move || {
        readwrite_chan(&sender);
    });

    let _ = event_loop.run(&mut server_handler);
}

fn readwrite_chan(channel: &EventLoopSender<String>) {
    let mut stdin_reader = old_io::stdin();
    let mut read_buf = [0; 4096];
    loop {
        // Have to block here, so we can't immediately terminate if server closes socket.
        match stdin_reader.read(&mut read_buf) {
            Ok(n) => {
                let read = read_buf.slice_to(n);
                let byte_vec: Vec<u8> = read.to_vec();
                let message = String::from_utf8(byte_vec).unwrap();
                channel.send(message).unwrap();
            },
            Err(f) => {
                panic!(f.to_string());
            }
        }
    }
}
