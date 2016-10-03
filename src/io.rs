use addressable::{Addressable, Address, Result};

const PULSE_START: Address = 0x00;
const PULSE_END: Address = 0x07;
const TRIANGLE_START: Address = 0x08;
const TRIANGLE_END: Address = 0x0b;
const NOISE_START: Address = 0x0c;
const NOISE_END: Address = 0x0f;
const DMC_START: Address = 0x10;
const DMC_END: Address = 0x13;
pub const STATUS: Address = 0x15;
pub const FRAME_COUNTER: Address = 0x17;

struct IoRegisters {
    joy1: u8,
    joy2: u8,
}

impl IoRegisters {
    fn new() -> Self {
        IoRegisters {
            joy1: 0,
            joy2: 0,
        }
    }
}

pub struct Io {
    registers: IoRegisters,
}

impl Io {
    pub fn new() -> Self {
        Io {
            registers: IoRegisters::new(),
        }
    }
}

impl Addressable for Io {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            0x16 => Ok(self.registers.joy1),
            0x17 => Ok(self.registers.joy2),
            _ => Ok(0),
        }
    }

    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            0x16 => self.registers.joy1 = data,
            0x17 => self.registers.joy2 = data,
            _ => {},
        }

        Ok(())
    }
}
