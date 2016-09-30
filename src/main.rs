#![allow(dead_code)]
extern crate getopts;

use getopts::Options;

use std::env;
use std::fs;

#[macro_use]
mod macros;

mod nes;
mod image;
mod ines;
mod cpu_memory_layout;
mod cpu_memory_layout_change;
mod ram;
mod vram;
mod mirror;
mod addressable;
mod cartridge;
mod nrom_cartridge;
mod cpu;
mod ppu;
mod ppu_memory_layout;
mod debug;
mod instruction;

use debug::NesDebug;

fn make_arg_parser() -> Options {
    let mut opts = Options::new();

    opts.optflag("d", "dump", "Print the contents of ROM");
    opts.optflag("h", "help", "Print help menu");

    opts
}

fn print_usage(program: &str, parser: Options) {
    let brief = format!("Usage: {} FILE", program);
    println!("{}", parser.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let parser = make_arg_parser();

    let matches = match parser.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(&program, parser);
            return;
        }
    };

    let filename = if matches.free.is_empty() {
        print_usage(&program, parser);
        return;
    } else {
        matches.free[0].clone()
    };

    let file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    let image = match ines::parse_file(file) {
        Ok(i) => i,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut nes = match nes::make_nes(&image) {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    if matches.opt_present("d") {
        println!("{}", (&mut nes).dump_rom());
        return;
    }

    nes.init().expect("initialization failed");

    if let Err(_) = nes.emulate_loop() {
        println!("{}", (&mut nes).dump_memory(0..0x7ff));
    }
}
