use addressable::{Address, Result};

const PALETTE_SIZE: usize = 0x20;

pub struct Palette {
    ram: Vec<u8>,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            ram: vec![0; PALETTE_SIZE],
        }
    }
    pub fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        Ok(self.ram[address as usize])
    }
    pub fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()> {
        self.ram[address as usize] = data;
        Ok(())
    }
}
