use addressable::{Addressable, Address, Result};

pub struct PpuRegisterFile {}

impl PpuRegisterFile {
    fn new() -> Self {
        PpuRegisterFile {}
    }
}

pub struct Ppu {
    pub registers: PpuRegisterFile,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { registers: PpuRegisterFile::new() }
    }
}

impl Addressable for PpuRegisterFile {
    fn read8(&mut self, _: Address) -> Result<u8> {
        println!("WARNING: Ignoring read from PPU register!");
        Ok(0) // TODO
    }

    fn read8_pure(&mut self, _: Address) -> Result<u8> {
        Ok(0)
    }

    fn read8_side_effects(&mut self, _: Address) -> Result<()> {
        println!("WARNING: Ignoring read from PPU register!");
        Ok(())
    }

    fn write8(&mut self, _: Address, _: u8) -> Result<()> {
        println!("WARNING: Ignoring write to PPU register!");
        Ok(()) // TODO
    }
}
