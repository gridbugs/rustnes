#![allow(dead_code)]
extern crate getopts;

use std::env;
use std::fs;

use getopts::Options;

mod ines;
use ines::{Ines};

fn make_arg_parser() -> Options {
    Options::new()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let parser = make_arg_parser();

    let matches = match parser.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let filename = if matches.free.is_empty() {
        let brief = format!("Usage: {} FILE", program);
        println!("{}", parser.usage(&brief));
        return;
    } else {
        matches.free[0].clone()
    };

    let file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!(e.to_string()),
    };

    let ines = match Ines::parse_file(file) {
        Ok(i) => i,
        Err(e) => panic!("{:?}", e),
    };

    println!("{:?}", ines);
}
