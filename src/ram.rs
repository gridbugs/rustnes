use addressable::{Address, Addressable, Result, Error};

pub const NES_RAM_NUM_BYTES: usize = 0x800;

pub struct NesRam {
    ram: Vec<u8>,
}

impl NesRam {
    pub fn new() -> Self {
        NesRam {
            ram: vec![0; NES_RAM_NUM_BYTES],
        }
    }
}

impl Addressable for NesRam {
    fn read(&mut self, address: Address) -> Result<u8> {
        let index = address as usize;
        if index < NES_RAM_NUM_BYTES {
            Ok(self.ram[index])
        } else {
            Err(Error::BusErrorRead(address))
        }
    }

    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        let index = address as usize;
        if index < NES_RAM_NUM_BYTES {
            self.ram[index] = data;
            Ok(())
        } else {
            Err(Error::BusErrorWrite(address))
        }
    }
}
