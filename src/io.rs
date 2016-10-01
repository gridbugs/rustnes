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

pub struct Io {}

impl Io {
    pub fn new() -> Self {
        Io {}
    }
}

impl Addressable for Io {
    fn read8(&mut self, _: Address) -> Result<u8> {
        Ok(0)
    }

    fn write8(&mut self, _: Address, _: u8) -> Result<()> {
        Ok(())
    }
}
