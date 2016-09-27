use addressable::{CpuAddressable, Address, Result, Error};

pub struct NesExpansionRom {}

impl NesExpansionRom {
    pub fn new() -> Self {
        NesExpansionRom {}
    }
}

impl CpuAddressable for NesExpansionRom {
    fn read8(&mut self, address: Address) -> Result<u8> {
        Err(Error::UnimplementedRead(address))
    }

    fn write(&mut self, address: Address, _: u8) -> Result<()> {
        Err(Error::UnimplementedWrite(address))
    }
}
