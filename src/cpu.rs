use std::fmt;
use std::result;

use addressable;
use instruction;

use instruction::{Instruction, MemoryAddressingMode, AddressingMode};
use addressable::{Addressable, Address};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InstructionError(instruction::Error),
    MemoryError(addressable::Error),
    UnimplementedInstruction(Instruction),
    UnimplementedMemoryAddressingMode(MemoryAddressingMode),
    UnimplementedAddressingMode(AddressingMode),
}

#[derive(Clone, Copy)]
pub struct StatusRegister {
    negative: bool,
    overflow: bool,
    brk_command: bool,
    decimal_mode: bool,
    irq_disable: bool,
    zero: bool,
    carry: bool,
}

impl StatusRegister {
    fn new() -> Self {
        StatusRegister {
            negative: false,
            overflow: false,
            brk_command: false,
            decimal_mode: false,
            irq_disable: false,
            zero: false,
            carry: false,
        }
    }

    fn compare(&mut self, register: u8, memory: u8) {
        if register == memory {
            self.negative = false;
            self.zero = true;
            self.carry = true;
        } else if register < memory {
            self.negative = true;
            self.zero = false;
            self.carry = false;
        } else {
            self.negative = false;
            self.zero = false;
            self.carry = true;
        }
    }
}

impl fmt::Display for StatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            try!(write!(f, "N "));
        } else {
            try!(write!(f, "- "));
        }
        if self.overflow {
            try!(write!(f, "V "));
        } else {
            try!(write!(f, "- "));
        }
        try!(write!(f, "_ "));
        if self.brk_command {
            try!(write!(f, "B "));
        } else {
            try!(write!(f, "- "));
        }
        if self.decimal_mode {
            try!(write!(f, "D "));
        } else {
            try!(write!(f, "- "));
        }
        if self.irq_disable {
            try!(write!(f, "I "));
        } else {
            try!(write!(f, "- "));
        }
        if self.zero {
            try!(write!(f, "Z "));
        } else {
            try!(write!(f, "- "));
        }
        if self.carry {
            try!(write!(f, "C "));
        } else {
            try!(write!(f, "- "));
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct RegisterFile {
    accumulator: u8,
    x_index: u8,
    y_index: u8,
    stack_pointer: u8,
    program_counter: u16,
    status: StatusRegister,
}

impl RegisterFile {
    fn new() -> Self {
        RegisterFile {
            accumulator: 0,
            x_index: 0,
            y_index: 0,
            stack_pointer: 0,
            program_counter: 0,
            status: StatusRegister::new(),
        }
    }

    fn set_arithmetic_flags(&mut self, data: u8) {
        self.status.negative = data & 0x80 != 0;
        self.status.zero = data == 0;
    }

    fn set_arithmetic_flags_accumulator(&mut self) {
        let accumulator = self.accumulator;
        self.set_arithmetic_flags(accumulator);
    }

    fn set_arithmetic_flags_x_index(&mut self) {
        let x_index = self.x_index;
        self.set_arithmetic_flags(x_index);
    }

    fn set_arithmetic_flags_y_index(&mut self) {
        let y_index = self.y_index;
        self.set_arithmetic_flags(y_index);
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Registers"));
        try!(writeln!(f, "---------"));
        try!(writeln!(f, "PC:     0x{:02x}", self.program_counter));
        try!(writeln!(f, "SP:     0x{:02x}", self.stack_pointer));
        try!(writeln!(f, "ACC:    0x{:02x}", self.accumulator));
        try!(writeln!(f, "X:      0x{:02x}", self.x_index));
        try!(writeln!(f, "Y:      0x{:02x}", self.y_index));
        try!(writeln!(f, "Status: {}", self.status));
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Cpu {
    pub registers: RegisterFile,
}

const RESET_VECTOR: Address = 0xfffc;
const STACK_PAGE_BOTTOM: Address = 0x0100;

impl Cpu {
    pub fn new() -> Self {
        Cpu { registers: RegisterFile::new() }
    }

    pub fn init<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        self.registers.program_counter = try!(memory.read16_le(RESET_VECTOR)
            .map_err(Error::MemoryError));

        Ok(())
    }

    pub fn tick<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        let opcode = try!(self.fetch8(memory));

        let instruction = try!(Self::decode_instruction(opcode));

        try!(self.emulate_instruction(instruction, memory));

        Ok(())
    }

    fn decode_instruction(opcode: u8) -> Result<Instruction> {
        match Instruction::decode(opcode) {
            Ok(i) => Ok(i),
            Err(e) => Err(Error::InstructionError(e)),
        }
    }

    fn fetch8<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<u8> {
        let pc = self.registers.program_counter;
        let opcode = try!(memory.read8(pc).map_err(Error::MemoryError));

        self.registers.program_counter = pc.wrapping_add(1);

        Ok(opcode)
    }

    fn fetch16_le<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<u16> {
        let pc = self.registers.program_counter;
        let opcode = try!(memory.read16_le(pc).map_err(Error::MemoryError));

        self.registers.program_counter = pc.wrapping_add(2);

        Ok(opcode)
    }

    fn addressing_mode_load<Memory: Addressable>(&mut self,
                                                 mode: MemoryAddressingMode,
                                                 memory: &mut Memory)
                                                 -> Result<u8> {
        match mode {
            MemoryAddressingMode::Immediate => self.fetch8(memory),
            MemoryAddressingMode::ZeroPage => {
                let address = try!(self.fetch8(memory)) as u16;
                memory.read8(address).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::Absolute => {
                let address = try!(self.fetch16_le(memory));
                memory.read8(address).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::AbsoluteXIndexed => {
                let address = try!(self.fetch16_le(memory)).wrapping_add(self.registers.x_index as u16);
                memory.read8(address).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::AbsoluteYIndexed => {
                let address = try!(self.fetch16_le(memory)).wrapping_add(self.registers.y_index as u16);
                memory.read8(address).map_err(Error::MemoryError)
            }
            _ => Err(Error::UnimplementedMemoryAddressingMode(mode)),
        }
    }

    fn addressing_mode_store<Memory: Addressable>(&mut self,
                                                  mode: MemoryAddressingMode,
                                                  data: u8,
                                                  memory: &mut Memory)
                                                  -> Result<()> {
        match mode {
            MemoryAddressingMode::ZeroPage => {
                let address = try!(self.fetch8(memory)) as u16;
                memory.write8(address, data).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::Absolute => {
                let address = try!(self.fetch16_le(memory));
                memory.write8(address, data).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::AbsoluteXIndexed => {
                let address = try!(self.fetch16_le(memory)).wrapping_add(self.registers.x_index as u16);
                memory.write8(address, data).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::AbsoluteYIndexed => {
                let address = try!(self.fetch16_le(memory)).wrapping_add(self.registers.y_index as u16);
                memory.write8(address, data).map_err(Error::MemoryError)
            }
            MemoryAddressingMode::IndirectYIndexed => {
                let address_ptr = try!(self.fetch8(memory)) as u16;
                let address = try!(memory.read16_le(address_ptr).map_err(Error::MemoryError))
                    .wrapping_add(self.registers.y_index as u16);
                memory.write8(address, data).map_err(Error::MemoryError)
            }
            _ => Err(Error::UnimplementedMemoryAddressingMode(mode)),
        }
    }

    fn addressing_mode_address<Memory: Addressable>(&mut self,
                                                    mode: MemoryAddressingMode,
                                                    memory: &mut Memory)
                                                    -> Result<Address> {
        match mode {
            MemoryAddressingMode::ZeroPage => Ok(try!(self.fetch8(memory)) as u16),
            _ => unimplemented!(),
        }
    }

    fn relative_branch(&mut self, offset: u8) {
        // Casts allow negative signed 8-bit value to be correctly
        // added to unsigned 16-bit program counter.
        // u8 to i8 makes the offset signed.
        // i8 to i16 sign extends to 16 bits.
        // i16 to u16 turns sign extended value into unsigned.
        let offset = ((offset as i8) as i16) as u16;

        let pc = self.registers.program_counter;
        self.registers.program_counter = pc.wrapping_add(offset);
    }

    fn emulate_instruction<Memory: Addressable>(&mut self,
                                                instruction: Instruction,
                                                memory: &mut Memory)
                                                -> Result<()> {
        match instruction {
            Instruction::SEI => {
                self.set_disable_interrupt_status();
            }
            Instruction::CLI => {
                self.clear_disable_interrupt_status();
            }
            Instruction::SED => {
                self.set_decimal_mode();
            }
            Instruction::CLD => {
                self.clear_decimal_mode();
            }
            Instruction::LDA(mode) => {
                self.registers.accumulator = try!(self.addressing_mode_load(mode, memory));
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::STA(mode) => {
                let accumulator = self.registers.accumulator;
                try!(self.addressing_mode_store(mode, accumulator, memory));
            }
            Instruction::LDX(mode) => {
                self.registers.x_index = try!(self.addressing_mode_load(mode, memory));
                self.registers.set_arithmetic_flags_x_index();
            }
            Instruction::STX(mode) => {
                let x_index = self.registers.x_index;
                try!(self.addressing_mode_store(mode, x_index, memory));
            }
            Instruction::LDY(mode) => {
                self.registers.y_index = try!(self.addressing_mode_load(mode, memory));
                self.registers.set_arithmetic_flags_y_index();
            }
            Instruction::STY(mode) => {
                let y_index = self.registers.y_index;
                try!(self.addressing_mode_store(mode, y_index, memory));
            }
            Instruction::TXS => {
                self.registers.stack_pointer = self.registers.x_index;
            }
            Instruction::BPL => {
                let offset = try!(self.fetch8(memory));
                if !self.registers.status.negative {
                    self.relative_branch(offset);
                }
            }
            Instruction::BEQ => {
                let offset = try!(self.fetch8(memory));
                if self.registers.status.zero {
                    self.relative_branch(offset);
                }
            }
            Instruction::BNE => {
                let offset = try!(self.fetch8(memory));
                if !self.registers.status.zero {
                    self.relative_branch(offset);
                }
            }
            Instruction::BCS => {
                let offset = try!(self.fetch8(memory));
                if self.registers.status.carry {
                    self.relative_branch(offset);
                }
            }
            Instruction::AND(mode) => {
                let accumulator = self.registers.accumulator;
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.accumulator = accumulator & operand;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::CMP(mode) => {
                let accumulator = self.registers.accumulator;
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.status.compare(accumulator, operand);
            }
            Instruction::CPX(mode) => {
                let x_index = self.registers.x_index;
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.status.compare(x_index, operand);
            }
            Instruction::CPY(mode) => {
                let y_index = self.registers.y_index;
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.status.compare(y_index, operand);
            }
            Instruction::DEX => {
                self.registers.x_index = self.registers.x_index.wrapping_sub(1);
                self.registers.set_arithmetic_flags_x_index();
            }
            Instruction::DEY => {
                self.registers.y_index = self.registers.y_index.wrapping_sub(1);
                self.registers.set_arithmetic_flags_y_index();
            }
            Instruction::DEC(mode) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let value = try!(memory.read8(address).map_err(Error::MemoryError)).wrapping_sub(1);
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::JSR => {
                let subroutine_address = try!(self.fetch16_le(memory));
                let return_address = self.registers.program_counter.wrapping_sub(1);
                try!(self.push16_le(return_address, memory));
                self.registers.program_counter = subroutine_address;
            }
            Instruction::RTS => {
                let address = try!(self.pull16_le(memory)).wrapping_add(1);
                self.registers.program_counter = address;
            }
            _ => return Err(Error::UnimplementedInstruction(instruction)),
        }

        Ok(())
    }

    fn set_disable_interrupt_status(&mut self) {
        self.registers.status.irq_disable = true;
    }
    fn clear_disable_interrupt_status(&mut self) {
        self.registers.status.irq_disable = false;
    }
    fn set_decimal_mode(&mut self) {
        self.registers.status.decimal_mode = true;
    }
    fn clear_decimal_mode(&mut self) {
        self.registers.status.decimal_mode = false;
    }
    fn push8<Memory: Addressable>(&mut self, data: u8, memory: &mut Memory) -> Result<()> {
        try!(memory.write8(STACK_PAGE_BOTTOM | self.registers.stack_pointer as u16, data).map_err(Error::MemoryError));
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
        Ok(())
    }
    fn pull8<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<u8> {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        memory.read8(STACK_PAGE_BOTTOM | self.registers.stack_pointer as u16).map_err(Error::MemoryError)
    }
    fn push16_le<Memory: Addressable>(&mut self, data: u16, memory: &mut Memory) -> Result<()> {
        let lo = data as u8;
        let hi = (data >> 8) as u8;
        try!(self.push8(hi, memory));
        self.push8(lo, memory)
    }
    fn pull16_le<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<u16> {
        let lo = try!(self.pull8(memory)) as u16;
        let hi = try!(self.pull8(memory)) as u16;
        Ok((hi << 8) | lo)
    }
}
