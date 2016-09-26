use addressable::{Address, PpuAddressable, Result};

pub const NES_VRAM_NUM_BYTES: usize = 0x800;

pub struct NesVram {
    ram: Vec<u8>,
}

impl NesVram {
    pub fn new() -> Self {
        NesVram {
            ram: vec![0; NES_VRAM_NUM_BYTES],
        }
    }
}

impl PpuAddressable for NesVram {
    fn read(&mut self, address: Address) -> Result<u8> {
        Ok(self.ram[address as usize])
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.ram[address as usize] = data;
        Ok(())
    }
}
