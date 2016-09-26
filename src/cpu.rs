use addressable::{CpuAddressable, Address, Result};

pub struct Cpu<Memory: CpuAddressable> {
    memory: Memory,
}

impl<Memory: CpuAddressable> Cpu<Memory> {
    pub fn new(memory: Memory) -> Self {
        Cpu {
            memory: memory,
        }
    }
}

impl<Memory: CpuAddressable> CpuAddressable for Cpu<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        self.memory.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write(address, data)
    }
}
