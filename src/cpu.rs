use std::fmt;

use addressable::{CpuAddressable, PpuAddressable, Address, Result};

pub struct RegisterFile {
    accumulator: u8,
    x_index: u8,
    y_index: u8,
    stack_pointer: u8,
    program_counter: u16,
    status: u8,
}

impl RegisterFile {
    fn new() -> Self {
        RegisterFile {
            accumulator: 0,
            x_index: 0,
            y_index: 0,
            stack_pointer: 0,
            program_counter: 0,
            status: 0,
        }
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Registers"));
        try!(writeln!(f, "---------"));
        try!(writeln!(f, "PC:     0x{:02x}", self.program_counter));
        try!(writeln!(f, "SP:     0x{:02x}", self.stack_pointer));
        try!(writeln!(f, "Status: 0x{:02x}", self.status));
        try!(writeln!(f, "ACC:    0x{:02x}", self.accumulator));
        try!(writeln!(f, "X:      0x{:02x}", self.x_index));
        try!(writeln!(f, "Y:      0x{:02x}", self.y_index));
        Ok(())
    }
}

pub struct Cpu<Memory: CpuAddressable + PpuAddressable> {
    pub memory: Memory,
    pub registers: RegisterFile,
}

const RESET_VECTOR: Address = 0xfffc;

impl<Memory: CpuAddressable + PpuAddressable> Cpu<Memory> {
    pub fn new(memory: Memory) -> Self {
        Cpu {
            memory: memory,
            registers: RegisterFile::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.registers.program_counter = try!(self.memory.read16_le(RESET_VECTOR));
        Ok(())
    }
}

impl<Memory: CpuAddressable + PpuAddressable> CpuAddressable for Cpu<Memory> {
    fn read8(&mut self, address: Address) -> Result<u8> {
        self.memory.read8(address)
    }

    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.write8(address, data)
    }
}

impl<Memory: CpuAddressable + PpuAddressable> PpuAddressable for Cpu<Memory> {
    fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        self.memory.ppu_read8(address)
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()> {
        self.memory.ppu_write8(address, data)
    }
}
