extern crate getopts;
extern crate "rust-nc" as rustnc;

use getopts::{optflag,getopts,OptGroup,usage};
use std::os;

use rustnc::{print_usage};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        optflag("l", "listen", "Listen for incoming connections."),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(program.as_slice(), opts);
            panic!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
}
