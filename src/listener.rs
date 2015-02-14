use getopts::{Options,Matches};
use std::net::{TcpListener,TcpStream};
use std::thread::Thread;
use std::io::{Read,BufStream,BufRead};
use std::old_io::{LineBufferedWriter,stdio};

pub fn listen(matches: &Matches) {
    // append port to localhost IP address
    let mut s = String::from_str("127.0.0.1:");
    s.push_str(matches.free[0].as_slice());
    let listener = TcpListener::bind(s.as_slice()).unwrap();
    // block until we get connection
    match listener.accept() {
        Ok(res) => {
            let (stream, socket_addr) = res;
            // nc doesn't handle multiple connections, so no need to do `handle_client`
            // in thread.
            handle_client(stream);
        },
        Err(f) => {
            panic!(f.to_string());
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut buf_stream = BufStream::new(stream);
    let mut read_buf = [0; 4096];
    let mut writer = LineBufferedWriter::new(stdio::stdout());
    loop {
        match(buf_stream.read(&mut read_buf)) {
            Ok(n) => {
                if n > 0 {
                    writer.write_all(&read_buf);
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
