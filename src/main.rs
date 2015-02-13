extern crate getopts;
extern crate "rust-nc" as rustnc;

use getopts::{Options,Matches};
use std::os;

use rustnc::{print_usage,nc};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "listen", "Listen for incoming connections.");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(args.tail()) {
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

    nc(&matches);
}
