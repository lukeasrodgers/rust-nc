extern crate getopts;
use getopts::{optflag,getopts,OptGroup,usage};
use std::os;

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}
