use addressable::{PpuAddressable, Address, Result};

pub struct Ppu<Memory: PpuAddressable> {
    memory: Memory,
}

impl<Memory: PpuAddressable> Ppu<Memory> {
    pub fn new(memory: Memory) -> Self {
        Ppu {
            memory: memory,
        }
    }
}

impl<Memory: PpuAddressable> PpuAddressable for Ppu<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        self.memory.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write(address, data)
    }
}
