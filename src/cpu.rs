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

const STATUS_CARRY: u8 = bit!(0);
const STATUS_ZERO: u8 = bit!(1);
const STATUS_IRQ_DISABLE: u8 = bit!(2);
const STATUS_DECIMAL: u8 = bit!(3);
const STATUS_BRK: u8 = bit!(4);
const STATUS_UNUSED: u8 = bit!(5);
const STATUS_OVERFLOW: u8 = bit!(6);
const STATUS_NEGATIVE: u8 = bit!(7);

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

    fn to_byte(&self) -> u8 {
        let mut byte = STATUS_UNUSED;
        if self.negative {
            byte |= STATUS_NEGATIVE;
        }
        if self.overflow {
            byte |= STATUS_OVERFLOW;
        }
        if self.brk_command {
            byte |= STATUS_BRK;
        }
        if self.decimal_mode {
            byte |= STATUS_DECIMAL;
        }
        if self.irq_disable {
            byte |= STATUS_IRQ_DISABLE;
        }
        if self.zero {
            byte |= STATUS_ZERO;
        }
        if self.carry {
            byte |= STATUS_CARRY;
        }
        byte
    }

    fn from_byte(byte: u8) -> Self {
        let mut status = Self::new();
        if byte & STATUS_CARRY != 0 {
            status.carry = true;
        }
        if byte & STATUS_ZERO != 0 {
            status.zero = true;
        }
        if byte & STATUS_IRQ_DISABLE != 0 {
            status.irq_disable = true;
        }
        if byte & STATUS_DECIMAL != 0 {
            status.decimal_mode = true;
        }
        if byte & STATUS_BRK != 0 {
            status.brk_command = true;
        }
        if byte & STATUS_OVERFLOW != 0 {
            status.overflow = true;
        }
        if byte & STATUS_NEGATIVE != 0 {
            status.negative = true;
        }
        status
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
        try!(write!(f, "\tPC:     0x{:02x}", self.program_counter));
        try!(write!(f, "\tSP:     0x{:02x}", self.stack_pointer));
        try!(write!(f, "\tACC:    0x{:02x}", self.accumulator));
        try!(write!(f, "\tX:      0x{:02x}", self.x_index));
        try!(write!(f, "\tY:      0x{:02x}", self.y_index));
        try!(write!(f, "\tStatus: {}", self.status));
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct InterruptState {
    pub nmi: bool,
}

impl InterruptState {
    fn new() -> Self {
        InterruptState {
            nmi: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cpu {
    pub registers: RegisterFile,
    pub interrupts: InterruptState,
    pub count: u64,
}

const RESET_VECTOR: Address = 0xfffc;
const NMI_VECTOR: Address = 0xfffa;
const STACK_PAGE_BOTTOM: Address = 0x0100;

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: RegisterFile::new(),
            interrupts: InterruptState::new(),
            count: 0,
        }
    }

    pub fn init<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        self.registers.program_counter = try!(memory.read16_le(RESET_VECTOR)
            .map_err(Error::MemoryError));

        Ok(())
    }

    pub fn tick<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        if self.interrupts.nmi {
            try!(self.nmi(memory));
        }

        let pc = self.registers.program_counter;
        let opcode = try!(self.fetch8(memory));

        let instruction = try!(Self::decode_instruction(opcode));

        try!(self.emulate_instruction(instruction, memory));

        self.count += 1;

        Ok(())
    }

    fn interrupt_common<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        let pc = self.registers.program_counter;
        let status = self.registers.status.to_byte();
        try!(self.push16_le(pc, memory));
        try!(self.push8(status, memory));
        Ok(())
    }

    fn nmi<Memory: Addressable>(&mut self, memory: &mut Memory) -> Result<()> {
        self.interrupts.nmi = false;

        try!(self.interrupt_common(memory));

        self.registers.program_counter = try!(memory.read16_le(NMI_VECTOR)
                                              .map_err(Error::MemoryError));

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
            _ => {
                let address = try!(self.addressing_mode_address(mode, memory));
                memory.read8(address).map_err(Error::MemoryError)
            }
        }
    }

    fn addressing_mode_store<Memory: Addressable>(&mut self,
                                                  mode: MemoryAddressingMode,
                                                  data: u8,
                                                  memory: &mut Memory)
                                                  -> Result<()> {
        let address = try!(self.addressing_mode_address(mode, memory));
        memory.write8(address, data).map_err(Error::MemoryError)
    }

    fn addressing_mode_address<Memory: Addressable>(&mut self,
                                                    mode: MemoryAddressingMode,
                                                    memory: &mut Memory)
                                                    -> Result<Address> {
        let address = match mode {
            MemoryAddressingMode::ZeroPage => try!(self.fetch8(memory)) as u16,
            MemoryAddressingMode::Absolute => try!(self.fetch16_le(memory)),
            MemoryAddressingMode::AbsoluteXIndexed => try!(self.fetch16_le(memory)).wrapping_add(self.registers.x_index as u16),
            MemoryAddressingMode::AbsoluteYIndexed => try!(self.fetch16_le(memory)).wrapping_add(self.registers.y_index as u16),
            MemoryAddressingMode::IndirectYIndexed => {
                let address_ptr = try!(self.fetch8(memory)) as u16;
                try!(memory.read16_le(address_ptr).map_err(Error::MemoryError))
                    .wrapping_add(self.registers.y_index as u16)
            }
            MemoryAddressingMode::ZeroPageXIndexed => try!(self.fetch8(memory)).wrapping_add(self.registers.x_index) as u16,
            MemoryAddressingMode::ZeroPageYIndexed => try!(self.fetch8(memory)).wrapping_add(self.registers.y_index) as u16,
            _ => return Err(Error::UnimplementedMemoryAddressingMode(mode)),
        };

        Ok(address)
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
            Instruction::CLC => {
                self.registers.status.carry = false;
            }
            Instruction::SEC => {
                self.registers.status.carry = true;
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
            Instruction::PHA => {
                let accumulator = self.registers.accumulator;
                try!(self.push8(accumulator, memory));
            }
            Instruction::PHP => {
                let status = self.registers.status.to_byte();
                try!(self.push8(status, memory));
            }
            Instruction::PLP => {
                let status = try!(self.pull8(memory));
                self.registers.status = StatusRegister::from_byte(status);
            }
            Instruction::PLA => {
                self.registers.accumulator = try!(self.pull8(memory));
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::TXA => {
                self.registers.accumulator = self.registers.x_index;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::TYA => {
                self.registers.accumulator = self.registers.y_index;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::TAX => {
                self.registers.x_index = self.registers.accumulator;
                self.registers.set_arithmetic_flags_x_index();
            }
            Instruction::TAY => {
                self.registers.y_index = self.registers.accumulator;
                self.registers.set_arithmetic_flags_y_index();
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
            Instruction::BMI => {
                let offset = try!(self.fetch8(memory));
                if self.registers.status.negative {
                    self.relative_branch(offset);
                }
            }
            Instruction::BCC => {
                let offset = try!(self.fetch8(memory));
                if !self.registers.status.carry {
                    self.relative_branch(offset);
                }
            }
            Instruction::AND(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.accumulator &= operand;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::ORA(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.accumulator |= operand;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::EOR(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                self.registers.accumulator ^= operand;
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
            Instruction::INX => {
                self.registers.x_index = self.registers.x_index.wrapping_add(1);
                self.registers.set_arithmetic_flags_x_index();
            }
            Instruction::DEY => {
                self.registers.y_index = self.registers.y_index.wrapping_sub(1);
                self.registers.set_arithmetic_flags_y_index();
            }
            Instruction::INY => {
                self.registers.y_index = self.registers.y_index.wrapping_add(1);
                self.registers.set_arithmetic_flags_y_index();
            }
            Instruction::DEC(mode) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let value = try!(memory.read8(address).map_err(Error::MemoryError)).wrapping_sub(1);
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::INC(mode) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let value = try!(memory.read8(address).map_err(Error::MemoryError)).wrapping_add(1);
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
            Instruction::RTI => {
                let status = try!(self.pull8(memory));
                self.registers.status = StatusRegister::from_byte(status);
                let program_counter = try!(self.pull16_le(memory));
                self.registers.program_counter = program_counter;
            }
            Instruction::BIT(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                let and = self.registers.accumulator & operand;
                self.registers.status.zero = and == 0;
                self.registers.status.negative = operand & bit!(7) != 0;
                self.registers.status.overflow = operand & bit!(6) != 0;
            }
            Instruction::JMP => {
                self.registers.program_counter = try!(self.fetch16_le(memory));
            }
            Instruction::JMPI => {
                let mut address_ptr_lo = try!(self.fetch8(memory));
                let address_ptr_hi = try!(self.fetch8(memory)) as u16;

                let address_lo = try!(memory.read8((address_ptr_hi << 8) | (address_ptr_lo as u16))
                                      .map_err(Error::MemoryError)) as u16;
                address_ptr_lo = address_ptr_lo.wrapping_add(1);
                let address_hi = try!(memory.read8((address_ptr_hi << 8) | (address_ptr_lo as u16))
                                      .map_err(Error::MemoryError)) as u16;

                self.registers.program_counter = (address_hi << 8) | address_lo;
            }
            Instruction::LSR(AddressingMode::Accumulator) => {
                self.registers.status.carry = self.registers.accumulator & bit!(0) != 0;
                self.registers.accumulator >>= 1;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::LSR(AddressingMode::Memory(mode)) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let mut value = try!(memory.read8(address).map_err(Error::MemoryError));
                self.registers.status.carry = value & bit!(0) != 0;
                value >>= 1;
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::ROR(AddressingMode::Memory(mode)) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let mut value = try!(memory.read8(address).map_err(Error::MemoryError));
                let carry = self.registers.status.carry;
                self.registers.status.carry = value & bit!(0) != 0;
                value >>= 1;
                if carry {
                    value |= bit!(7);
                }
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::ROL(AddressingMode::Memory(mode)) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let mut value = try!(memory.read8(address).map_err(Error::MemoryError));
                let carry = self.registers.status.carry;
                self.registers.status.carry = value & bit!(7) != 0;
                value <<= 1;
                if carry {
                    value |= bit!(0);
                }
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::ROL(AddressingMode::Accumulator) => {
                let carry = self.registers.status.carry;
                let mut value = self.registers.accumulator;
                self.registers.status.carry = value & bit!(7) != 0;
                value <<= 1;
                if carry {
                    value |= bit!(0);
                }
                self.registers.accumulator = value;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::ROR(AddressingMode::Accumulator) => {
                let carry = self.registers.status.carry;
                let mut value = self.registers.accumulator;
                self.registers.status.carry = value & bit!(0) != 0;
                value >>= 1;
                if carry {
                    value |= bit!(7);
                }
                self.registers.accumulator = value;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::ASL(AddressingMode::Memory(mode)) => {
                let address = try!(self.addressing_mode_address(mode, memory));
                let mut value = try!(memory.read8(address).map_err(Error::MemoryError));
                self.registers.status.carry = value & bit!(7) != 0;
                value <<= 1;
                try!(memory.write8(address, value).map_err(Error::MemoryError));
                self.registers.set_arithmetic_flags(value);
            }
            Instruction::ASL(AddressingMode::Accumulator) => {
                self.registers.status.carry = self.registers.accumulator & bit!(7) != 0;
                self.registers.accumulator <<= 1;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::ADC(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                let (partial_sum, carry_a) = self.registers.accumulator.overflowing_add(operand);
                let (sum, carry_b) = partial_sum.overflowing_add(if self.registers.status.carry { 1 } else { 0 });

                let carry = carry_a || carry_b;

                self.registers.status.overflow = (!(operand ^ self.registers.accumulator)) & (operand ^ sum) & bit!(7) != 0;
                self.registers.accumulator = sum;
                self.registers.status.carry = carry;
                self.registers.set_arithmetic_flags_accumulator();
            }
            Instruction::SBC(mode) => {
                let operand = try!(self.addressing_mode_load(mode, memory));
                let (partial_dif, borrow_a) = self.registers.accumulator.overflowing_sub(operand);
                let (dif, borrow_b) = partial_dif.overflowing_sub(if self.registers.status.carry { 0 } else { 1 });

                let borrow = borrow_a || borrow_b;

                self.registers.status.overflow = (self.registers.accumulator ^ operand) &
                                                 (self.registers.accumulator ^ dif) & bit!(7) != 0;

                self.registers.accumulator = dif;
                self.registers.status.carry = !borrow;
                self.registers.set_arithmetic_flags_accumulator();
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
