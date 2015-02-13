use getopts::{Options,Matches};
use std::net::{TcpListener,TcpStream};
use std::thread::Thread;
use std::io::{Read,BufStream,BufRead};

pub fn listen(matches: &Matches) {
    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                Thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    // drop(listener);
}

fn handle_client(stream: TcpStream) {
    let mut buf_stream = BufStream::new(stream);
    let mut read_buf = [0; 4096];
    loop {
        match(buf_stream.read(&mut read_buf)) {
            Ok(n) => {
                for c in read_buf[..n].iter() {
                    print!("{}", *c as char);
                }
            }
            Err(f) => {
                panic!("done".to_string());
            }
        }
    }
}
