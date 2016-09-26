use addressable::{CpuAddressable, PpuAddressable, Address, Result, Error};

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
    fn ppu_read(&mut self, address: Address) -> Result<u8> {
        self.memory.ppu_read(address)
    }

    fn ppu_write(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.ppu_write(address, data)
    }
}

impl<Memory: PpuAddressable> CpuAddressable for Ppu<Memory> {
    fn read(&mut self, address: Address) -> Result<u8> {
        Err(Error::UnimplementedRead(address))
    }

    fn write(&mut self, address: Address, _: u8) -> Result<()> {
        Err(Error::UnimplementedWrite(address))
    }
}
