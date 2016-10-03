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

pub const BUTTON_A: u8 = bit!(0);
pub const BUTTON_B: u8 = bit!(1);
pub const BUTTON_SELECT: u8 = bit!(2);
pub const BUTTON_START: u8 = bit!(3);
pub const BUTTON_UP: u8 = bit!(4);
pub const BUTTON_DOWN: u8 = bit!(5);
pub const BUTTON_LEFT: u8 = bit!(6);
pub const BUTTON_RIGHT: u8 = bit!(7);

struct IoRegisters {
    joy1: u8,
}

impl IoRegisters {
    fn new() -> Self {
        IoRegisters {
            joy1: 0,
        }
    }
}

pub struct Io {
    registers: IoRegisters,
    joy1: u8,
}

impl Io {
    pub fn new() -> Self {
        Io {
            registers: IoRegisters::new(),
            joy1: 0,
        }
    }

    pub fn joy1_press(&mut self, button: u8) {
        self.joy1 |= button;
    }
}

impl Addressable for Io {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            0x16 => {
                let data = self.registers.joy1 & bit!(0);
                self.registers.joy1 >>= 1;
                Ok(data)
            }
            _ => Ok(0),
        }
    }

    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            0x16 => {
                if data == 0 {
                    self.registers.joy1 = self.joy1;
                    self.joy1 = 0;
                }
            }
            _ => {},
        }

        Ok(())
    }
}
