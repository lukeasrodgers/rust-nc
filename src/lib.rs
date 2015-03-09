extern crate getopts;
use getopts::{Options,Matches};
use std::os;

mod listener;
mod connecter;

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(brief.as_slice()));
}

pub fn nc(matches: &Matches) {
    if matches.opt_present("l") {
        listener::nc_listen(matches);
    }
    else {
        connect(matches);
    }
}

fn connect(matches: &Matches) {
    if matches.free.len() >= 2 {
        connecter::nc_connect(matches);
    }
}
