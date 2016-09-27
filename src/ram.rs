use addressable::{Address, CpuAddressable, Result};

pub const NES_RAM_NUM_BYTES: usize = 0x800;

pub struct NesRam {
    ram: Vec<u8>,
}

impl NesRam {
    pub fn new() -> Self {
        NesRam { ram: vec![0; NES_RAM_NUM_BYTES] }
    }
}

impl CpuAddressable for NesRam {
    fn read8(&mut self, address: Address) -> Result<u8> {
        Ok(self.ram[address as usize])
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        self.ram[address as usize] = data;
        Ok(())
    }
}
