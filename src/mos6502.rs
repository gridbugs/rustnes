use addressable::{Addressable, Address, Result};

pub struct Mos6502<Memory: Addressable> {
    memory: Memory,
}

impl<Memory: Addressable> Mos6502<Memory> {
    pub fn new(memory: Memory) -> Self {
        Mos6502 {
            memory: memory,
        }
    }
}

impl<Memory: Addressable> Addressable for Mos6502<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        self.memory.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write(address, data)
    }
}
