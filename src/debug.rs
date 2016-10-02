use std::fmt;
use std::cell::RefCell;
use std::ops::Range;

use nes;
use addressable::{Address, AddressDiff};

pub trait NesDebug {
    fn dump_rom<'a>(&'a mut self) -> NesRomDump<'a>;
    fn dump_memory<'a>(&'a mut self, range: Range<Address>) -> NesMemoryDump<'a>;
    fn ppu_dump_memory<'a>(&'a mut self, range: Range<Address>) -> NesPpuMemoryDump<'a>;
}

pub struct NesRomDump<'a> {
    nes: RefCell<&'a mut Box<nes::Nes>>,
}

pub struct NesMemoryDump<'a> {
    nes: RefCell<&'a mut Box<nes::Nes>>,
    range: Range<Address>,
}
pub struct NesPpuMemoryDump<'a> {
    nes: RefCell<&'a mut Box<nes::Nes>>,
    range: Range<Address>,
}

impl<'a> NesDebug for &'a mut Box<nes::Nes> {
    fn dump_rom(&mut self) -> NesRomDump {
        NesRomDump { nes: RefCell::new(self) }
    }
    fn dump_memory(&mut self, range: Range<Address>) -> NesMemoryDump {
        NesMemoryDump {
            nes: RefCell::new(self),
            range: range,
        }
    }
    fn ppu_dump_memory(&mut self, range: Range<Address>) -> NesPpuMemoryDump {
        NesPpuMemoryDump {
            nes: RefCell::new(self),
            range: range,
        }
    }
}


const WIDTH: AddressDiff = 32;

const PRG_ROM_START: Address = 0x8000;
const PRG_ROM_SIZE: AddressDiff = 0x8000;
const MEMORY_START: AddressDiff = 0x000;
const MEMORY_END: AddressDiff = 0xffff;

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

impl<'a> fmt::Display for NesMemoryDump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nes = self.nes.borrow_mut();

        let mut address = self.range.start;
        loop {
            if address % WIDTH == 0 {
                try!(write!(f, "\n0x{:04x}: ", address));
            } else {
                try!(write!(f, " "));
            }

            if let Ok(data) = nes.read8(address) {
                if data == 0 {
                    try!(write!(f, ".."));
                } else {
                    try!(write!(f, "{:02x}", data));
                }
            } else {
                try!(write!(f, "??"));
            }

            if address == self.range.end {
                break;
            } else {
                address += 1;
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for NesPpuMemoryDump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nes = self.nes.borrow_mut();

        let mut address = self.range.start;
        loop {
            if address % WIDTH == 0 {
                try!(write!(f, "\n0x{:04x}: ", address));
            } else {
                try!(write!(f, " "));
            }

            if let Ok(data) = nes.ppu_read8(address) {
                if data == 0 {
                    try!(write!(f, ".."));
                } else {
                    try!(write!(f, "{:02x}", data));
                }
            } else {
                try!(write!(f, "??"));
            }

            if address == self.range.end {
                break;
            } else {
                address += 1;
            }
        }
        Ok(())
    }
}
