use std::fmt;
use std::cell::RefCell;

use nes;
use addressable::{Address, AddressDiff};

pub trait NesDebug {
    fn dump_rom<'a>(&'a mut self) -> NesRomDump<'a>;
}

pub struct NesRomDump<'a> {
    nes: RefCell<&'a mut Box<nes::Nes>>,
}

impl<'a> NesDebug for &'a mut Box<nes::Nes> {
    fn dump_rom(&mut self) -> NesRomDump {
        NesRomDump { nes: RefCell::new(self) }
    }
}


const WIDTH: AddressDiff = 32;

const PRG_ROM_START: Address = 0x8000;
const PRG_ROM_SIZE: AddressDiff = 0x8000;

const PATTERN_TABLE_START: Address = 0x0000;
const PATTERN_TABLE_SIZE: AddressDiff = 0x2000;

impl<'a> fmt::Display for NesRomDump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nes = self.nes.borrow_mut();

        try!(writeln!(f, "\nCPU"));
        try!(writeln!(f, "==========================================="));
        for i in 0..PRG_ROM_SIZE {
            let addr = PRG_ROM_START + i;
            if let Ok(data) = nes.read8(addr) {
                if i == 0 {
                    try!(write!(f, "\nPRG ROM Bank 0:"));
                    try!(write!(f, "\n-------------------------------------------"));
                } else if i == PRG_ROM_SIZE / 2 {
                    try!(write!(f, "\n\nPRG ROM Bank 1:"));
                    try!(write!(f, "\n-------------------------------------------"));
                }

                if i % WIDTH == 0 {
                    try!(write!(f, "\n0x{:04x}: ", addr));
                } else {
                    try!(write!(f, " "));
                }
                try!(write!(f, "{:02x}", data));
            } else {
                break;
            }
        }


        try!(writeln!(f, "\nPPU"));
        try!(writeln!(f, "=========================================="));
        for i in 0..PATTERN_TABLE_SIZE {
            let addr = PATTERN_TABLE_START + i;
            if let Ok(data) = nes.ppu_read8(addr) {
                if i == 0 {
                    try!(write!(f, "\nPattern Table 0:"));
                    try!(write!(f, "\n-------------------------------------------"));
                } else if i == PATTERN_TABLE_SIZE / 2 {
                    try!(write!(f, "\n\nPattern Table 1:"));
                    try!(write!(f, "\n-------------------------------------------"));
                }

                if i % WIDTH == 0 {
                    try!(write!(f, "\n0x{:04x}: ", addr));
                } else {
                    try!(write!(f, " "));
                }
                try!(write!(f, "{:02x}", data));
            } else {
                break;
            }
        }

        Ok(())
    }
}
