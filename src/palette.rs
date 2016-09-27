use addressable::{PpuAddressable, Address, Result, Error};

pub struct Palette {}

impl Palette {
    pub fn new() -> Self {
        Palette {}
    }
}

impl PpuAddressable for Palette {
    fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        Err(Error::UnimplementedRead(address))
    }

    fn ppu_write(&mut self, address: Address, _: u8) -> Result<()> {
        Err(Error::UnimplementedWrite(address))
    }
}
