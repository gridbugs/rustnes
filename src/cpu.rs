use addressable::{CpuAddressable, PpuAddressable, Address, Result};

pub struct Cpu<Memory: CpuAddressable + PpuAddressable> {
    memory: Memory,
}

impl<Memory: CpuAddressable + PpuAddressable> Cpu<Memory> {
    pub fn new(memory: Memory) -> Self {
        Cpu { memory: memory }
    }
}

impl<Memory: CpuAddressable + PpuAddressable> CpuAddressable for Cpu<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        self.memory.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write(address, data)
    }
}

impl<Memory: CpuAddressable + PpuAddressable> PpuAddressable for Cpu<Memory> {
    fn ppu_read(&mut self, address: Address) -> Result<u8> {
        self.memory.ppu_read(address)
    }

    fn ppu_write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.ppu_write(address, data)
    }
}
