use addressable::{Address, Result};

pub struct Palette {}
impl Palette {
    pub fn new() -> Self {
        Palette {}
    }
    pub fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        println!("pallete read {:04x}", address);
        Ok(0)
    }
    pub fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()> {
        println!("pallete write {:04x}: {:02x}", address, data);
        Ok(())
    }
}
