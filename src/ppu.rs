use addressable::{CpuAddressable, PpuAddressable, Address, Result};

pub struct Ppu<Memory: PpuAddressable> {
    memory: Memory,
}

impl<Memory: PpuAddressable> Ppu<Memory> {
    pub fn new(memory: Memory) -> Self {
        Ppu { memory: memory }
    }
}

impl<Memory: PpuAddressable> PpuAddressable for Ppu<Memory> {
    fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        self.memory.ppu_read8(address)
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.ppu_write8(address, data)
    }
}

impl<Memory: PpuAddressable> CpuAddressable for Ppu<Memory> {
    fn read8(&mut self, _: Address) -> Result<u8> {
        println!("WARNING: Ignoring read from PPU register!");
        Ok(0) // TODO
    }

    fn write8(&mut self, _: Address, _: u8) -> Result<()> {
        println!("WARNING: Ignoring write to PPU register!");
        Ok(()) // TODO
    }
}
