use std::result;

use addressable;
use addressable::{Addressable, PpuAddressable, Address};

pub type Result<T> = result::Result<T, Error>;

// Mapper Numbers
pub const NROM: usize = 0;

#[derive(Debug)]
pub enum Error {
    InvalidRomSize,
    InvalidChrRomSize,
    IncorrectMapper,
    UnknownMapper(usize),
    InvalidNametableMirroring,
}

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;
pub const CHR_ROM_BANK_SIZE: usize = 0x2000;

// CPU address space offsets within cartridge
pub const CARTRIDGE_START: Address = 0x6000;
pub const RAM_START: Address = 0x6000 - CARTRIDGE_START;
pub const RAM_END: Address = 0x7fff - CARTRIDGE_START;
pub const LOWER_ROM_START: Address = 0x8000 - CARTRIDGE_START;
pub const LOWER_ROM_END: Address = 0xbfff - CARTRIDGE_START;
pub const UPPER_ROM_START: Address = 0xc000 - CARTRIDGE_START;
pub const UPPER_ROM_END: Address = 0xffff - CARTRIDGE_START;

// PPU address space offsets
pub const PATTERN_TABLE_START: Address = 0x0000;
pub const PATTERN_TABLE_END: Address = 0x1fff;
pub const NAME_TABLE_START: Address = 0x2000;
pub const NAME_TABLE_END: Address = 0x2fff;

pub trait CpuInterface {
    fn ram_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn ram_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
    fn lower_rom_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn lower_rom_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
    fn upper_rom_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn upper_rom_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
}

pub trait PpuInterface {
    fn pattern_table_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn pattern_table_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
    fn name_table_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn name_table_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
}

impl<C: CpuInterface> Addressable for C {
    fn read8(&mut self, address: Address) -> addressable::Result<u8> {
        match address {
            RAM_START...RAM_END => self.ram_read(address - RAM_START),
            LOWER_ROM_START...LOWER_ROM_END => self.lower_rom_read(address - LOWER_ROM_START),
            UPPER_ROM_START...UPPER_ROM_END => self.upper_rom_read(address - UPPER_ROM_START),
            _ => Err(addressable::Error::BusErrorRead(address)),
        }
    }

    fn write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        match address {
            RAM_START...RAM_END => self.ram_write(address - RAM_START, data),
            LOWER_ROM_START...LOWER_ROM_END => {
                self.lower_rom_write(address - LOWER_ROM_START, data)
            }
            UPPER_ROM_START...UPPER_ROM_END => {
                self.upper_rom_write(address - UPPER_ROM_START, data)
            }
            _ => Err(addressable::Error::BusErrorWrite(address)),
        }
    }
}

impl<P: PpuInterface> PpuAddressable for P {
    fn ppu_read8(&mut self, address: Address) -> addressable::Result<u8> {
        match address {
            PATTERN_TABLE_START...PATTERN_TABLE_END => {
                self.pattern_table_read(address - PATTERN_TABLE_START)
            }
            NAME_TABLE_START...NAME_TABLE_END => self.name_table_read(address - NAME_TABLE_START),
            _ => Err(addressable::Error::BusErrorRead(address)),
        }
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        match address {
            PATTERN_TABLE_START...PATTERN_TABLE_END => {
                self.pattern_table_write(address - PATTERN_TABLE_START, data)
            }
            NAME_TABLE_START...NAME_TABLE_END => {
                self.name_table_write(address - NAME_TABLE_START, data)
            }
            _ => Err(addressable::Error::BusErrorWrite(address)),
        }
    }
}
