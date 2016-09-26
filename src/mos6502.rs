use addressable::{CpuAddressable, Address, Result};

pub struct Mos6502<Memory: CpuAddressable> {
    memory: Memory,
}

impl<Memory: CpuAddressable> Mos6502<Memory> {
    pub fn new(memory: Memory) -> Self {
        Mos6502 {
            memory: memory,
        }
    }
}

impl<Memory: CpuAddressable> CpuAddressable for Mos6502<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        self.memory.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write(address, data)
    }
}
