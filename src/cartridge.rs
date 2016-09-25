use std::result;

use addressable;
use addressable::{Addressable, Address};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRomSize,
    IncorrectMapper,
    UnknownMapper(usize),
}

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;

pub const RAM_START: Address = 0x6000;
pub const RAM_END: Address = 0x7fff;
pub const LOWER_ROM_START: Address = 0x8000;
pub const LOWER_ROM_END: Address = 0xbfff;
pub const UPPER_ROM_START: Address = 0xc000;
pub const UPPER_ROM_END: Address = 0xffff;

pub const NROM: usize = 0;

pub trait AddressableCartridge {
    fn ram_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn ram_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
    fn lower_rom_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn lower_rom_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
    fn upper_rom_read(&mut self, address: Address) -> addressable::Result<u8>;
    fn upper_rom_write(&mut self, address: Address, data: u8) -> addressable::Result<()>;
}

impl<C: AddressableCartridge> Addressable for C {
    fn read(&mut self, address: Address) -> addressable::Result<u8> {
        match address {
            RAM_START ... RAM_END => self.ram_read(address - RAM_START),
            LOWER_ROM_START ... LOWER_ROM_END => self.lower_rom_read(address - LOWER_ROM_START),
            UPPER_ROM_START ... UPPER_ROM_END => self.upper_rom_read(address - UPPER_ROM_START),
            _ => Err(addressable::Error::BusErrorRead(address)),
        }
    }

    fn write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        match address {
            RAM_START ... RAM_END => self.ram_write(address - RAM_START, data),
            LOWER_ROM_START ... LOWER_ROM_END => self.lower_rom_write(address - LOWER_ROM_START, data),
            UPPER_ROM_START ... UPPER_ROM_END => self.upper_rom_write(address - UPPER_ROM_START, data),
            _ => Err(addressable::Error::BusErrorWrite(address)),
        }
    }
}
